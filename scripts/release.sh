#!/bin/bash
set -e

if [ -z "$1" ]; then
  echo "Usage: ./scripts/release.sh <version>"
  echo "Example: ./scripts/release.sh 0.2.0"
  exit 1
fi

VERSION="$1"
TAG="v$VERSION"
ROOT="$(git rev-parse --show-toplevel)"

# Bump version
"$ROOT/scripts/bump-version.sh" "$VERSION"

# Commit, tag, push
cd "$ROOT"
git add -A
git commit -m "release $TAG"
git tag "$TAG"
git push
git push origin "$TAG"

echo ""
echo "Released $TAG"
