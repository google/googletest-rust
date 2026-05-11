#!/bin/bash
# fail on error
set -e

# Read binary path from argument
BINARY_PATH="$1"

if [ -z "$BINARY_PATH" ]; then
  echo "Usage: $0 <binary_path>"
  exit 1
fi

# Run the binary, capture output, and process with sed
# We ignore the failure of the binary as per the original genrule (|| true)
( "${BINARY_PATH}" 2>&1 || true ) | \
sed -n -e '/^Google Test trace:/p' -e '/^  .*trace/p' | \
sed 's#^.*scoped_trace_panic_test.rs#  scoped_trace_panic_test.rs#g'
