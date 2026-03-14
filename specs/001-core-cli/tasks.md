# Tasks: Core CLI Token Counting

**Branch**: `001-core-cli` | **Date**: 2026-03-13 | **Plan**: [plan.md](./plan.md) | **Spec**: [001-core-cli.md](../../.specify/features/001-core-cli.md)

**Input**: Design documents from `specs/001-core-cli/` (plan.md, research.md, data-model.md, contracts/)

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (US-001 through US-009)
- Exact file paths included in descriptions

---

## Phase 0: Project Setup (Foundation)

**Purpose**: Initialize Rust project with dependencies and basic structure

- [ ] T001 Initialize Cargo project at repository root with metadata (name, version 0.1.0, authors, license MIT, edition 2021, MSRV 1.85.0)
- [ ] T002 Add dependencies to Cargo.toml: clap 4.6+ (derive), tiktoken-rs 0.9.1+, anyhow 1.0.102+, thiserror 1.0+
- [ ] T003 Add dev-dependencies to Cargo.toml: criterion 0.5+ (benchmarks), tempfile 3.0+ (test fixtures)
- [ ] T004 [P] Configure rustfmt in .rustfmt.toml (max_width=100, edition=2021)
- [ ] T005 [P] Configure clippy in .cargo/config.toml (deny warnings, pedantic lints)
- [ ] T006 Create module structure: src/cli/, src/tokenizers/, src/models/, src/output/, src/error.rs
- [ ] T007 Create test structure: tests/integration/, tests/unit/, tests/fixtures/
- [ ] T008 Create scripts/ directory for fixture generation
- [ ] T009 [P] Setup GitHub Actions CI workflow (.github/workflows/ci.yml): Linux Ubuntu 22.04, Rust 1.85, test + lint + build
- [ ] T010 [P] Add cargo audit to CI for security scanning
- [ ] T011 Create benches/ directory for criterion benchmarks
- [ ] T012 Verify: `cargo build` succeeds and CI workflow syntax is valid

**Checkpoint**: Project structure ready, CI configured, dependencies added

---

## Phase 1: Core Tokenization Logic (Heart of the Tool)

**Purpose**: Implement OpenAI tokenization with exact accuracy using tiktoken-rs

### Test Fixture Generation (One-Time Setup)

- [ ] T013 [ALL] Create scripts/generate_fixtures.py per quickstart.md (Python tiktoken 0.5.2, generates JSON with ~50 test cases)
- [ ] T014 [ALL] Run fixture generation script to create tests/fixtures/tokenization_reference.json (ASCII, Unicode, edge cases, 4 encodings)
- [ ] T015 [ALL] Commit generated fixtures to git (source of truth for accuracy validation)

### Tokenizer Trait & OpenAI Implementation

- [ ] T016 [P] [US-001] Define `Tokenizer` trait in src/tokenizers/mod.rs per contracts/tokenizer.rs (count_tokens, get_model_info methods)
- [ ] T017 [P] [US-001] Define `ModelInfo` struct in src/tokenizers/mod.rs (name, encoding, context_window, description)
- [ ] T018 [US-001] Implement `OpenAITokenizer` in src/tokenizers/openai.rs using tiktoken-rs CoreBPE
- [ ] T019 [US-001] Add error handling in OpenAITokenizer (EncodingError, wrap tiktoken-rs errors with context)
- [ ] T020 [US-001] Test OpenAITokenizer with "Hello world" fixture (expect 2 tokens for cl100k_base)

### Model Registry & Alias Resolution

