default: fmt lint build test

build:
  cargo build

fmt:
  find . -name "*.nix" | xargs alejandra
  cargo fmt

lint:
  cargo clippy

run:
  cargo run

test:
  cargo test
