#!/bin/bash

set -e

# Function to build a crate
build_crate() {
    local crate_name=$1
    local release_mode=$2
    echo "Building $crate_name..."
    if [ "$release_mode" = "true" ]; then
        cargo build --package "d_$crate_name" --release
    else
        cargo build --package "d_$crate_name"
    fi
}

# Function to build a crate with debug info
build_debug() {
    local crate_name=$1
    echo "Building $crate_name with debug info..."
    RUSTFLAGS="-g" cargo build --package "d_$crate_name"
}

# Main build script
main() {
    if [ "$#" -eq 0 ]; then
        build_crate "compiler"
        build_crate "cli"
        build_crate "lsp"
    else
        local command=$1
        shift
        case "$command" in
            "build")
                for crate in "$@"; do
                    build_crate "$crate"
                done
                ;;
            "build-debug")
                for crate in "$@"; do
                    build_debug "$crate"
                done
                ;;
            *)
                echo "Invalid command: $command"
                exit 1
                ;;
        esac
    fi
}

main "$@"