- [ ] T021 [P] [US-002] Define `ModelRegistry` struct in src/tokenizers/registry.rs per contracts/model.rs
- [ ] T022 [P] [US-002] Define `ModelConfig` struct (name, encoding, context_window, aliases)
- [ ] T023 [US-002] Implement ModelRegistry::new() with lazy initialization using once_cell or lazy_static
- [ ] T024 [US-002] Add GPT-3.5-turbo config (cl100k_base, 16385 tokens, aliases: ["gpt-3.5", "gpt35", "openai/gpt-3.5-turbo"])
- [ ] T025 [US-002] Add GPT-4 config (cl100k_base, 128000 tokens, aliases: ["gpt4", "openai/gpt-4"])
- [ ] T026 [US-002] Add GPT-4-turbo config (cl100k_base, 128000 tokens, aliases: ["gpt4-turbo", "openai/gpt-4-turbo"])
- [ ] T027 [US-002] Add GPT-4o config (o200k_base, 128000 tokens, aliases: ["gpt4o", "openai/gpt-4o"])
- [ ] T028 [US-002] Implement get_model() with case-insensitive lookup and alias resolution
- [ ] T029 [US-003] Implement fuzzy model name matching using strsim crate (Levenshtein distance ≤2, suggest alternatives)
- [ ] T030 [US-002] Test model resolution: "gpt-4" → canonical, "gpt4" → "gpt-4", "GPT-4" → "gpt-4"

### Unit Tests for Tokenization Accuracy

- [ ] T031 [P] [US-001] Write unit test in tests/unit/tokenizer_tests.rs: load tokenization_reference.json fixtures
- [ ] T032 [P] [US-001] Test all cl100k_base fixtures (GPT-4, GPT-3.5, GPT-4-turbo) against reference values
- [ ] T033 [P] [US-001] Test all o200k_base fixtures (GPT-4o) against reference values
- [ ] T034 [P] [US-001] Test Unicode handling: "Hello 世界 🌍" → 6 tokens (cl100k_base)
- [ ] T035 [P] [US-001] Test edge cases: empty string → 0 tokens, single char, whitespace variations
- [ ] T036 [US-001] Test large text (1MB fixture): verify streaming works, memory <500MB

### Benchmarks

- [ ] T037 [P] [ALL] Create benches/tokenization.rs with criterion benchmarks
- [ ] T038 [P] [ALL] Benchmark small input (100 bytes): target <10ms
- [ ] T039 [P] [ALL] Benchmark medium input (1MB): target <100ms
- [ ] T040 [P] [ALL] Benchmark large input (100MB): target <5s
- [ ] T041 [ALL] Run `cargo bench` and verify all targets met on Linux

**Checkpoint**: Tokenization core working, all fixtures pass, benchmarks meet targets

---

## Phase 2: CLI Argument Parsing (User Interface)

**Purpose**: Parse command-line arguments with clap, handle all flags

- [ ] T042 [P] [US-009] Define `Cli` struct in src/cli/args.rs with clap derive macros
- [ ] T043 [P] [US-009] Add `--model <MODEL>` argument with default "gpt-3.5-turbo"
- [ ] T044 [P] [US-009] Add `--verbose` flag (count occurrences, 0-3 levels)
- [ ] T045 [P] [US-009] Add `--list-models` flag (print model registry, then exit)
- [ ] T046 [P] [US-009] Add `--help` flag (clap auto-generates from doc comments)
- [ ] T047 [P] [US-009] Add `--version` flag (clap auto-generates from Cargo.toml)
- [ ] T048 [US-009] Implement list_models() function: iterate ModelRegistry, print each model with aliases and context window
- [ ] T049 [US-002] Add model name normalization in Cli::parse(): lowercase, trim whitespace
- [ ] T050 [US-009] Write integration test in tests/integration/help_version.rs: `token-count --help` → contains "USAGE", "OPTIONS", "EXAMPLES"
- [ ] T051 [US-009] Test `token-count --version` → "token-count 0.1.0"
- [ ] T052 [US-002] Test `token-count --list-models` → lists all 4 models with aliases
- [ ] T053 [US-009] Verify help text fits in 24 lines (per quickstart Test 30)

**Checkpoint**: All CLI flags work, help text is clear and concise

---

## Phase 3: Input Processing (Stdin Handling)

**Purpose**: Read stdin with streaming support, handle UTF-8 validation

