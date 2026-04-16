#!/bin/bash
# Install git hooks for the project

HOOK_DIR="$(git rev-parse --git-dir)/hooks"
mkdir -p "$HOOK_DIR"

cat > "$HOOK_DIR/pre-commit" << 'HOOK'
#!/bin/bash
set -e

echo "Running pre-commit checks..."

# Rust
echo "  cargo fmt --check"
cargo fmt --all -- --check
echo "  cargo clippy"
cargo clippy --workspace -- -D warnings

# Frontend
echo "  prettier --check"
npx prettier --check src/
echo "  eslint"
npx eslint src/

echo "All checks passed!"
HOOK

chmod +x "$HOOK_DIR/pre-commit"
echo "Pre-commit hook installed!"
