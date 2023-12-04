#!/bin/bash

# Check if a test name argument is provided
if [ $# -eq 0 ]; then
  echo "Please provide a test name."
  exit 1
fi

# Capture the test name argument
test_name="$1"

# Run the specified test with `cargo test`
cargo test "$test_name" -- --nocapture
