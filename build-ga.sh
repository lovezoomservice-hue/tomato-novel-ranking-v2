#!/usr/bin/env bash
# ============================================================
# Build script for Tomato Novel Downloader (Ranking Edition)
# Usage:
#   Linux/macOS:  chmod +x build-ga.sh && ./build-ga.sh
#   Windows Git Bash: bash build-ga.sh
# ============================================================
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "=== Tomato Novel Downloader - Building ==="
echo "Platform: $(uname -s)"
echo ""

# Check Rust
if ! command -v cargo &>/dev/null; then
    echo "Error: Rust/Cargo not found. Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal
    source "$HOME/.cargo/env"
fi

cargo --version
rustc --version
echo ""

# Switch to no-official-api mode
echo "Switching to no-official-api mode..."
cp Cargo_no_official.toml Cargo.toml
echo "Done."
echo ""

# Determine features based on platform
PLATFORM="$(uname -s)"
if [[ "$PLATFORM" == "Linux" ]]; then
    FEATURES="tts"
    OUTPUT_NAME="TomatoNovelDownloader-Linux_amd64"
elif [[ "$PLATFORM" == "Darwin" ]]; then
    FEATURES="tts,clipboard,clipboard-arboard"
    OUTPUT_NAME="TomatoNovelDownloader-macOS_arm64"
else
    # Windows Git Bash / MSYS
    FEATURES="tts,clipboard,clipboard-arboard"
    OUTPUT_NAME="TomatoNovelDownloader-Win64.exe"
fi

echo "Building with features: $FEATURES"
echo "Output name: $OUTPUT_NAME"
echo ""

# Build
echo "Compiling (this may take 10-30 minutes on first build)..."
cargo build --release --features "$FEATURES"
echo ""
echo "Build complete!"

# Copy output
OUTPUT_DIR="$SCRIPT_DIR/dist"
mkdir -p "$OUTPUT_DIR"

if [[ "$PLATFORM" == "MINGW"* ]] || [[ "$PLATFORM" == "MSYS"* ]] || [[ "$PLATFORM" == "CYGWIN"* ]]; then
    cp target/release/tomato-novel-downloader.exe "$OUTPUT_DIR/$OUTPUT_NAME"
    echo "Output: $OUTPUT_DIR/$OUTPUT_NAME"
else
    cp target/release/tomato-novel-downloader "$OUTPUT_DIR/$OUTPUT_NAME"
    echo "Output: $OUTPUT_DIR/$OUTPUT_NAME"
fi

echo ""
echo "=== Build successful! ==="
echo ""
echo "To run the Web UI:"
echo "  $OUTPUT_DIR/$OUTPUT_NAME --server"
echo ""
echo "To run with password protection:"
echo "  $OUTPUT_DIR/$OUTPUT_NAME --server --password your_password"
echo ""
echo "Then open in browser: http://127.0.0.1:18423/"
