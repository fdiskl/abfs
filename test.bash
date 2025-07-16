#!/bin/bash

set -euo pipefail

TEST_DIR="./bin/tests"
EXIT_CODE=0


GREEN='\033[0;32m'
RED='\033[0;31m'
RESET='\033[0m'

for test in "$TEST_DIR"/*; do
  if [[ -x "$test" && -f "$test" ]]; then
    echo "Running: $test"
    "$test"
    STATUS=$?
    if [[ $STATUS -eq 0 ]]; then
      echo -e "${GREEN}Passed: $test${RESET}"
    else
      echo -e "${RED}Failed: $test with status $STATUS${RESET}"
      EXIT_CODE=1
    fi
  fi
done

exit $EXIT_CODE
