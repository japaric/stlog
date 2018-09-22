set -euxo pipefail

main() {
    cargo check --all
}

main
