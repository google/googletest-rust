#!/bin/bash
BINARY_PATH="$1"

if [ -z "$BINARY_PATH" ]; then
  echo "Usage: $0 <binary_path>"
  exit 1
fi

# Run the binary and capture output.
set +e
OUTPUT=$("${BINARY_PATH}" 2>&1)
STATUS=$?
set -e

# 101 is the standard Rust panic exit code.
if [[ ${STATUS} -ne 101 ]]; then
  echo "Expected panic (exit code 101), but script exited with ${STATUS}."
  echo "Output:"
  echo "${OUTPUT}"
  exit 1
fi

echo "${OUTPUT}" | \
sed -n -e '/^Google Test trace:/p' -e '/^  .*trace/p' | \
sed 's#^.*async_scoped_trace_panic_test.rs#  async_scoped_trace_panic_test.rs#g'
