#!/usr/bin/env bash
set -Eexuo pipefail

# Script that, given a comment that was prepared by `ci_run_benchmark.sh`,
# fails or succeeds depending on whether or not there were changes in performance.

# If changed, then other scripts need to be updated as well.
COMMENT_MESSAGE_PATH=/tmp/canbench_comment_message.txt

if grep -q "(regressed by \|(improved by" "$COMMENT_MESSAGE_PATH"; then
  exit 1
fi