- [ ] T054 [P] [US-001] Implement read_stdin() in src/cli/input.rs using BufReader with 64KB chunks
- [ ] T055 [P] [US-001] Add UTF-8 validation with helpful error messages (show first invalid byte offset)
- [ ] T056 [US-001] Handle empty input: return Ok("") with 0 bytes
- [ ] T057 [US-006] Implement InvalidUtf8Error in src/error.rs with byte offset context
- [ ] T058 [US-001] Test with ASCII input: echo "Hello" | token-count → no errors
- [ ] T059 [US-001] Test with Unicode input: echo "Hello 世界 🌍" | token-count → no errors
- [ ] T060 [US-006] Test with invalid UTF-8: tests/fixtures/invalid_utf8.bin → error with byte offset
- [ ] T061 [US-005] Test with large file (>100MB): verify streaming works, memory <500MB using valgrind or heaptrack
- [ ] T062 [US-001] Test empty input: echo "" | token-count → output "0"

**Checkpoint**: Stdin reading works for all input sizes, UTF-8 validation catches errors early

---

## Phase 4: Output Formatting (Display Results)

**Purpose**: Format output according to verbosity level (0, 1-2, 3+)

- [ ] T063 [P] [US-004] Define `OutputFormatter` trait in src/output/mod.rs per contracts/output.rs
- [ ] T064 [P] [US-004] Define `TokenizationResult` struct (model_name, token_count, model_info)
- [ ] T065 [P] [US-004] Implement `SimpleFormatter` in src/output/simple.rs (verbosity 0): output number only
- [ ] T066 [P] [US-004] Implement `VerboseFormatter` in src/output/verbose.rs (verbosity 1-2): multi-line with model info, context window percentage
- [ ] T067 [P] [US-004] Implement `DebugFormatter` in src/output/debug.rs (verbosity 3): token IDs, decoded tokens (first 10 only)
- [ ] T068 [US-004] Implement select_formatter() function: match verbosity level to formatter
- [ ] T069 [US-004] Test SimpleFormatter: input "Hello world" → output "2" (number only, no newline)
- [ ] T070 [US-004] Test VerboseFormatter: input "Hello world" → output multi-line with "Model: gpt-4", "Tokens: 2", "Context: 0.0016%"
- [ ] T071 [US-004] Test DebugFormatter: input "Hello world" → output with "Token IDs: [...]", "Sample Decoded: [...]"
- [ ] T072 [US-004] Write integration test in tests/integration/verbosity.rs for all 3 verbosity levels

**Checkpoint**: Output formatting matches feature spec examples exactly

---

## Phase 5: Error Handling (Robustness)

**Purpose**: Handle all error scenarios gracefully with helpful messages and correct exit codes

- [ ] T073 [P] [US-006] Define `TokenError` enum in src/error.rs (InvalidUtf8, UnknownModel, IoError, TokenizationError)
- [ ] T074 [P] [US-006] Implement Display for TokenError with helpful messages
- [ ] T075 [P] [US-006] Implement From<std::io::Error> for TokenError
- [ ] T076 [P] [US-006] Implement From<tiktoken_rs::Error> for TokenError
- [ ] T077 [US-007] Implement UnknownModelError with fuzzy suggestions (use strsim for Levenshtein distance)
- [ ] T078 [US-007] Add error message template: "Unknown model 'gpt5'. Did you mean: gpt-4, gpt-4o, gpt-4-turbo?"
- [ ] T079 [US-008] Implement error-to-exit-code mapping in src/main.rs: InvalidUtf8 → 1, UnknownModel → 2, IoError → 1
- [ ] T080 [US-006] Test invalid UTF-8 error: echo -ne '\xff\xfe' | token-count → stderr "Error: Input contains invalid UTF-8 at byte 0", exit 1
- [ ] T081 [US-007] Test unknown model error: echo "test" | token-count --model gpt5 → stderr with suggestions, exit 2
- [ ] T082 [US-007] Test fuzzy suggestions: echo "test" | token-count --model gpt4-tubro → suggests "gpt-4-turbo"
- [ ] T083 [US-008] Test IO error handling: token-count < /dev/null (closed stdin) → graceful error, exit 1
- [ ] T084 [ALL] Write integration test in tests/integration/error_handling.rs for all error scenarios

