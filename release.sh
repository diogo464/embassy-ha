#!/bin/bash
set -euo pipefail

VERSION="${1:-}"

if [[ -z "$VERSION" ]]; then
    echo "Usage: $0 <version>"
    echo "Example: $0 0.3.0"
    exit 1
fi

if [[ ! "$VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo "Error: Version must be in format x.y.z"
    exit 1
fi

echo "Releasing version $VERSION"

# Verify clean working directory
if [[ -n "$(git status --porcelain)" ]]; then
    echo "Error: Working directory has uncommitted changes"
    git status --short
    exit 1
fi

echo "Running pre-release checks..."

echo "  cargo fmt --check"
cargo fmt --check

echo "  cargo clippy"
cargo clippy -- -D warnings

echo "  cargo check --tests --examples"
cargo check --tests --examples

echo "  cargo test"
cargo test

echo "  cargo publish --dry-run"
cargo publish --dry-run

echo "All checks passed. Updating version..."

sed -i "s/^version = \".*\"/version = \"$VERSION\"/" Cargo.toml

echo "Creating commit..."
git add Cargo.toml
git commit -m "chore: release v$VERSION"

echo "Creating tag..."
git tag -a "v$VERSION" -m "v$VERSION"

echo "Pushing to remote..."
git push
git push --tags

echo "Release v$VERSION complete!"
echo "To publish to crates.io, run: cargo publish"
