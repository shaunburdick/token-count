# Token Counter CLI - Project Summary

**Project**: token-count  
**Status**: Specification Complete ✅  
**Last Updated**: 2026-03-13  
**Phase**: Ready for Planning (Phase 4)

---

## 📋 Overview

A POSIX-style CLI tool for counting tokens in LLM model inputs using exact tokenization. Built in Rust for maximum performance and cross-platform compatibility.

**Core Value Proposition**: 
> Pipe any text into `token-count` and instantly get accurate token counts for GPT-4, Claude, Gemini, and other LLM models—no browser, no API calls, just a fast, offline binary that works everywhere.

---

## 🎯 MVP Scope (v0.1.0)

### What's Included
✅ **OpenAI Model Support** (exact tokenization via tiktoken-rs)
- GPT-3.5 Turbo
- GPT-4 (base, turbo, o)
- Model aliases (gpt4, gpt35, etc.)

✅ **CLI Features**
- Stdin piping (`echo "text" | token-count --model gpt-4`)
- Multiple verbosity levels (0-3)
- Smart model name resolution
- Clear error messages with suggestions

✅ **Installation Methods**
- `curl | bash` installer (Linux/macOS)
- Homebrew tap
- Cargo install
- GitHub Releases (manual download)

✅ **Quality Standards**
- 80%+ test coverage
- Cross-platform CI (Linux, macOS, Windows)
- <30MB binary size
- <10ms latency for small inputs

### What's Deferred (Post-MVP)
⏳ **Phase 2 (v0.2.0)**: Anthropic Claude + Google Gemini support  
⏳ **Phase 3 (v0.3.0)**: Meta Llama + Mistral support  
❌ **Never**: Cost estimation, REPL mode, config files, GUI

---

## 📂 Specification Artifacts

### Constitution
**File**: `.specify/memory/constitution.md`

**Core Principles** (7):
1. POSIX Simplicity - Behaves like wc/grep/cat
2. Accuracy Over Speed - Exact tokenization or nothing
3. Zero External Dependencies - Single offline binary
4. Cross-Platform First-Class - Windows = Linux = macOS
5. Fail Fast with Clear Errors - No silent failures
6. Installation Should Be Trivial - One-command install
7. Semantic Versioning Strictly Enforced

**Technical Stack**:
- Language: Rust 1.75.0+ (stable)
- CLI: clap 4.6.0+
- Tokenization: tiktoken-rs 0.9.1+, llm-tokenizer 1.3.0+
- Error Handling: anyhow 1.0.102+

**Quality Gates**:
- Unit test coverage ≥80%
- `cargo clippy` zero warnings
- Cross-platform CI tests
- Binary size ≤30MB

---

### Feature 001: Core CLI Token Counting
**File**: `.specify/features/001-core-cli.md`

**User Stories** (9):
- US-001: Quick token count from stdin
- US-002: Model alias support (gpt4, claude-sonnet, etc.)
- US-003: Default model (gpt-3.5-turbo)
- US-004: Verbose output levels (-v, -vv, -vvv)
- US-005: File input via redirection
- US-006: Unknown model error handling
- US-007: Invalid UTF-8 error handling
- US-008: Empty input handling (print 0)
- US-009: Help and version info

**Key Requirements**:
- **FR-001**: CLI parsing with clap derive API
- **FR-002**: Streaming input processing (handles >1GB)
- **FR-003**: OpenAI model resolution (exact + aliases + provider format)
- **FR-004**: Exact tokenization via tiktoken-rs
- **FR-005**: Four verbosity levels with detailed output
- **FR-006**: Comprehensive error handling (exit codes 0/1/2)
- **FR-007**: Performance targets (<10ms for <10KB input)
- **FR-008**: Cross-platform compatibility (Linux/macOS/Windows)

**Testing Strategy**:
- Unit tests for all tokenization logic
- Integration tests for each user story
- Performance benchmarks (small/medium/large inputs)
- Cross-platform CI tests

---

### Feature 002: Installation & Distribution
**File**: `.specify/features/002-installation.md`

**User Stories** (5):
- US-010: One-line install (curl | bash)
- US-011: Homebrew installation
- US-012: Cargo installation
- US-013: GitHub Releases manual download
- US-014: Checksum verification

