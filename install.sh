#!/bin/sh
# Glow CLI installer — downloads the latest release for your platform.
#
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/learnloopllc/glow-cli/main/install.sh | sh

set -e

REPO="learnloopllc/glow-cli"
INSTALL_DIR="/usr/local/bin"

# Detect platform
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case "$OS" in
  darwin)
    case "$ARCH" in
      arm64|aarch64) TARGET="aarch64-apple-darwin" ;;
      x86_64)        TARGET="x86_64-apple-darwin" ;;
      *) echo "Unsupported architecture: $ARCH" >&2; exit 1 ;;
    esac
    ;;
  linux)
    case "$ARCH" in
      x86_64) TARGET="x86_64-unknown-linux-gnu" ;;
      *) echo "Unsupported architecture: $ARCH" >&2; exit 1 ;;
    esac
    ;;
  *) echo "Unsupported OS: $OS" >&2; exit 1 ;;
esac

ASSET="glow-${TARGET}.tar.gz"

# Get latest version
VERSION=$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name"' | sed 's/.*"tag_name": *"\([^"]*\)".*/\1/')
if [ -z "$VERSION" ]; then
  echo "Failed to determine latest version" >&2
  exit 1
fi

URL="https://github.com/${REPO}/releases/download/${VERSION}/${ASSET}"

echo "Installing glow ${VERSION} (${TARGET})..."

# Download and extract to temp dir
TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT
curl -fsSL "$URL" | tar xz -C "$TMP"

# Install (may need sudo)
if [ -w "$INSTALL_DIR" ]; then
  mv "$TMP/glow" "$TMP/glw" "$INSTALL_DIR/"
else
  echo "Installing to ${INSTALL_DIR} (requires sudo)..."
  sudo mv "$TMP/glow" "$TMP/glw" "$INSTALL_DIR/"
fi

echo "Installed glow ${VERSION} to ${INSTALL_DIR}"
echo "  glow --help    to get started"
echo "  glow --version to verify"
