#!/usr/bin/env bash
set -Eexuo pipefail

# Script that runs `canbench` at a given directory and outputs a comment
# that is intended to be posted on the pull request.

# Path to run `canbench` from.
CANISTER_PATH=$1

CANBENCH_OUTPUT=/tmp/canbench_output.txt

# If changed, then other scripts need to be updated as well.
COMMENT_MESSAGE_PATH=/tmp/canbench_comment_message.txt

cargo install --path ./canbench-bin
cd "$CANISTER_PATH"

canbench --less-verbose >> $CANBENCH_OUTPUT

echo "# \`canbench\` (dir: $CANISTER_PATH)
" > $COMMENT_MESSAGE_PATH

if grep -q "(regressed by \|(improved by" "${CANBENCH_OUTPUT}"; then
  echo "Significant performance change detected! ⚠️
  If the change is expected, run \`canbench --persist\` to save the updated benchmark results." >> $COMMENT_MESSAGE_PATH
else
  echo "No significant performance changes detected ✅" >> $COMMENT_MESSAGE_PATH
fi

## Add the output of canbench to the file.
{
  echo ""
  echo "\`\`\`"
  cat "$CANBENCH_OUTPUT"
  echo "\`\`\`"
} >> $COMMENT_MESSAGE_PATH

# Output the comment to stdout.
cat $COMMENT_MESSAGE_PATH
