#!/usr/bin/env python3
"""
Generate Rust struct definitions from an OpenAPI JSON spec.

Usage:
    python3 generate-rust-types.py --url https://api.example.com --output src/api/v1_0_0.rs
    python3 generate-rust-types.py --url https://api.example.com --output src/api/v1_0_0.rs --version 1.0.0
"""

import argparse
import json
import os
import re
import sys
import urllib.request
import urllib.error

# ── Rust reserved keywords (need r# prefix) ─────────────────────
RUST_KEYWORDS = {
    "as", "async", "await", "break", "const", "continue", "crate", "dyn",
    "else", "enum", "extern", "false", "fn", "for", "if", "impl", "in",
    "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "Self", "static", "struct", "super", "trait", "true", "type",
    "unsafe", "use", "where", "while", "yield",
}


def to_snake_case(name):
    """Convert a camelCase or PascalCase string to snake_case."""
    # Insert underscore before uppercase letters that follow lowercase/digits
    s = re.sub(r'([a-z0-9])([A-Z])', r'\1_\2', name)
    # Insert underscore between consecutive uppercase letters followed by lowercase
    s = re.sub(r'([A-Z]+)([A-Z][a-z])', r'\1_\2', s)
    return s.lower()


def rust_field_name(json_name):
    """
    Convert a JSON property name to a valid Rust field name.
    Returns (rust_name, needs_rename) where needs_rename is True if
    a #[serde(rename = "...")] annotation is needed.
    """
    snake = to_snake_case(json_name)
    # Replace any non-alphanumeric chars with underscore
    snake = re.sub(r'[^a-z0-9_]', '_', snake)
    # Strip leading/trailing underscores
    snake = snake.strip('_')
    # Collapse double underscores
    while '__' in snake:
        snake = snake.replace('__', '_')
    # If the name starts with a digit, prefix with underscore
    if snake and snake[0].isdigit():
        snake = '_' + snake

    needs_rename = (snake != json_name)

    # Handle Rust keywords — always emit rename for raw identifiers
    if snake in RUST_KEYWORDS:
        return 'r#' + snake, True

    return snake, needs_rename


def sanitize_struct_name(name):
    """
    Sanitize a schema name into a valid Rust struct identifier.
    Hyphens are removed and the following character is uppercased (PascalCase).
    E.g. 'ActivityResponse-Input' -> 'ActivityResponseInput'
    """
    # Split on hyphens, capitalize each part, rejoin
    parts = name.split('-')
    return ''.join(part[0].upper() + part[1:] if part else '' for part in parts)


def resolve_ref(ref_str):
    """Extract the schema name from a $ref string like '#/components/schemas/Foo'."""
    parts = ref_str.split('/')
    return sanitize_struct_name(parts[-1])


