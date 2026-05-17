# Glow — CLI

Glow is source-available for academic, research, educational, and other noncommercial use under the [PolyForm Noncommercial License 1.0.0](./LICENSE).

Commercial use requires a separate written license from Purdue Research Foundation / Purdue University. Contact: ashok@learn-loop.org.

This repository contains the **CLI** — the canonical tool for deploying and managing a Glow instance. It is one of four components in the Glow platform:

| Component | What it is |
|---|---|
| [api](https://github.com/glow-academic/api) | FastAPI backend |
| [client](https://github.com/glow-academic/client) | Next.js frontend |
| **cli** (this repo) | Rust CLI — what end users actually install |
| [docs](https://github.com/glow-academic/docs) | Nextra docs site |

## Install

```bash
brew tap glow-academic/tap
brew install glow             # stable
# or:
brew install glow-beta        # pre-release channel
```

For non-Homebrew installs, download a prebuilt binary from [releases](https://github.com/glow-academic/cli/releases) for your platform (`darwin-arm64`, `darwin-x64`, `linux-arm64`, `linux-x64`).

## Quick start

```bash
glow init       # interactive wizard writes ~/.glow/instances/default/glow-deploy.yaml
glow deploy     # pulls api + client images, brings up the stack
glow status     # show container health + active blue/green colors
glow redeploy   # update to a new api/client version with zero-downtime swap
glow stop / start / destroy
glow backup list / create / restore
```

See the [docs](https://glow-academic.github.io/docs/) for the full deployment guide, including topology modes (`airgapped`, `exposed`, `api_only`), TLS setup, and configuration reference.

## Local development

```bash
cargo build
cargo run -- --help
cargo test
```

## License

This project is licensed under the [PolyForm Noncommercial License 1.0.0](./LICENSE).

This is not an OSI-approved open-source license. It is intended to support academic and research dissemination while preserving separate commercial licensing rights.