**Key Requirements**:
- **FR-010**: GitHub Actions release pipeline (build + publish)
- **FR-011**: Install script with platform detection + checksum verification
- **FR-012**: Homebrew formula in homebrew-tap repository
- **FR-013**: Cargo publish to crates.io
- **FR-014**: Release checklist (pre-release → release → post-release)
- **FR-015**: Documentation (README, INSTALL, CHANGELOG)

**Build Targets**:
- Linux x64 (GNU + musl)
- macOS x64 (Intel)
- macOS ARM64 (Apple Silicon)
- Windows x64

**Testing Strategy**:
- Install script testing on multiple distros
- Homebrew formula testing (manual)
- GitHub Actions workflow verification
- Cross-platform binary functionality tests

---

## 📊 Feature Summary Table

| Feature | Priority | Status | User Stories | Dependencies |
|---------|----------|--------|--------------|--------------|
| 001: Core CLI | P1 (MVP) | ✅ Specified | 9 | None |
| 002: Installation | P1 (MVP) | ✅ Specified | 5 | Feature 001 |

**Total User Stories**: 14  
**Total Functional Requirements**: 24 (FR-001 through FR-015, plus FR-003a)

---

## 🔄 Development Phases

### Phase 1: Specification (COMPLETE ✅)
- [x] Constitution ratified
- [x] Feature 001 specification complete
- [x] Feature 002 specification complete
- [x] All clarifications resolved
- [x] Zero ambiguities remaining

### Phase 2: Planning (NEXT STEP)
**Assigned to**: `modern-architect-engineer` agent

**Deliverables**:
1. Implementation plan (`specs/001-core-cli/plan.md`)
2. Research findings (`specs/001-core-cli/research.md`)
3. Data models (`specs/001-core-cli/data-model.md`)
4. API contracts (if applicable)
5. Quickstart validation scenarios

**Planning Steps**:
```bash
# Create feature branch
git checkout -b 001-core-cli

# Run setup script
.specify/scripts/bash/setup-plan.sh --json

# Architect fills in plan documents
# ... (modern-architect-engineer work)
```

### Phase 3: Task Breakdown (PENDING)
**Assigned to**: `modern-architect-engineer` agent (continues)

**Deliverables**:
- Ordered task list in `specs/001-core-cli/tasks.md`
- Each task is actionable, testable, completable

### Phase 4: Implementation (PENDING)
**Assigned to**: `modern-architect-engineer` agent (continues)

**Deliverables**:
- Working code with TDD approach
- All tests passing
- Quality checks green (clippy, fmt, tests)

---

## 🎓 Clarifications Resolved

All questions from initial specification were answered and documented:

### Model Support
- **P1 (MVP)**: OpenAI (all GPT models)
- **P2 (v0.2.0)**: Anthropic Claude, Google Gemini
- **P3 (v0.3.0)**: Meta Llama, Mistral
- **Not Supported**: DeepSeek, Qwen (insufficient library support)

### Accuracy Strategy
Smart hybrid approach:
- Exact tokenization for OpenAI (tiktoken-rs)
- Estimation with warning for Claude/Gemini (char-based + 20% margin)
- Never claim support for models we can't tokenize accurately

### Output Formats
Four verbosity levels confirmed:
- Level 0: Number only (`142`)
- Level 1 (-v): Model + count
- Level 2 (-vv): Add context window usage
- Level 3 (-vvv): Add token IDs + decoded samples

### Model Naming
Support three formats:
- Exact: `gpt-4-turbo-2024-04-09`
- Aliases: `gpt4`, `claude-sonnet`
- Provider format: `openai/gpt-4`

### Installation Priority
P1 methods only:
- curl | bash ✅
- Homebrew ✅
- Cargo ✅
- GitHub Releases ✅

### Explicitly Out of Scope
- ❌ Cost estimation (pricing changes frequently)
- ❌ Model comparison mode (call tool multiple times instead)
- ❌ REPL mode (use shell history)
- ❌ Config files (unnecessary complexity)
- ❌ NPM/APT/RPM packages (not in MVP)

---

## 📈 Success Metrics

### Product Metrics (6 months)
- 🎯 1,000+ GitHub stars
- 🎯 10,000+ installs via Homebrew/Cargo
- 🎯 <1% error rate from user feedback
- 🎯 95% of operations complete in <100ms

