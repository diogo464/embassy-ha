# Release Command

Create a new cargo release for embassy-ha.

## Instructions

1. **Ask for the new version number** - Prompt the user for the version (e.g., "0.3.0"). Do not include the "v" prefix in the version number itself.

2. **Verify clean working directory** - Run `git status --porcelain` and fail if there are any uncommitted changes. The working directory must be clean before proceeding.

3. **Run pre-release checks** - Run the following commands in order. If any command fails, **stop immediately**, report the error, and let the user fix the issues manually. Do NOT attempt to fix issues automatically:
   - `cargo fmt --check` - Verify code is properly formatted
   - `cargo clippy` - Run linter (must have no warnings)
   - `cargo check --tests --examples` - Check compilation of tests and examples
   - `cargo test` - Run the test suite
   - `cargo publish --dry-run` - Verify the package can be published

4. **Update version in Cargo.toml** - Only after all pre-release checks pass, update the `version` field in `Cargo.toml` to the new version.

5. **Create release commit** - Create a commit with the message: `chore: release v{version}`

6. **Create tag** - Create an annotated tag with the name `v{version}` and message `v{version}`

7. **Push to remote** - Push both the commit and the tag to the remote:
   - `git push`
   - `git push --tags`

## Important Notes

- If any pre-release check fails, **stop immediately** and report the failure. Do NOT:
  - Attempt to fix the issues automatically (e.g., running `cargo fmt` to fix formatting)
  - Update the version in Cargo.toml
  - Create the commit or tag
- The user is responsible for fixing any issues and re-running the release command
- The tag format must be `v{version}` (e.g., `v0.3.0`) to match existing conventions
- Use `--dry-run` for cargo publish to verify without actually publishing
