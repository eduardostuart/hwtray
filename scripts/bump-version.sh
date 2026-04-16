#!/bin/bash
set -e

if [ -z "$1" ]; then
  echo "Usage: ./scripts/bump-version.sh <version>"
  echo "Example: ./scripts/bump-version.sh 0.2.0"
  exit 1
fi

VERSION="$1"
ROOT="$(git rev-parse --show-toplevel)"
CONF="$ROOT/crates/homewizard-desktop/tauri.conf.json"

sed -i '' "s/\"version\": \".*\"/\"version\": \"$VERSION\"/" "$CONF"

echo "Bumped to $VERSION"
echo "  - $CONF"
