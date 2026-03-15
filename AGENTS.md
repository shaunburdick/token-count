# token-count Development Guidelines

Auto-generated from all feature plans. Last updated: 2026-03-14

## Active Technologies

- Rust 1.86.0+ (stable channel) (001-core-cli)

## Project Structure

```text
src/
tests/
```

## Commands

cargo test && cargo clippy

## Code Style

Rust 1.86.0+ (stable channel): Follow standard conventions

## Recent Changes

- 001-core-cli: Added Rust 1.86.0+ (stable channel) (MSRV updated from 1.85 for io::Error::other() compatibility)

<!-- MANUAL ADDITIONS START -->

## Development Workflow

### Feature Implementation Process

1. **Specification Phase** (if not already complete)
   - Receive or create feature specifications in `specs/###-feature-name/`
   - Includes: `spec.md`, `plan.md`, `tasks.md`, `data-model.md`, `contracts/`, etc.

2. **Create Feature Branch**
   ```bash
   git checkout -b 004-feature-name  # Use feature number prefix
   ```

3. **Implementation**
   - Follow tasks outlined in `specs/###-feature-name/tasks.md`
   - Write tests first (TDD approach)
   - Maintain zero clippy warnings: `cargo clippy -- -D warnings`
   - Run full test suite: `cargo test`
   - Update documentation as you go

4. **Pre-Commit Verification** (MANDATORY)
   ```bash
   cargo test                    # All tests must pass
   cargo clippy -- -D warnings   # Zero warnings required
   cargo fmt -- --check          # Formatting must be correct
   cargo build --release         # Must build successfully
   ```

5. **Commit Changes**
   ```bash
   git add -A
   git commit -m "feat: brief description
   
   Detailed explanation of changes.
   
   - Bullet point 1
   - Bullet point 2"
   ```

6. **Create Pull Request**
   ```bash
   # Push branch
   git push -u origin 004-feature-name
   
   # Create PR with comprehensive description
   gh pr create --title "feat: Feature Title" --body "$(cat <<'EOF'
   ## Summary
   Brief overview of changes
   
   ## What's New
   - Feature 1
   - Feature 2
   
   ## Testing
   - All tests passing
   - Manual verification completed
   EOF
   )"
   ```

7. **Monitor CI Checks**
   ```bash
   # Watch PR status
   gh pr view <PR-NUMBER> --json statusCheckRollup
   
   # Wait for all checks to pass:
   # - Build
   # - Lint
   # - Test
   # - Security Audit
   ```

8. **Fix Issues** (if CI fails)
   - Address any failing checks
   - Commit fixes to the same branch
   - CI will automatically re-run

9. **Wait for Human Review & Merge**
   - Do NOT merge PRs yourself
   - Wait for project owner to review and squash/merge
   - Owner will merge when ready

10. **Post-Merge Cleanup**
    ```bash
    # Fetch latest main
    git fetch origin main
    git checkout main
    git pull origin main
    
    # Delete feature branch (local)
    git branch -D 004-feature-name
    
    # Remote branch is auto-deleted by GitHub after merge
    git fetch --prune
    
    # Verify tests still pass on main
    cargo test
    ```

11. **Version Bump & Release**
    ```bash
    # Update version in Cargo.toml
    # For new features: bump minor version (0.2.2 → 0.3.0)
    # For bug fixes: bump patch version (0.2.2 → 0.2.3)
    
    # Build and verify version
    cargo build --release
    cargo run --release -- --version
    
    # Commit version bump
    git add Cargo.toml Cargo.lock
    git commit -m "chore: bump version to X.Y.Z"
    git push origin main
    
    # Create annotated release tag
    git tag -a vX.Y.Z -m "Release vX.Y.Z: Brief description
    
    ## What's New
    - Feature 1
    - Feature 2
    
    ## Technical Details
    - Detail 1
    - Detail 2"
    
    # Push tag to trigger release workflow
    git push origin vX.Y.Z
    ```

12. **Monitor Release Workflow**
    - GitHub Actions will automatically:
      - Build release binaries for all platforms
      - Generate checksums
      - Create GitHub Release
      - Attach binaries to release
    - Check workflow: `gh run list --workflow=release.yml`
    - View release: `gh release view vX.Y.Z`