def resolve_type(prop, schemas, required=True):
    """
    Resolve an OpenAPI property schema to a Rust type string.
    Returns the Rust type as a string.
    """
    if '$ref' in prop:
        return resolve_ref(prop['$ref'])

    # anyOf handling (nullable pattern and union types)
    if 'anyOf' in prop:
        variants = prop['anyOf']
        non_null = [v for v in variants if v.get('type') != 'null']
        has_null = len(non_null) < len(variants)

        if len(non_null) == 1:
            inner = resolve_type(non_null[0], schemas)
            if has_null:
                return f'Option<{inner}>'
            return inner
        elif len(non_null) == 0:
            return 'serde_json::Value'
        else:
            # Multiple non-null types — can't express as a simple Rust type
            return 'serde_json::Value'

    # oneOf — treat similarly to anyOf
    if 'oneOf' in prop:
        variants = prop['oneOf']
        non_null = [v for v in variants if v.get('type') != 'null']
        has_null = len(non_null) < len(variants)

        if len(non_null) == 1:
            inner = resolve_type(non_null[0], schemas)
            if has_null:
                return f'Option<{inner}>'
            return inner
        else:
            return 'serde_json::Value'

    # allOf — if single ref, use that; otherwise fall through
    if 'allOf' in prop:
        refs = [p for p in prop['allOf'] if '$ref' in p]
        if len(refs) == 1 and len(prop['allOf']) == 1:
            return resolve_ref(refs[0]['$ref'])
        # Multiple allOf — merge would be needed, but for field type just use Value
        if len(refs) == 1:
            return resolve_ref(refs[0]['$ref'])
        return 'serde_json::Value'

    prop_type = prop.get('type')

    if prop_type == 'string':
        return 'String'
    elif prop_type == 'integer':
        return 'i64'
    elif prop_type == 'number':
        return 'f64'
    elif prop_type == 'boolean':
        return 'bool'
    elif prop_type == 'array':
        items = prop.get('items', {})
        item_type = resolve_type(items, schemas)
        return f'Vec<{item_type}>'
    elif prop_type == 'object':
        additional = prop.get('additionalProperties')
        if additional is not None:
            if isinstance(additional, dict):
                if additional:
                    val_type = resolve_type(additional, schemas)
                else:
                    # additionalProperties: {} (any value)
                    val_type = 'serde_json::Value'
                return f'std::collections::HashMap<String, {val_type}>'
            else:
                # additionalProperties: true
                return 'std::collections::HashMap<String, serde_json::Value>'
        if prop.get('properties'):
            # Inline object with properties — can't generate a named struct here,
            # fall back to Value
            return 'serde_json::Value'
        return 'serde_json::Value'
    elif prop_type is None:
        # No type specified — generic value
        return 'serde_json::Value'
    else:
        return 'serde_json::Value'


def merge_allof(schema, schemas):
    """
    Merge allOf entries into a single schema with combined properties and required.
    """
    merged_props = {}
    merged_required = set()
    base_title = schema.get('title', '')

    for sub in schema.get('allOf', []):
        if '$ref' in sub:
            ref_name = resolve_ref(sub['$ref'])
            ref_schema = schemas.get(ref_name, {})
            # Recursively merge if the ref itself has allOf
            if 'allOf' in ref_schema:
                ref_schema = merge_allof(ref_schema, schemas)
            for pname, pschema in ref_schema.get('properties', {}).items():
                merged_props[pname] = pschema
            for r in ref_schema.get('required', []):
                merged_required.add(r)
        else:
            for pname, pschema in sub.get('properties', {}).items():
                merged_props[pname] = pschema
            for r in sub.get('required', []):
                merged_required.add(r)

    # Also include any direct properties on the schema itself
    for pname, pschema in schema.get('properties', {}).items():
        merged_props[pname] = pschema
    for r in schema.get('required', []):
        merged_required.add(r)

    return {
        'type': 'object',
        'properties': merged_props,
        'required': list(merged_required),
        'title': base_title,
    }


def generate_struct(name, schema, schemas):
    """
    Generate a Rust struct definition string for a given schema.
    Returns None if the schema should be skipped.
    """
    # Handle allOf by merging
    if 'allOf' in schema:
        schema = merge_allof(schema, schemas)

    properties = schema.get('properties', {})

    # Skip empty schemas or schemas that are just a primitive type with no properties
    if not properties:
        schema_type = schema.get('type')
        if schema_type in ('string', 'integer', 'number', 'boolean', None):
            return None
        if schema_type == 'object' and not schema.get('additionalProperties'):
            return None
        if schema_type == 'array':
            return None
        return None

    required_fields = set(schema.get('required', []))
    lines = []

    safe_name = sanitize_struct_name(name)
    lines.append('#[derive(Debug, Clone, Serialize, Deserialize)]')
    lines.append(f'pub struct {safe_name} {{')

    # Sort fields: required first, then optional — but actually the existing files
    # DON'T sort by required. They appear to use the order from the schema, with
    # required fields first then optional. Looking more carefully, the existing files
    # list required fields first, then optional fields.
    # Let's match: required fields in their original order, then optional in original order.
    prop_items = list(properties.items())
    required_items = [(k, v) for k, v in prop_items if k in required_fields]
    optional_items = [(k, v) for k, v in prop_items if k not in required_fields]
    ordered_items = required_items + optional_items

    for i, (prop_name, prop_schema) in enumerate(ordered_items):
        field_name, needs_rename = rust_field_name(prop_name)
        is_required = prop_name in required_fields
        rust_type = resolve_type(prop_schema, schemas)

        annotations = []

        # Add rename annotation if needed
        if needs_rename:
            annotations.append(f'    #[serde(rename = "{prop_name}")]')

        if not is_required:
            annotations.append('    #[serde(default)]')
            # Wrap in Option if not already Option (from anyOf-with-null)
            if not rust_type.startswith('Option<'):
                rust_type = f'Option<{rust_type}>'

        for ann in annotations:
            lines.append(ann)

        lines.append(f'    pub {field_name}: {rust_type},')

    lines.append('}')
    return '\n'.join(lines)


