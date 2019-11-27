set -euxo pipefail

main() {
    cargo check --target $T

    if [ $TRAVIS_RUST_VERSION = nightly ]; then
        cargo check --features spanned --target $T
    fi
}

# fake Travis variables to be able to run this on a local machine
if [ -z ${TRAVIS_RUST_VERSION-} ]; then
    case $(rustc -V) in
        *nightly*)
            TRAVIS_RUST_VERSION=nightly
            ;;
        *beta*)
            TRAVIS_RUST_VERSION=beta
            ;;
        *)
            TRAVIS_RUST_VERSION=stable
            ;;
    esac
fi

if [ -z ${T-} ]; then
    T=$(rustc -Vv | grep host | cut -d ' ' -f2)
fi

main
