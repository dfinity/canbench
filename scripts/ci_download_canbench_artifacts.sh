#!/usr/bin/env bash
set -Eexuo pipefail

# Identifies the benchmarks provided in the artifacts and outputs them.
json_array="["

# Loop through each file with prefix "canbench" in the current directory
for file in canbench_result_*; do
if [ -e "$file" ]; then  # Check if the file actually exists to avoid including the pattern itself
  # Read the content of the file, escaping double quotes and adding escaped newlines
  content=$(<"$file/$file" sed 's/"/\\"/g' | awk '{printf "%s\\n", $0}' | sed '$ s/\\n$//')

  # Construct a JSON object for the current file with "title" and "result" keys
  json_object="{\"title\":\"$file\",\"result\":\"$content\"},"

  # Append the JSON object to the array string
  json_array+="$json_object"
fi
done

# Remove the trailing comma from the JSON array string
json_array=${json_array%,}

# Close the JSON array string
json_array+="]"

# Use jq to format the JSON array string properly and assign it to an environment variable
# Output the list of benchmarks into the matrix, to be used by the next job.
echo "matrix={\"benchmark\": $json_array}" >> "$GITHUB_OUTPUT"
echo "pr_number=$(cat ./pr_number/pr_number)" >> "$GITHUB_OUTPUT"

