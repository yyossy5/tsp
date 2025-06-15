#!/usr/bin/env bash
set -euo pipefail

# Usage: tmux-multi-pane.sh <pane_count> <directory>
# Description: Create specified number of panes in current tmux window, all cd'd to specified directory

readonly SCRIPT_NAME="$(basename "${BASH_SOURCE[0]}")"

usage() {
    echo "Usage: ${SCRIPT_NAME} <pane_count> <directory>"
    echo "  pane_count: Number of panes to create (including current pane)"
    echo "  directory:  Directory to cd into for all panes"
    echo ""
    echo "Example:"
    echo "  ${SCRIPT_NAME} 4 /Users/y_yoshida/Projects/tsp"
    exit 1
}

main() {
    local pane_count="${1:?Error: pane_count required}"
    local target_dir="${2:?Error: directory required}"

    # Validate inputs
    if ! [[ "${pane_count}" =~ ^[0-9]+$ ]] || [[ "${pane_count}" -lt 1 ]]; then
        echo "Error: pane_count must be a positive integer" >&2
        usage
    fi

    if [[ ! -d "${target_dir}" ]]; then
        echo "Error: Directory '${target_dir}' does not exist" >&2
        exit 1
    fi

    # Check if we're in a tmux session
    if [[ -z "${TMUX:-}" ]]; then
        echo "Error: Not in a tmux session" >&2
        exit 1
    fi

    # Get absolute path
    target_dir="$(cd "${target_dir}" && pwd)"

    # Move current pane to target directory
    tmux send-keys "cd '${target_dir}'" Enter

    # Create additional panes (pane_count - 1)
    for ((i = 1; i < pane_count; i++)); do
        if ((i % 2 == 1)); then
            # Split horizontally
            tmux split-window -h -c "${target_dir}"
        else
            # Split vertically
            tmux split-window -v -c "${target_dir}"
        fi
    done

    # Arrange panes in a tiled layout
    tmux select-layout tiled

    echo "Created ${pane_count} panes in directory: ${target_dir}"
}

# Show usage if no arguments
if [[ $# -eq 0 ]]; then
    usage
fi

main "$@"