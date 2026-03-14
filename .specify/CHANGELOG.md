# Changelog

All notable changes to the token-count specifications will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Specification Changes

## [Spec 1.1.0] - 2026-03-13

### Changed
- **BREAKING SPEC CHANGE**: Renamed project from `token-counter` to `token-count`
  - **Reason**: `token-counter` already taken on crates.io by another CLI tool
  - **Impact**: All command examples, installation instructions, and documentation updated
  - **Details**: See `.specify/RESEARCH-HOMEBREW-CARGO.md` for full research findings

### Fixed
- Corrected Homebrew tap installation instructions
  - Previous (incorrect): `brew install shaunburdick/tap/token-counter`
  - Corrected: `brew install shaunburdick/tap/token-count`
  - Repository must be named `homebrew-tap` (not `homebrew-token-count`)
  - Formula class name changed from `TokenCounter` to `TokenCount`
  - Formula file: `Formula/token-count.rb`

### Documentation
- Added research document: `.specify/RESEARCH-HOMEBREW-CARGO.md`
- Updated all specifications with correct naming:
  - `.specify/memory/constitution.md`
  - `.specify/features/001-core-cli.md`
  - `.specify/features/002-installation.md`
  - `.specify/PROJECT-SUMMARY.md`
  - `README.md`

## [Spec 1.0.0] - 2026-03-13

### Added
- Initial specification phase complete
- Constitution ratified (v1.0.0)
  - 7 core principles defined
  - Technical stack decisions documented
  - Quality standards established
- Feature 001: Core CLI Token Counting (v1.0)
  - 9 user stories
  - 24 functional requirements
  - Complete testing strategy
- Feature 002: Installation & Distribution (v1.0)
  - 5 user stories
  - Installation methods: curl|bash, Homebrew, Cargo, GitHub Releases
  - Complete CI/CD pipeline specification
- Project summary document created
- README with project overview

### Clarifications Resolved
- Model support priorities (P1: OpenAI/Claude/Gemini, P2: Llama/Mistral)
- Accuracy strategy (exact for OpenAI, estimation for others post-MVP)
- Output verbosity levels (4 levels: default, -v, -vv, -vvv)
- Model naming (exact + aliases + provider format)
- Default model: `gpt-3.5-turbo`
- Installation priorities (curl|bash + Homebrew + Cargo + GitHub Releases)
- No config files (unnecessary complexity)
- Error handling behavior for all edge cases
- Explicitly out of scope: cost estimation, comparison mode, REPL, config files

---

## Specification Version History

| Version | Date | Changes | Status |
|---------|------|---------|--------|
| 1.1.0 | 2026-03-13 | Package naming corrections | Current |
| 1.0.0 | 2026-03-13 | Initial specifications | Superseded |

---

## Notes

- This changelog tracks changes to the **specifications**, not the implementation
- Implementation changelog will be in the project root `CHANGELOG.md` once development begins
- Breaking specification changes increment the major version (Spec X.0.0)
- New features or clarifications increment minor version (Spec 1.X.0)
- Documentation fixes increment patch version (Spec 1.0.X)
