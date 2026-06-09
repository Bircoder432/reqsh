#!/bin/sh

set -e

REPO="hars-21/reqsh"
BIN_NAME="reqsh"
INSTALL_DIR="$HOME/.local/bin"

echo "Installing $BIN_NAME..."

OS="$(uname -s)"
ARCH="$(uname -m)"

case "$(echo "$OS" | tr '[:upper:]' '[:lower:]')" in
    linux)
        OS="unknown-linux-gnu"
        EXT="tar.gz"
        ;;
    darwin)
        OS="apple-darwin"
        EXT="tar.gz"
        ;;
    mingw*|msys*)
        OS="pc-windows-msvc"
        EXT="zip"
        BIN_NAME="reqsh.exe"
        ;;
    *)
        echo "Unsupported OS: $OS"
        exit 1
        ;;
esac

case "$ARCH" in
    x86_64)
        ARCH="x86_64"
        ;;
    arm64|aarch64)
        ARCH="aarch64"
        ;;
    *)
        echo "Unsupported architecture: $ARCH"
        exit 1
        ;;
esac

TARGET="${ARCH}-${OS}"

VERSION=$(
    curl -fsSLI -o /dev/null -w '%{url_effective}' \
    "https://github.com/${REPO}/releases/latest" \
    | sed 's|.*/||'
)

if [ -z "$VERSION" ]; then
    echo "Failed to fetch latest version"
    exit 1
fi

FILE="${BIN_NAME}-${VERSION}-${TARGET}.${EXT}"

URL="https://github.com/${REPO}/releases/download/${VERSION}/${FILE}"

TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

ARCHIVE_PATH="$TMP_DIR/$FILE"

echo "Downloading $FILE..."
curl -fsSL "$URL" -o "$ARCHIVE_PATH"

echo "Extracting archive..."
if [ "$EXT" = "zip" ]; then
    unzip -o "$ARCHIVE_PATH" -d "$TMP_DIR"
else
    tar -xzf "$ARCHIVE_PATH" -C "$TMP_DIR"
fi

chmod +x "$TMP_DIR/$BIN_NAME" 2>/dev/null || true

mkdir -p "$INSTALL_DIR"

echo "Installing to $INSTALL_DIR..."
mv "$TMP_DIR/$BIN_NAME" "$INSTALL_DIR/$BIN_NAME"

echo
echo "reqsh installed successfully"
echo

"$INSTALL_DIR/$BIN_NAME" --version

case ":$PATH:" in
    *":$INSTALL_DIR:"*)
        ;;
    *)
        echo
        echo "Add this to your shell profile:"
        echo
        echo "export PATH=\"$INSTALL_DIR:\$PATH\""
        ;;
esac