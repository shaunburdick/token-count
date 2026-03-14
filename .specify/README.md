# Specification Directory

This directory contains all specification artifacts for the token-counter project, following the [spec-kit](https://github.com/github/spec-kit) methodology.

## 📁 Directory Structure

```
.specify/
├── PROJECT-SUMMARY.md          # High-level project overview
├── memory/
│   └── constitution.md         # Project principles & technical decisions
├── features/
│   ├── 001-core-cli.md        # Core CLI token counting functionality
│   └── 002-installation.md    # Installation & distribution
├── scripts/                    # Spec-kit automation scripts
└── templates/                  # Document templates
```

## 📖 Reading Order

1. **Start Here**: [PROJECT-SUMMARY.md](./PROJECT-SUMMARY.md) - Overview of the entire project
2. **Core Principles**: [memory/constitution.md](./memory/constitution.md) - The "law of the land"
3. **Features**:
   - [001-core-cli.md](./features/001-core-cli.md) - Main CLI functionality
   - [002-installation.md](./features/002-installation.md) - How users install the tool

## 🎯 Current Status

**Phase**: Specification Complete ✅  
**Next Phase**: Planning (create implementation plans)

### What's Complete
- ✅ Constitution ratified (v1.0.0)
- ✅ Feature 001 specification (v1.0)
- ✅ Feature 002 specification (v1.0)
- ✅ All clarifications resolved
- ✅ Zero ambiguities remaining

### What's Next
Hand off to `modern-architect-engineer` agent to create:
1. Implementation plans
2. Data models
3. API contracts (if applicable)
4. Task breakdown

## 🏗️ Spec-Kit Workflow

### Phase 1: Constitution ✅
Create the project's "north star" - principles that guide all decisions.

**Artifact**: `memory/constitution.md`

### Phase 2: Specification ✅
Define features with user stories, requirements, and acceptance criteria.

**Artifacts**: `features/###-feature-name.md`

### Phase 3: Clarification ✅
Resolve all ambiguities before planning begins.

**Status**: All questions answered and documented in feature specs.

### Phase 4: Planning (NEXT)
Architect translates specs into technical implementation plans.

**Command**:
```bash
git checkout -b 001-core-cli
.specify/scripts/bash/setup-plan.sh --json
# Then fill in specs/001-core-cli/plan.md
```

**Artifacts**: `specs/###-feature-name/plan.md`, `research.md`, `data-model.md`

### Phase 5: Tasking (PENDING)
Break plan into ordered, actionable tasks.

**Artifacts**: `specs/###-feature-name/tasks.md`

### Phase 6: Implementation (PENDING)
Execute tasks with TDD approach.

**Deliverables**: Working code, passing tests, quality checks green

## 📝 Document Versions

| Document | Version | Last Updated | Status |
|----------|---------|--------------|--------|
| Constitution | 1.0.0 | 2026-03-13 | Ratified |
| Feature 001 | 1.0 | 2026-03-13 | Specified |
| Feature 002 | 1.0 | 2026-03-13 | Specified |

## 🔗 Key Links

- **Spec-Kit**: https://github.com/github/spec-kit
- **Repository**: https://github.com/shaunburdick/token-count
- **Crates.io**: https://crates.io/crates/token-counter (will be published)

## 🤝 Contributing to Specs

### Proposing Changes
1. Open GitHub Discussion with proposed change
2. Reference specific requirement (FR-XXX) or principle
3. Explain rationale (why current spec is insufficient)
4. Allow 1 week for feedback

### Amending Constitution
1. Propose amendment in Discussion
2. Requires approval from project owner
3. Version bump: Major = breaking principle change, Minor = new principle, Patch = clarification
4. Document in "Constitutional Amendments" section

### Updating Features
1. Create branch: `spec/update-feature-001`
2. Update feature spec with version bump (e.g., v1.0 → v1.1)
3. Add entry to "Clarifications Applied" section
4. Create PR with rationale

## ⚠️ Important Notes

- **Constitution is Law**: All PRs must align with constitutional principles
- **No Ambiguity**: Specs must be implementable without asking questions
- **Clarifications Are Permanent**: Once resolved, clarifications become requirements
- **Feature Branch Per Feature**: Always create branches like `001-feature-name`

## 🎓 Spec-Kit Commands (OpenCode)

These commands are available via slash commands in OpenCode:

```bash
/speckit.constitution    # Establish project principles
/speckit.specify         # Create feature specifications
/speckit.clarify         # Ask structured questions
/speckit.plan            # Create implementation plans
/speckit.tasks           # Generate actionable tasks
/speckit.implement       # Execute implementation
/speckit.analyze         # Cross-artifact consistency check
/speckit.checklist       # Generate quality checklists
```

---

**Last Updated**: 2026-03-13  
**Specification Phase**: Complete ✅
