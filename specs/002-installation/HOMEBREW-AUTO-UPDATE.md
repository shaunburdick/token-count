# Homebrew Auto-Update Strategy

**Feature**: 002-installation  
**Question**: How to automatically update homebrew-tap repo when there's a new release?

## Solution: GitHub Action Automation

We'll use the [`mislav/bump-homebrew-formula-action`](https://github.com/mislav/bump-homebrew-formula-action) GitHub Action to automatically update the Homebrew formula on every release.

### How It Works

1. **You push a git tag** (e.g., `v0.1.0`) to the token-count repository
2. **GitHub Actions release workflow runs**:
   - Builds binaries for all platforms
   - Packages them as tar.gz/zip
   - Generates SHA256 checksums
   - Creates GitHub Release with all assets
3. **Auto-update job runs** (after release completes):
   - Fetches the new tarball from GitHub Release
   - Calculates SHA256 checksum
   - Updates `Formula/token-count.rb` in homebrew-tap repo
   - Commits and pushes the change

**Result**: Homebrew formula is updated automatically within minutes of tagging a release. No manual work required.

### Setup Requirements

#### 1. Create Personal Access Token
Go to: https://github.com/settings/tokens/new

- **Token type**: Personal access token (classic)
- **Note**: "Homebrew Tap Updates for token-count"
- **Scopes**: Select:
  - ✅ `repo` (Full control of private repositories)
  - ✅ `workflow` (Update GitHub Action workflows)
- **Expiration**: No expiration (or 1 year and set reminder to regenerate)
- Click "Generate token"
- **Copy the token** (you won't see it again!)

#### 2. Add Token as Repository Secret
1. Go to: https://github.com/shaunburdick/token-count/settings/secrets/actions
2. Click "New repository secret"
3. Name: `HOMEBREW_TAP_TOKEN`
4. Value: Paste the token from step 1
5. Click "Add secret"

#### 3. Add Auto-Update Job to Release Workflow

In `.github/workflows/release.yml`, add this job after the `release` job:

```yaml
jobs:
  # ... existing build and release jobs ...

  update-homebrew:
    name: Update Homebrew formula
    needs: release  # Wait for release to complete
    runs-on: ubuntu-latest
    steps:
      - uses: mislav/bump-homebrew-formula-action@v3
        with:
          # Formula name (must match filename without .rb)
          formula-name: token-count
          
          # Path to formula in homebrew-tap repo
          formula-path: Formula/token-count.rb
          
          # Your homebrew tap repository
          homebrew-tap: shaunburdick/homebrew-tap
          
          # URL to download tarball (uses git tag name)
          download-url: https://github.com/shaunburdick/token-count/releases/download/${{ github.ref_name }}/token-count-${{ github.ref_name }}-x86_64-unknown-linux-gnu.tar.gz
          
          # Commit message template
          commit-message: |
            token-count ${{ github.ref_name }}
            
            Automated update from https://github.com/shaunburdick/token-count/releases/tag/${{ github.ref_name }}
        env:
          # Use the secret we created
          COMMITTER_TOKEN: ${{ secrets.HOMEBREW_TAP_TOKEN }}
```

### What Gets Updated Automatically

The action will update these fields in `Formula/token-count.rb`:

- **`url`**: Points to new release tarball
- **`sha256`**: Recalculated from new tarball (most important!)
- **`version`**: Updated to match git tag (if field exists in formula)

**Example update**:
```diff
class TokenCount < Formula
  desc "Count tokens for LLM models with exact tokenization"
  homepage "https://github.com/shaunburdick/token-count"
- version "0.1.0"
+ version "0.1.1"

  if OS.mac? && Hardware::CPU.intel?
-   url "https://github.com/.../releases/download/v0.1.0/token-count-v0.1.0-x86_64-apple-darwin.tar.gz"
+   url "https://github.com/.../releases/download/v0.1.1/token-count-v0.1.1-x86_64-apple-darwin.tar.gz"
-   sha256 "abc123..."
+   sha256 "def456..."
  end
  # ... same for other platforms
end
```

### Benefits

✅ **Zero manual work** per release  
✅ **No typos** in version numbers or checksums  
✅ **Fast updates** - formula updated within 2-3 minutes of tagging  
✅ **Reliable** - action is used by 200+ projects including major tools  
✅ **Auditable** - all changes logged in git history  
✅ **Safe** - action only has write access to homebrew-tap, not main repo

### Limitations & Fallbacks

**Limitations** (based on action design):
- Only updates single-file formulas (fine for us)
- Assumes simple formula structure without complex Ruby conditionals
- Uses Linux tarball URL for checksum calculation (works for all platforms since we use same source)

**If action fails**:
1. Check GitHub Actions logs in token-count repo
2. Common issues:
   - Token expired → regenerate and update secret
   - Wrong download URL → verify release assets exist
   - Formula syntax error → manually fix formula
3. Manual fallback: Update formula by hand (takes 5 minutes):
   ```bash
   # Download new tarball
   curl -LO https://github.com/.../releases/download/v0.1.1/token-count-v0.1.1-x86_64-unknown-linux-gnu.tar.gz
   
   # Calculate checksum
   shasum -a 256 token-count-v0.1.1-x86_64-unknown-linux-gnu.tar.gz
   
   # Update Formula/token-count.rb with new URL and sha256
   # Commit and push to homebrew-tap
   ```

### Testing the Auto-Update

**First time setup** (v0.1.0):
1. Create homebrew-tap repo
2. Manually create initial Formula/token-count.rb
3. Add auto-update job to release.yml
4. Add HOMEBREW_TAP_TOKEN secret

**Test with next release** (v0.1.1 or v0.2.0):
1. Tag new release: `git tag v0.1.1 && git push origin v0.1.1`
2. Wait for GitHub Actions to complete (~15 minutes)
3. Check homebrew-tap repo - should see new commit from action
4. Test installation: `brew upgrade token-count`
5. Verify version: `token-count --version`

### Alternative Considered

**Manual updates**: Update formula by hand after each release  
**Why rejected**: Error-prone, slows down release process, doesn't scale

### References

- [bump-homebrew-formula-action](https://github.com/mislav/bump-homebrew-formula-action) - Official documentation
- [Homebrew Formula Automation](https://docs.brew.sh/How-To-Open-a-Homebrew-Pull-Request#automated-pull-requests) - Homebrew best practices
- Example projects using this action:
  - [cli/cli](https://github.com/cli/cli/blob/trunk/.github/workflows/deployment.yml) (GitHub CLI)
  - [sharkdp/bat](https://github.com/sharkdp/bat)
  - [BurntSushi/ripgrep](https://github.com/BurntSushi/ripgrep)

---

**Bottom line**: This is a battle-tested, widely-used solution that eliminates manual work and potential errors in Homebrew releases. Setup takes 10 minutes, saves hours over the lifetime of the project.