**Checkpoint**: All error messages are helpful, exit codes correct, fuzzy suggestions work

---

## Phase 6: Main Binary Integration (Putting It All Together)

**Purpose**: Wire up all components in main.rs, create library API in lib.rs

- [ ] T085 [US-001] Implement count_tokens() public function in src/lib.rs (library API)
- [ ] T086 [US-001] Export Tokenizer trait, ModelInfo, TokenizationResult from lib.rs
- [ ] T087 [US-001] Implement main() in src/main.rs: parse CLI args, read stdin, tokenize, format output, handle errors
- [ ] T088 [US-001] Wire up model selection: CLI arg → ModelRegistry → Tokenizer instance
- [ ] T089 [US-001] Wire up verbosity selection: CLI flag → OutputFormatter selection
- [ ] T090 [US-001] Add exit code mapping: Ok(()) → exit(0), TokenError → exit(1 or 2)
- [ ] T091 [US-001] Test end-to-end: echo "Hello world" | cargo run -- --model gpt-4 → "2"
- [ ] T092 [US-001] Test with file redirection: cargo run -- --model gpt-4 < tests/fixtures/ascii.txt → correct count
- [ ] T093 [US-002] Test model alias: echo "test" | cargo run -- --model gpt4 → same as --model gpt-4
- [ ] T094 [US-004] Test verbosity: echo "test" | cargo run -- --model gpt-4 -v → verbose output
- [ ] T095 [US-004] Test debug mode: echo "test" | cargo run -- --model gpt-4 -vvv → token IDs shown

**Checkpoint**: Binary works end-to-end, all user stories functional

---

## Phase 7: Integration Testing (Quality Assurance - Linux MVP)

**Purpose**: End-to-end tests for all user stories on Linux (Ubuntu 22.04)

### Integration Tests Per User Story

- [ ] T096 [P] [US-001] Write tests/integration/cli_basic.rs: echo "Hello world" | token-count → "2"
- [ ] T097 [P] [US-001] Test stdin piping: cat file.txt | token-count → correct count
- [ ] T098 [P] [US-002] Write tests/integration/model_aliases.rs: test all 4 models with canonical names
- [ ] T099 [P] [US-002] Test all aliases: gpt4, gpt35, gpt4o, gpt4-turbo → resolve correctly
- [ ] T100 [P] [US-003] Test case-insensitive: --model GPT-4, --model Gpt-4 → works
- [ ] T101 [P] [US-004] Write tests/integration/verbosity.rs: test -v, -vv, -vvv output formats
- [ ] T102 [P] [US-005] Write tests/integration/file_input.rs: token-count < file.txt, token-count < /dev/stdin
- [ ] T103 [P] [US-006] Write tests/integration/error_handling.rs: invalid UTF-8, unknown model, empty input
- [ ] T104 [P] [US-007] Test fuzzy model suggestions: --model gpt5, --model gpt4-tubro
- [ ] T105 [P] [US-008] Test exit codes: success → 0, UTF-8 error → 1, unknown model → 2
- [ ] T106 [P] [US-009] Write tests/integration/help_version.rs: --help, --version, --list-models

### Test Fixtures

- [ ] T107 [P] [ALL] Create tests/fixtures/ascii.txt (simple ASCII, ~1KB)
- [ ] T108 [P] [ALL] Create tests/fixtures/unicode.txt (emoji, CJK, Arabic, ~1KB)
- [ ] T109 [P] [ALL] Generate tests/fixtures/large.txt (>10MB for streaming tests)
- [ ] T110 [P] [ALL] Create tests/fixtures/invalid_utf8.bin (byte sequence 0xFF 0xFE)

### Performance & Memory Tests (Linux)

