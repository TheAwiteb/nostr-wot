set quiet
set shell := ["/usr/bin/env", "bash", "-c"]

JUST_EXECUTABLE := "just -u -f " + justfile()
header := "Available tasks:\n"

_default:
    @{{JUST_EXECUTABLE}} --list-heading "{{header}}" --list

# Run the CI
@ci: && msrv
    cargo build -q
    cargo fmt -- --check
    cargo clippy -- -D warnings
    cargo doc --no-deps
    cargo-deny check all
    RUST_LOG=error taplo fmt --check

# Check that the current MSRV is correct
@msrv:
    nix run .#msrv
