#!/usr/bin/env bash
set -euo pipefail

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly INSTALL_DIR="${HOME}/.local/bin"

main() {
    echo "Installing tsp command..."
    
    # Create install directory if it doesn't exist
    mkdir -p "${INSTALL_DIR}"
    
    # Copy tsp command
    cp "${SCRIPT_DIR}/tsp" "${INSTALL_DIR}/tsp"
    chmod +x "${INSTALL_DIR}/tsp"
    
    echo "✓ tsp command installed to ${INSTALL_DIR}/tsp"
    
    # Check if install directory is in PATH
    if [[ ":${PATH}:" != *":${INSTALL_DIR}:"* ]]; then
        echo ""
        echo "⚠️  ${INSTALL_DIR} is not in your PATH"
        echo "Add the following line to your shell profile (~/.bashrc, ~/.zshrc, etc.):"
        echo ""
        echo "    export PATH=\"\${HOME}/.local/bin:\${PATH}\""
        echo ""
        echo "Then restart your shell or run: source ~/.zshrc"
    else
        echo "✓ ${INSTALL_DIR} is already in your PATH"
        echo ""
        echo "You can now use: tsp <pane_count> <directory>"
    fi
}

main "$@"