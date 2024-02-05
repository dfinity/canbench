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

CANBENCH_RESULTS_FILE="$CANISTER_PATH/canbench_results.yml"

# Install canbench
cargo install --path ./canbench-bin

# Detect if there are performance changes relative to the main branch.
# Github CI is setup such that the main branch is available in the directory.

ls -al $MAIN_BRANCH_DIR
ls -al $MAIN_BRANCH_DIR/"$CANISTER_PATH"

if [ ! -f "$CANBENCH_RESULTS_FILE" ]; then
    echo "$CANBENCH_RESULTS_FILE not found. Did you forget to run \`canbench --persist\`?";
    exit 1
fi

# If the main branch has a results file, compare the PR with the current result.
if [ -f "$MAIN_BRANCH_DIR/$CANBENCH_RESULTS_FILE" ]; then
    mv "$CANBENCH_RESULTS_FILE" "$CANBENCH_RESULTS_FILE}.current"

    cp "$MAIN_BRANCH_DIR/$CANBENCH_RESULTS_FILE" "$CANBENCH_RESULTS_FILE"
fi

cd "$CANISTER_PATH"

canbench --less-verbose --persist >> $CANBENCH_OUTPUT

echo "# \`canbench\` 🏋 (dir: $CANISTER_PATH)" > $COMMENT_MESSAGE_PATH

if grep -q "(regressed by \|(improved by" "${CANBENCH_OUTPUT}"; then
  echo "**Significant performance change detected! ⚠️**" >> $COMMENT_MESSAGE_PATH;
else
  echo "**No significant performance changes detected ✅**" >> $COMMENT_MESSAGE_PATH
fi

if cmp -s "${CANBENCH_RESULTS_FILE}.current" "$CANBENCH_RESULTS_FILE"; then
  echo "**$CANBENCH_RESULTS_FILE is up to date ✅**" >> $COMMENT_MESSAGE_PATH;
else
  echo "**$CANBENCH_RESULTS_FILE is not up to date ❌**
  If the performance change is expected, run \`canbench --persist\` to save the updated benchmark results." >> $COMMENT_MESSAGE_PATH
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
