# Use powershell if running on windows
set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

alias l := lint
alias c := check
alias b := build
alias br := build-release
alias t := test
alias tf := test-full

default: lint check test

# Check code styles
lint:
    cargo clippy

# Check that the code will compile
check:
    cargo check

# Run tests in debug mode, omitting perft tests
test:
    cargo test

# Runs tests in production mode, which includes perft tests
test-full:
    cargo test --release

# Compiles Athena to a debug binary
build:
    cargo build

# Compiles Athena to a release binary
build-release:
    cargo build --release
