# QitOps Roadmap

This document outlines the development roadmap for QitOps, positioning it as a compelling alternative to established testing tools like Postman and k6.

## Vision

QitOps aims to be the premier CLI-first testing tool for developers and QA professionals, offering a unified interface for API, performance, security, and web testing with minimal dependencies and maximum flexibility.

## 🔁 Phase 0: Lock the Core (Current)

Before pursuing feature parity with other tools, we're focusing on making the current implementation rock-solid:

- [x] Implement basic API testing module
- [x] Implement basic performance testing module
- [x] Implement basic security testing module
- [x] Implement basic web testing module
- [x] Finalize and stabilize all core modules
- [x] Implement JSON config schema validation in common.rs
- [ ] Clean CLI output with --format options (JSON, human)
- [ ] Ensure cargo build --release produces a static binary
- [x] Document the config format for each testing mode

**Milestone**: CLI MVP — the "Postman/k6 replacement for power users"

## 📦 Phase 1: Parity Foundation (0-2 months)

### 🧪 API Testing: Postman Lite
- [x] Implement Collections (array of requests in one config)
- [x] Add Variable & Environment interpolation
- [x] Add Request chaining via captured outputs
- [x] Expand Auth to support OAuth2, JWT, API Key
- [ ] Add optional --history logging to a local SQLite file

### ⚙️ Performance Testing: k6 Core
- [ ] Implement load profiles (constant, ramp-up, spike)
- [ ] Support scenarios in one file (multiple endpoints)
- [ ] Track custom metrics (via tags)
- [ ] Add thresholds for pass/fail criteria
- [ ] CLI streaming metrics (real-time bar/line output)

**Milestone**: "Parity with a Purpose"

## 🧑‍💻 Phase 2: Differentiators (2-4 months)

### 🔧 Integrations
- [ ] Git sync for test configs (qitops sync)
- [ ] GitHub Actions example template
- [ ] Dockerfile for CLI-only image
- [ ] Native CI support: --ci-mode

### 📊 Reporting
- [ ] JSON + HTML reporters (extend with templates)
- [ ] Markdown summary logs (for commits)
- [ ] CSV export for audit logs

### 🧪 Data-Driven Testing
- [ ] Parametrize tests with CSV/JSON datasets
- [ ] Support template placeholders: {{user_id}}

**Milestone**: "Beyond Parity"

## 🧠 Phase 3: AI & Ecosystem (4-6 months)

### 🔌 Plugin Architecture (CLI-first)
- [ ] Define QitOpsPlugin trait
- [ ] Implement dynamic plugin loader (optional shared lib .so or .dll)

### 🧠 AI Integration
- [ ] Generate tests from OpenAPI files
- [ ] Record & replay traffic (CLI proxy + storage)
- [ ] Recommend missing edge cases or optimization via LLM

### 🖥️ UI (Optional)
- [ ] Add TUI with textual or ratatui
- [ ] Optional Web UI (if CLI usage + community demand)

**Milestone**: "Next-Generation Testing Platform"

## 📈 Strategic Advantages

| Feature | Why It Wins |
|---------|-------------|
| Unified Tool | One CLI for API, performance, security, and (later) AI |
| Rust Static Binary | No runtime, no nonsense — fast and portable |
| Open Source CLI-first | Speaks devops, CI, Git |
| TestOps as Code | Config-driven testing becomes auditable, repeatable |
| QitOps OS-ready | Tightest vertical integration possible — CLI + OS |

## Contributing

We welcome contributions to help realize this roadmap! See our [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on how to get involved.

## Prioritization

This roadmap is subject to change based on community feedback and evolving requirements. The core team will prioritize features based on:

1. Core stability and reliability
2. Features that enable CI/CD integration
3. Features that differentiate QitOps from competitors
4. Features that expand the ecosystem

## Immediate Next Steps

- Finalize the README and document config structure per test type
- Create a qitops-collections.json sample and scaffold the feature
- Start with simple chaining: response.body.token → next.headers.Authorization
- Prep plugin.rs with trait + registration model (even before loading logic)