- [ ] T111 [ALL] Run cargo bench on Linux, verify all targets met (see quickstart Test 27)
- [ ] T112 [ALL] Test memory usage with 1GB file using /usr/bin/time -v or valgrind: verify <500MB resident
- [ ] T113 [ALL] Test performance: small (<10KB) <10ms, medium (1MB) <100ms, large (100MB) <5s
- [ ] T114 [ALL] Track binary size: cargo build --release && ls -lh target/release/token-count (expect 40-60MB)

### CI Validation

- [ ] T115 [ALL] Run full test suite on Linux CI: cargo test --all
- [ ] T116 [ALL] Run clippy on CI: cargo clippy -- -D warnings (zero warnings)
- [ ] T117 [ALL] Run rustfmt check on CI: cargo fmt --check
- [ ] T118 [ALL] Run cargo audit on CI: security scan, fail on HIGH/CRITICAL vulnerabilities
- [ ] T119 [ALL] Verify CI passes on PR before merge

**Checkpoint**: All tests pass on Linux, benchmarks meet targets, CI green

---

## Phase 8: Documentation & Polish (User Experience)

**Purpose**: Complete user-facing documentation and examples

- [ ] T120 [P] [ALL] Write README.md with installation instructions (cargo install, GitHub releases)
- [ ] T121 [P] [ALL] Add usage examples to README: basic usage, model selection, verbosity, file input
- [ ] T122 [P] [ALL] Document supported models in README (4 OpenAI models with aliases)
- [ ] T123 [P] [ALL] Create CHANGELOG.md with v0.1.0 release notes (MVP features, Linux-only, OpenAI models)
- [ ] T124 [P] [ALL] Enhance --help text with examples section (per quickstart Test 9)
- [ ] T125 [P] [ALL] Add doc comments to all public functions in src/lib.rs with examples
- [ ] T126 [P] [ALL] Add module-level doc comments in src/tokenizers/mod.rs, src/output/mod.rs
- [ ] T127 [ALL] Run cargo doc to generate API documentation, verify it builds without warnings
- [ ] T128 [ALL] Add CONTRIBUTING.md with development setup instructions
- [ ] T129 [ALL] Add LICENSE file (MIT license per constitution)
- [ ] T130 [ALL] Run all quickstart tests (30 scenarios) to validate completeness
- [ ] T131 [ALL] Final review: verify constitution compliance, all acceptance criteria met

**Checkpoint**: Documentation complete, ready for v0.1.0 release

---

## Dependencies & Execution Order

### Phase Dependencies

1. **Phase 0 (Setup)**: No dependencies - start immediately
2. **Phase 1 (Tokenization)**: Depends on Phase 0 completion (project structure)
3. **Phase 2 (CLI Parsing)**: Can run in parallel with Phase 1 (different files)
4. **Phase 3 (Input)**: Can run in parallel with Phase 1-2 (different files)
5. **Phase 4 (Output)**: Can run in parallel with Phase 1-3 (different files)
6. **Phase 5 (Errors)**: Can run in parallel with Phase 1-4 (different files)
7. **Phase 6 (Integration)**: Depends on Phase 1-5 completion (wires everything together)
8. **Phase 7 (Testing)**: Depends on Phase 6 completion (needs working binary)
9. **Phase 8 (Documentation)**: Can start anytime, finalize after Phase 7

### User Story Mapping

- **US-001** (Basic Usage): Phase 1, 3, 4, 6
- **US-002** (Model Selection): Phase 1, 2
- **US-003** (Case-Insensitive): Phase 1 (model registry)
- **US-004** (Verbosity): Phase 2, 4
- **US-005** (File Input): Phase 3
- **US-006** (UTF-8 Error): Phase 3, 5
- **US-007** (Unknown Model Error): Phase 5
- **US-008** (Exit Codes): Phase 5, 6
- **US-009** (Help/Version): Phase 2

### Parallel Opportunities

