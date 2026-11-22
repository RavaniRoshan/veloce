#!/usr/bin/env bash
# One-shot PR creator. Run from any directory after git push succeeded.
# Requires: gh CLI authenticated (gh auth login) OR set GITHUB_TOKEN.
set -euo pipefail

REPO="${REPO:-RavaniRoshan/veloce}"          # <-- adjust if repo name differs
HEAD_BRANCH="${HEAD_BRANCH:-qwen-streak-nov-2026}"
BASE="${BASE:-main}"
BODY_FILE="${BODY_FILE:-/workspace/.pr/PR_BODY.md}"

if ! command -v gh >/dev/null 2>&1; then
  echo "gh not found. Install: sudo apt update && sudo apt install -y gh"
  echo "Or use the manual fallback printed at the end."
fi

echo "== 1. Push streak branch =="
git push origin "HEAD:refs/heads/${HEAD_BRANCH}"

echo "== 2. Open PR =="
if command -v gh >/dev/null 2>&1; then
  gh pr create \
    --repo "$REPO" \
    --base "$BASE" \
    --head "$HEAD_BRANCH" \
    --title "chore+feat: repo hygiene, Dispatcher lossy API, changelog (streak 13/15/16/19 Nov)" \
    --body-file "$BODY_FILE"
else
  echo
  echo "Manual fallback — open this URL in your browser:"
  echo "https://github.com/${REPO}/compare/${BASE}...${HEAD_BRANCH}?expand=1"
  echo "Title: chore+feat: repo hygiene, Dispatcher lossy API, changelog (streak 13/15/16/19 Nov)"
  echo "Body:  copy contents of ${BODY_FILE}"
fi