### Technical Metrics
- 🎯 ≥80% test coverage maintained
- 🎯 ≥99% build success rate on CI/CD
- 🎯 Monthly releases for first 6 months
- 🎯 <48 hour response time on bug reports

### Operational Metrics
- 🎯 <25MB average binary size across platforms
- 🎯 <100MB memory usage for typical operations
- 🎯 100% cross-platform test parity

---

## 🚀 Next Actions

### For Specification Author (COMPLETE)
- [x] Constitution created and ratified
- [x] Feature 001 specification written
- [x] Feature 002 specification written
- [x] All clarifications documented as requirements
- [x] Summary document created

### For User (YOU - Action Required)
Please review the specifications and approve before proceeding:

1. **Read Constitution** (`.specify/memory/constitution.md`)
   - Do the 7 core principles align with your vision?
   - Are the technical decisions sound?
   - Any concerns about quality standards?

2. **Review Feature 001** (`.specify/features/001-core-cli.md`)
   - Do the 9 user stories cover all MVP functionality?
   - Are output formats what you expected?
   - Any missing edge cases?

3. **Review Feature 002** (`.specify/features/002-installation.md`)
   - Are installation methods comprehensive?
   - Is the release process clear?
   - Any platform-specific concerns?

4. **Approve or Request Changes**
   - If approved → Hand off to `modern-architect-engineer` for planning
   - If changes needed → Specify what to adjust

### For Architect (PENDING USER APPROVAL)
Once approved:

```bash
# Create feature branch
git checkout -b 001-core-cli

# Initialize planning
cd /home/shaunburdick/github/shaunburdick/token-count
.specify/scripts/bash/setup-plan.sh --json

# Hand off to modern-architect-engineer agent with this prompt:
"""
Create implementation plan for Feature 001 (Core CLI Token Counting).

Context:
- Constitution: .specify/memory/constitution.md
- Feature Spec: .specify/features/001-core-cli.md
- Dependencies: tiktoken-rs 0.9.1+, clap 4.6.0+

Generate:
1. Technical architecture (modules, traits, data flow)
2. Research findings (library evaluation, cross-platform considerations)
3. Data models (CLI args, model definitions, output formats)
4. Quickstart validation scenarios

Prioritize: POSIX simplicity, accuracy, zero external deps.
"""
```

---

## 📚 Reference Links

- **Repository**: https://github.com/shaunburdick/token-count
- **Spec-Kit Methodology**: https://github.com/github/spec-kit
- **tiktoken-rs**: https://crates.io/crates/tiktoken-rs
- **clap**: https://crates.io/crates/clap
- **Homebrew Tap Guide**: https://docs.brew.sh/How-to-Create-and-Maintain-a-Tap

---

## ✅ Specification Quality Checklist

- [x] Constitution exists and defines project principles
- [x] All requirements are clear and unambiguous
- [x] Acceptance criteria are specific and testable
- [x] Technology recommendations include specific versions
- [x] Quality gates and CI/CD practices are specified
- [x] Accessibility considerations included (clear errors, help text)
- [x] Specification is implementable without additional clarification
- [x] All clarification questions answered and documented
- [x] Feature specs have version numbers and "Last Updated" dates
- [x] Zero "[NEEDS CLARIFICATION]" markers remain

---

## 📝 Specification Updates

### Update 1.1.0 (2026-03-13): Package Naming Correction

**Issue**: During research verification, discovered:
1. `token-counter` already taken on crates.io (HuggingFace-based CLI)
2. Homebrew tap installation instructions were incorrect

**Resolution**: Renamed to `token-count` (User approved)

**Changes**:
- Package name: `token-count` (available on crates.io)
- Binary name: `token-count`
- Homebrew tap: `shaunburdick/homebrew-tap`
- Homebrew install: `brew install shaunburdick/tap/token-count`
- Formula file: `Formula/token-count.rb`
- Formula class: `TokenCount`

**Documentation Updated**:
- Constitution v1.1.0 (added Amendment 1.1.0)
- Feature 001 v1.1 (all command examples)
- Feature 002 v1.1 (Homebrew + Cargo sections)
- PROJECT-SUMMARY (all references)
- README (all installation commands)

**Research**: See `.specify/RESEARCH-HOMEBREW-CARGO.md` for complete findings

---

**Specification Phase Complete** ✅  
**Ready for Architecture & Planning Phase** 🏗️
