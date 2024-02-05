#!/usr/bin/env bash
set -Eexuo pipefail

# Script that runs `canbench` at a given directory and outputs a comment
# that is intended to be posted on the pull request.

# Path to run `canbench` from.
CANISTER_PATH=$1

pwd
ls -al

CANBENCH_OUTPUT=/tmp/canbench_output.txt

# If changed, then other scripts need to be updated as well.
COMMENT_MESSAGE_PATH=/tmp/canbench_comment_message.txt

# Github CI is expected to have the main branch checked out in this folder.
MAIN_BRANCH_DIR=_canbench_main_branch

# Install canbench
cargo install --path ./canbench-bin

# Detect if there are performance changes relative to the main branch.
# Github CI is setup such that the main branch is available in the directory.

ls -al $MAIN_BRANCH_DIR
ls -al $MAIN_BRANCH_DIR/"$CANISTER_PATH"

# Is there
if [ -f "$MAIN_BRANCH_DIR/$CANISTER_PATH/canbench_results.yml" ]; then
    echo "canbench results found!";
fi

cd "$CANISTER_PATH"

canbench --less-verbose >> $CANBENCH_OUTPUT

echo "# \`canbench\` 🏋 (dir: $CANISTER_PATH)
" > $COMMENT_MESSAGE_PATH

if grep -q "(regressed by \|(improved by" "${CANBENCH_OUTPUT}"; then
  echo "**Significant performance change detected! ⚠️**
  If the change is expected, run \`canbench --persist\` to save the updated benchmark results." >> $COMMENT_MESSAGE_PATH
else
  echo "**No significant performance changes detected ✅**" >> $COMMENT_MESSAGE_PATH
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
