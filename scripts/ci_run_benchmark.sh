#!/usr/bin/env bash
set -Eexuo pipefail

# Path to run `canbench` from.
CANISTER_PATH=$1

CANBENCH_OUTPUT=/tmp/canbench_output.txt

cargo install --path ./canbench-bin
cd "$CANISTER_PATH"

canbench --less-verbose >> $CANBENCH_OUTPUT
if grep -q "(regressed by \|(improved by" "${CANBENCH_OUTPUT}"; then
  echo "Significant performance change detected! ⚠️
  If the change is expected, run \`canbench --persist\` to save the updated benchmark results." > $CANBENCH_OUTPUT
else
  echo "No significant performance changes detected ✅" > $CANBENCH_OUTPUT
fi

## Add the output of canbench to the file.
echo "" >> $CANBENCH_OUTPUT
echo "\`\`\`" >> $CANBENCH_OUTPUT
canbench --less-verbose >> $CANBENCH_OUTPUT
echo "\`\`\`" >> $CANBENCH_OUTPUT

# Output the file's contents to stdout.
cat $CANBENCH_OUTPUT
