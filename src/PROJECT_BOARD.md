# QitOps Project Board Structure

This document outlines the structure for the GitHub project board to track QitOps development.

## Board Columns

### 📋 Backlog
All issues that have been identified but not yet prioritized for the current phase.

### 🔍 Triage
New issues that need to be evaluated, refined, and prioritized.

### 🚀 Phase 0: Core (Current Sprint)
Issues that are part of the current Phase 0 work to lock down the core functionality.

### 📦 Phase 1: Parity
Issues related to achieving feature parity with Postman and k6.

### 🧑‍💻 Phase 2: Differentiators
Issues related to implementing differentiating features.

### 🧠 Phase 3: AI & Ecosystem
Issues related to AI integration and ecosystem expansion.

### 👨‍💻 In Progress
Issues currently being worked on.

### 🔍 Review
Issues with pull requests that need review.

### ✅ Done
Completed issues.

## Issue Labels

### Priority
- `priority:high`: Critical for the current phase
- `priority:medium`: Important but not blocking
- `priority:low`: Nice to have

### Type
- `type:feature`: New functionality
- `type:enhancement`: Improvements to existing functionality
- `type:bug`: Something isn't working
- `type:documentation`: Documentation improvements
- `type:test`: Test-related changes

### Module
- `module:api`: API testing module
- `module:performance`: Performance testing module
- `module:security`: Security testing module
- `module:web`: Web testing module
- `module:ai`: AI integration
- `module:common`: Common functionality
- `module:cli`: Command-line interface
- `module:reporting`: Reporting functionality

### Phase
- `phase:0`: Core functionality
- `phase:1`: Parity features
- `phase:2`: Differentiators
- `phase:3`: AI & Ecosystem

### Status
- `status:blocked`: Blocked by another issue
- `status:help-wanted`: Looking for contributors
- `status:good-first-issue`: Good for newcomers

## Initial Issues for Phase 0

1. **Finalize API Testing Module**
   - Labels: `module:api`, `phase:0`, `priority:high`
   - Description: Review and finalize the API testing module, ensuring all core functionality is working correctly.

2. **Finalize Performance Testing Module**
   - Labels: `module:performance`, `phase:0`, `priority:high`
   - Description: Review and finalize the performance testing module, ensuring all core functionality is working correctly.

3. **Finalize Security Testing Module**
   - Labels: `module:security`, `phase:0`, `priority:high`
   - Description: Review and finalize the security testing module, ensuring all core functionality is working correctly.

4. **Implement JSON Config Schema Validation**
   - Labels: `module:common`, `phase:0`, `priority:high`
   - Description: Implement JSON schema validation for test configurations to provide better error messages and validation.

5. **Clean CLI Output Formatting**
   - Labels: `module:cli`, `phase:0`, `priority:medium`
   - Description: Implement clean CLI output with --format options (JSON, human).

6. **Static Binary Compilation**
   - Labels: `type:enhancement`, `phase:0`, `priority:high`
   - Description: Ensure cargo build --release produces a static binary for Linux.

7. **Document Config Formats**
   - Labels: `type:documentation`, `phase:0`, `priority:medium`
   - Description: Document the configuration format for each testing mode.

8. **Create Sample Configurations**
   - Labels: `type:documentation`, `phase:0`, `priority:medium`
   - Description: Create sample configuration files for each testing mode.

## Initial Issues for Phase 1

1. **Implement API Collections**
   - Labels: `module:api`, `phase:1`, `priority:high`
   - Description: Implement collections of API requests in a single configuration file.

2. **Add Variable & Environment Interpolation**
   - Labels: `module:common`, `phase:1`, `priority:high`
   - Description: Add support for variable and environment interpolation in configuration files.

3. **Implement Request Chaining**
   - Labels: `module:api`, `phase:1`, `priority:high`
   - Description: Add support for chaining requests by using data from previous responses.

4. **Expand Authentication Methods**
   - Labels: `module:api`, `phase:1`, `priority:medium`
   - Description: Add support for OAuth2, JWT, and API Key authentication.

5. **Implement Load Profiles**
   - Labels: `module:performance`, `phase:1`, `priority:high`
   - Description: Implement different load profiles (constant, ramp-up, spike) for performance testing.