- **Phase 0**: Tasks T004, T005, T009, T010 can run in parallel
- **Phase 1**: Fixture generation (T013-T015), trait definition (T016-T017), registry (T021-T022), tests (T031-T036), benchmarks (T037-T040) can run in parallel once core is ready
- **Phase 2**: All CLI arg definitions (T042-T047) can run in parallel
- **Phase 3**: Input reader (T054-T055), error handling (T057), tests (T058-T062) can run in parallel
- **Phase 4**: All formatter implementations (T065-T067) can run in parallel
- **Phase 5**: Error enum definition (T073-T076), fuzzy matching (T077-T078), tests (T080-T084) can run in parallel
- **Phase 7**: All integration tests (T096-T106), fixtures (T107-T110) can run in parallel
- **Phase 8**: All documentation tasks (T120-T129) can run in parallel

### Sequential Requirements

- Fixture generation (T013-T015) BEFORE accuracy tests (T031-T036)
- Tokenizer trait (T016) BEFORE OpenAI impl (T018)
- Model registry (T021-T028) BEFORE fuzzy matching (T029)
- All Phase 1-5 BEFORE Phase 6 integration
- Phase 6 BEFORE Phase 7 testing
- All phases BEFORE Phase 8 finalization

---

## Implementation Strategy

### MVP-First Approach (Recommended)

1. **Sprint 1**: Phase 0 (Setup) → verify `cargo build` works
2. **Sprint 2**: Phase 1 (Tokenization) → core accuracy validated with fixtures
3. **Sprint 3**: Phase 2-3 (CLI + Input) → can read stdin, parse args
4. **Sprint 4**: Phase 4-5 (Output + Errors) → handles all scenarios gracefully
5. **Sprint 5**: Phase 6 (Integration) → end-to-end binary working
6. **Sprint 6**: Phase 7 (Testing) → all user stories validated on Linux
7. **Sprint 7**: Phase 8 (Documentation) → ready for v0.1.0 release

### Parallel Development (Multi-Developer)

After Phase 0 completion:
- **Dev A**: Phase 1 (Tokenization core)
- **Dev B**: Phase 2-3 (CLI + Input) - can work independently
- **Dev C**: Phase 4-5 (Output + Errors) - can work independently

Once Phase 1-5 complete:
- **Dev A**: Phase 6 (Integration)
- **Dev B + C**: Phase 7 (Testing) - write integration tests in parallel

All devs: Phase 8 (Documentation) - divide documentation tasks

---

## Success Criteria

Before marking Feature 001 complete, verify:

- [ ] All 131 tasks completed and checked off
- [ ] All 9 user stories (US-001 through US-009) validated with integration tests
- [ ] All 30 quickstart test scenarios pass on Linux (Ubuntu 22.04)
- [ ] Token counts match reference fixtures (no accuracy regressions)
- [ ] Performance benchmarks meet targets (<10ms for <10KB, <100ms for 1MB, <5s for 100MB)
- [ ] Memory usage <500MB for large files (validated with valgrind)
- [ ] Binary size tracked (expected 40-60MB, acceptable per Amendment 1.3.0)
- [ ] CI pipeline green (test + lint + build + audit passing)
- [ ] Zero clippy warnings (cargo clippy -- -D warnings)
- [ ] Code formatted (cargo fmt --check)
- [ ] Documentation complete (README, CHANGELOG, API docs, help text)
- [ ] All acceptance criteria from feature spec met
- [ ] Constitution compliance verified (all 7 principles followed)

---

## Notes

- [P] tasks can run in parallel (different files, no dependencies)
- [Story] label maps task to specific user story (US-XXX from feature spec)
- Fixture generation is one-time setup, not part of CI (Python tiktoken not required for tests)
- Linux-only for MVP (v0.1.0), cross-platform expansion in v0.2.0+
- Binary size no longer a blocker per Amendment 1.3.0 (accuracy takes precedence)
- Commit after each logical task group (e.g., all Phase 0 setup, tokenizer trait + impl, etc.)
- Run quickstart validation tests frequently to catch regressions early
- Focus on accuracy first, optimize later if needed

**Task Breakdown Version**: 1.0 | **Last Updated**: 2026-03-13 | **Total Tasks**: 131