### Code Quality Standards

**Zero Suppressions Policy**
- ❌ NEVER use `#[allow(dead_code)]`, `#[allow(unused)]`, etc.
- ❌ NEVER use `@ts-ignore`, `eslint-disable`, or similar
- ✅ Fix code to comply with linter/compiler rules
- ✅ If a rule seems wrong, discuss with project owner first

**Testing Requirements**
- Write tests for all new functionality
- Maintain or improve test coverage
- Run full suite before every commit
- Manual testing for user-facing changes

**Documentation Requirements**
- Update README.md with new features/usage
- Update CHANGELOG.md with version entries
- Add doc comments to all public APIs
- Update INSTALL.md if build requirements change

### Git Branch Policy

**Protected Branches**
- `main`, `master`, `develop` - Protected, cannot commit directly
- Always work on feature branches
- Use naming: `###-feature-name` (e.g., `004-gemini-support`)

**Commit Message Format**
Follow Conventional Commits:
- `feat:` - New feature
- `fix:` - Bug fix
- `chore:` - Maintenance (version bumps, dependencies)
- `docs:` - Documentation only
- `test:` - Test changes
- `refactor:` - Code restructuring

### Release Process

**Semantic Versioning**
- `MAJOR.MINOR.PATCH` (e.g., `0.4.0`)
- MAJOR: Breaking changes
- MINOR: New features (backward compatible)
- PATCH: Bug fixes

**Release Workflow**
1. Merge PR to main
2. Update version in ALL of the following files:
   - [ ] `Cargo.toml` - Line 3: `version = "X.Y.Z"`
   - [ ] `README.md` - Bottom status line: `**Status**: ✅ vX.Y.Z Complete (...) | **Version**: X.Y.Z`
   - [ ] `CHANGELOG.md` - Add new `## [X.Y.Z] - YYYY-MM-DD` section at top
   - [ ] `CHANGELOG.md` - Add link at bottom: `[X.Y.Z]: https://github.com/shaunburdick/token-count/releases/tag/vX.Y.Z`
3. Build and verify version: `cargo build --release && ./target/release/token-count --version`
4. Run full test suite: `cargo test`
5. Commit version bump: `git commit -m "chore: bump version to X.Y.Z"`
6. Push to main: `git push origin main`
7. Create annotated tag with release notes:
   ```bash
   git tag -a vX.Y.Z -m "Release vX.Y.Z: Brief description
   
   ## What's New
   - Feature 1
   - Feature 2
   
   ## Technical Details
   - Detail 1
   - Detail 2"
   ```
8. Push tag: `git push origin vX.Y.Z`
9. GitHub Actions will automatically:
   - Build release binaries for all platforms
   - Generate checksums
   - Create GitHub Release
   - Attach binaries to release
10. Monitor workflow: `gh run list --workflow=release.yml`
11. Verify release: `gh release view vX.Y.Z`

**Version Bump Examples**
- New feature: `0.3.0` → `0.4.0` (minor bump)
- Bug fix: `0.3.0` → `0.3.1` (patch bump)
- Breaking change: `0.3.0` → `1.0.0` (major bump)

### Useful Commands

```bash
# Check current branch
git branch --show-current

# List all branches
git branch -a

# View PR status
gh pr view <NUMBER>

# List recent workflow runs
gh run list --limit 10

# Watch workflow in progress
gh run watch

# Test specific functionality
echo "test input" | cargo run --release -- --model <model-name>

# Build and check size
cargo build --release
ls -lh target/release/token-count

# Update agent context after implementation
.specify/scripts/bash/update-agent-context.sh opencode
```

### Project-Specific Notes

**Build Requirements**
- Rust 1.86.0+ (MSRV)
- CMake 3.10+ (for SentencePiece tokenizer, build-time only)

**Binary Size Target**
- Target: < 20MB
- Current: 16.8MB (includes OpenAI, Claude, and Gemini tokenizers)

**Test Performance**
- Full suite should complete in < 30 seconds
- Currently: ~19 seconds for 177 tests

**Dependencies**
- Minimize new dependencies
- Security audit all dependencies
- Document community-maintained dependencies

<!-- MANUAL ADDITIONS END -->
