# token-count

> A fast, accurate CLI tool for counting tokens in LLM model inputs

[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Status: Specification Phase](https://img.shields.io/badge/Status-Specification%20Phase-yellow.svg)](.specify/PROJECT-SUMMARY.md)

## Overview

`token-count` is a POSIX-style command-line tool that counts tokens for various LLM models using exact tokenization. Pipe any text in, get accurate token counts out—no browser, no API calls, just a fast offline binary.

```bash
# Quick token count
echo "Hello world" | token-count --model gpt-4
2

# From file
token-count --model claude-sonnet < document.txt
1842

# With context info
cat prompt.txt | token-count --model gpt-4 -vv
Model: gpt-4 (cl100k_base encoding)
Tokens: 142
Context Window: 8,192 tokens
Usage: 1.73%
```

## Status: 🏗️ Specification Phase

This project is currently in the **specification phase** following the [spec-kit](https://github.com/github/spec-kit) methodology. We're building detailed specifications before writing code to ensure clarity and completeness.

**✅ Completed**:
- Constitution (project principles & technical decisions)
- Feature 001: Core CLI Token Counting
- Feature 002: Installation & Distribution

**⏳ Next Steps**:
- Planning phase (create implementation architecture)
- Task breakdown
- TDD implementation

See [.specify/PROJECT-SUMMARY.md](.specify/PROJECT-SUMMARY.md) for full details.

## Why token-count?

**Problem**: Developers need to count tokens for LLM API calls, but current solutions require:
- Copy-pasting into web tokenizers (breaks workflow)
- Installing model-specific libraries (maintenance burden)
- Writing custom scripts for each project

**Solution**: One binary, works everywhere, supports all major models:

✅ **Accurate** - Exact tokenization for OpenAI models, smart estimation for others  
✅ **Fast** - <10ms for small inputs, streams large files efficiently  
✅ **Offline** - No network calls, no external dependencies  
✅ **Cross-platform** - Windows, Linux, macOS (Intel + Apple Silicon)  
✅ **Simple** - POSIX-style interface, works like `wc` or `grep`

## Planned Features (MVP v0.1.0)

### Supported Models
- **OpenAI**: GPT-3.5 Turbo, GPT-4, GPT-4 Turbo, GPT-4o (exact tokenization)
- **Post-MVP**: Claude, Gemini, Llama, Mistral

### CLI Features
- Stdin piping and file redirection
- Model aliases (`gpt4`, `claude-sonnet`)
- Multiple verbosity levels (`-v`, `-vv`, `-vvv`)
- Smart error messages with suggestions
- Default model (gpt-3.5-turbo)

### Installation Methods
```bash
# Homebrew (macOS/Linux)
brew install shaunburdick/tap/token-count

# Install script (Linux/macOS)
curl -fsSL https://raw.githubusercontent.com/shaunburdick/token-count/main/install.sh | bash

# Cargo
cargo install token-count

# Manual download
# Download from GitHub Releases
```

## Architecture Principles

From our [Constitution](.specify/memory/constitution.md):

1. **POSIX Simplicity** - Behaves like standard Unix utilities
2. **Accuracy Over Speed** - Exact tokenization or we don't support it
3. **Zero External Dependencies** - Single offline binary
4. **Cross-Platform First-Class** - All platforms equally supported
5. **Fail Fast with Clear Errors** - No silent failures
6. **Installation Should Be Trivial** - One-command install
7. **Semantic Versioning** - Predictable upgrade paths

## Technical Stack

- **Language**: Rust 1.75.0+ (stable)
- **CLI Parsing**: clap 4.6.0+ (derive API)
- **Tokenization**: 
  - tiktoken-rs 0.9.1+ (OpenAI models)
  - llm-tokenizer 1.3.0+ (multi-provider)
- **Error Handling**: anyhow 1.0.102+
- **Quality**: 80%+ test coverage, zero clippy warnings

## Development Roadmap

### Phase 1: Specification ✅ (Complete)
- [x] Constitution ratified
- [x] Feature specifications complete
- [x] All clarifications resolved

### Phase 2: Planning (Next)
- [ ] Technical architecture design
- [ ] Data model definitions
- [ ] Library research and evaluation

### Phase 3: MVP Implementation (v0.1.0)
- [ ] Core tokenization engine
- [ ] OpenAI model support
- [ ] CLI interface
- [ ] Cross-platform builds
- [ ] Installation scripts

### Phase 4: Extended Models (v0.2.0)
- [ ] Anthropic Claude support
- [ ] Google Gemini support

### Phase 5: Additional Providers (v0.3.0)
- [ ] Meta Llama support
- [ ] Mistral support

## Contributing

We're currently in the specification phase. Contributions are welcome once we begin implementation!

For now, you can:
- Review specifications in [.specify/](.specify/)
- Provide feedback on features in GitHub Discussions
- Star the repo to follow progress

## License

MIT License - see [LICENSE](LICENSE) for details

## Acknowledgments

Built with:
- [tiktoken-rs](https://github.com/zurawiki/tiktoken-rs) - Rust tiktoken implementation
- [clap](https://github.com/clap-rs/clap) - Command line argument parser
- [spec-kit](https://github.com/github/spec-kit) - Specification-driven development methodology

---

**Status**: 🏗️ In Development | **Phase**: Specification Complete  
**Author**: [Shaun Burdick](https://github.com/shaunburdick)
