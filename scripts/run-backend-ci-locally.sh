#!/bin/bash
set -e # Exit immediately if a command exits with a non-zero status.

WORKFLOW_FILE=".github/workflows/backend-ci.yml"
JOB_NAME="build_and_test_backend" # Must match the job name in the workflow file

# Determine event type: 'pull_request' or 'push'
# Default to 'pull_request', or pass 'push' as the first argument to the script
EVENT_TYPE=${1:-pull_request}

echo "Running Backend CI locally using act..."
echo "Workflow: $WORKFLOW_FILE"
echo "Job: $JOB_NAME"
echo "Event Type: $EVENT_TYPE"
echo "Using .actrc: $(cat .actrc || echo 'Not found or empty')"
echo "-----------------------------------------------------"

if [ "$EVENT_TYPE" == "pull_request" ]; then
    echo "INFO: Simulating a 'pull_request' event."
    echo "For path filters ('paths:') to work correctly with 'act', ensure you have committed"
    echo "changes in the 'backend/' directory on your current branch that are not on 'main'."
    echo "Example to create dummy changes for testing:"
    echo "  git checkout main && git pull origin main"
    echo "  git checkout -b temp-backend-test"
    echo "  mkdir -p backend/act_test_dummy && touch backend/act_test_dummy/file.txt"
    echo "  git add backend/act_test_dummy/file.txt"
    echo "  git commit -m 'feat: dummy change in backend for act testing'"
    echo "Then run this script: ./scripts/run-backend-ci-locally.sh pull_request"
    echo "After testing, you can clean up: git checkout main && git branch -D temp-backend-test"
    echo "-----------------------------------------------------"
    act pull_request --job "$JOB_NAME" -W "$WORKFLOW_FILE"
else
    echo "INFO: Simulating a 'push' to main event."
    echo "Path filters are not applied for 'push' to main; the workflow should always run."
    echo "-----------------------------------------------------"
    act push --job "$JOB_NAME" -W "$WORKFLOW_FILE"
fi

echo "-----------------------------------------------------"
echo "Backend CI local run finished."