def fetch_openapi_spec(url):
    """Fetch and parse the OpenAPI JSON spec from a URL using curl (more reliable on macOS)."""
    import subprocess
    spec_url = url.rstrip('/') + '/openapi.json'
    try:
        result = subprocess.run(
            ['curl', '-sf', '--max-time', '10', spec_url],
            capture_output=True, text=True, check=True,
        )
        return json.loads(result.stdout)
    except subprocess.CalledProcessError:
        print(f"Error: Could not fetch {spec_url}", file=sys.stderr)
        sys.exit(1)
    except json.JSONDecodeError:
        print(f"Error: Invalid JSON from {spec_url}", file=sys.stderr)
        sys.exit(1)


def load_openapi_file(path):
    """Load an OpenAPI JSON spec from a local file."""
    try:
        with open(path) as f:
            return json.load(f)
    except FileNotFoundError:
        print(f"Error: File not found: {path}", file=sys.stderr)
        sys.exit(1)
    except json.JSONDecodeError:
        print(f"Error: Invalid JSON in {path}", file=sys.stderr)
        sys.exit(1)


def main():
    parser = argparse.ArgumentParser(
        description='Generate Rust struct definitions from an OpenAPI JSON spec.'
    )
    parser.add_argument(
        '--url',
        help='Server base URL (will fetch <url>/openapi.json via curl)'
    )
    parser.add_argument(
        '--file',
        help='Local path to an openapi.json file (alternative to --url)'
    )
    parser.add_argument(
        '--output', required=True,
        help='Output file path for generated Rust code'
    )
    parser.add_argument(
        '--version',
        help='Override the version from the OpenAPI spec'
    )
    args = parser.parse_args()

    # Fetch or load the spec
    if args.file:
        spec = load_openapi_file(args.file)
    elif args.url:
        spec = fetch_openapi_spec(args.url)
    else:
        parser.error('Either --url or --file is required')

    # Extract version and title
    info = spec.get('info', {})
    title = info.get('title', 'unknown')
    version = args.version or info.get('version', '0.0.0')

    # Get all schemas
    schemas = spec.get('components', {}).get('schemas', {})
    if not schemas:
        print("Error: No schemas found in components.schemas", file=sys.stderr)
        sys.exit(1)

    # Generate structs sorted alphabetically by schema name
    struct_defs = []
    for name in sorted(schemas.keys()):
        schema = schemas[name]
        result = generate_struct(name, schema, schemas)
        if result is not None:
            struct_defs.append(result)

    schema_count = len(struct_defs)

    # Build the output
    output_lines = [
        f'// Auto-generated from {title} OpenAPI spec v{version}',
        '// Do not edit manually \u2014 regenerated on each API release.',
        f'// Schemas: {schema_count}',
        '',
        'use serde::{Deserialize, Serialize};',
        '',
    ]

    header = '\n'.join(output_lines) + '\n'
    body = '\n\n'.join(struct_defs) + '\n'
    output = header + body

    # Write to file
    out_dir = os.path.dirname(os.path.abspath(args.output))
    if out_dir:
        os.makedirs(out_dir, exist_ok=True)
    with open(args.output, 'w') as f:
        f.write(output)

    print(f"Generated {schema_count} structs from {title} v{version}")
    print(f"Written to {args.output}")


if __name__ == '__main__':
    main()
