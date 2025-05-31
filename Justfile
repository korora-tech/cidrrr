# Default recipe runs check
default: check

# Format all code
format:
    cargo fmt --all

# Run clippy on all code
lint:
    cargo clippy --tests --all-features --all-targets

# Run all tests with nextest
test:
    cargo nextest run --all-features --all-targets

# Run complete check (format, lint, test)
check: format lint test