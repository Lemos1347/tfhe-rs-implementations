_default:
  @just --list

clean:
  @cargo clean

build CMD:
  @cargo build --release --example {{CMD}}

run CMD *ARGS:
  @./target/release/examples/{{CMD}} {{ARGS}}

# TFHE Fibonacci benchmark recipes
[group('fibonacci')]
build-fibonacci:
  @just build fibonacci

# Run Fibonacci benchmark with default settings (10 numbers, default security)
[group('fibonacci')]
run-fibonacci:
  @just run fibonacci

# Run Fibonacci benchmark with specified number of Fibonacci numbers
[group('fibonacci')]
run-fibonacci-n N="15":
  @just run fibonacci "--num={{N}}"

# Run Fibonacci benchmark in fast (lower security) mode
[group('fibonacci')]
run-fibonacci-fast:
  @just run fibonacci --fast

# Run Fibonacci benchmark with specified number and in fast mode
[group('fibonacci')]
run-fibonacci-n-fast N="15":
  @just run fibonacci "--num={{N}}" --fast

# Show help for Fibonacci benchmark
[group('fibonacci')]
fibonacci-help:
  @just run fibonacci --help
