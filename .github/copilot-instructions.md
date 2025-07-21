# GitHub Copilot Code Review Instructions

## General Review Principles

### Pull Request Size and Scope
- **Flag large PRs**: Comment if a PR changes more than 500 lines or touches many unrelated areas
- **Suggest splitting**: Recommend breaking large changes into smaller, focused PRs
- **Question scope creep**: Ask about unrelated changes that seem outside the PR's stated purpose

### Code Quality and Style
- **Check adherence to style guidelines**: Verify changes follow the project's Rust conventions in [CONTRIBUTING.md](https://github.com/ratatui/ratatui/blob/main/CONTRIBUTING.md#code-formatting)
- **Verify xtask compliance**: Ensure `cargo xtask format` and `cargo xtask lint` would pass
- **Look for AI-generated patterns**: Be suspicious of verbose, overly-commented, or non-idiomatic code

### Architectural Considerations
- **Reference ARCHITECTURE.md**: Point to [ARCHITECTURE.md](https://github.com/ratatui/ratatui/blob/main/ARCHITECTURE.md) for changes affecting crate boundaries
- **Question fundamental changes**: Flag modifications to core configuration, linting rules, or build setup without clear justification
- **Verify appropriate crate placement**: Ensure changes are in the correct crate per the modular structure

### Breaking Changes and Deprecation
- **Require deprecation**: Insist on deprecation warnings rather than immediate removal of public APIs
- **Ask for migration path**: Request clear upgrade instructions for breaking changes
- **Suggest feature flags**: Recommend feature flags for experimental or potentially disruptive changes
- **Reference versioning policy**: Point to the requirement of at least one version notice before removal

### Testing and Documentation
- **Verify test coverage**: Ensure new functionality includes appropriate tests
- **Check for test removal**: Question any removal of existing tests without clear justification
- **Require documentation**: Ensure public APIs are documented with examples
- **Validate examples**: Check that code examples are minimal and follow project style

### Specific Areas of Concern

#### Configuration Changes
- **Lint configuration**: Question changes to `.clippy.toml`, `rustfmt.toml`, or CI configuration
- **Cargo.toml modifications**: Scrutinize dependency changes or workspace modifications
- **Build system changes**: Require justification for xtask or build process modifications

#### Large Code Additions
- **Question necessity**: Ask if large code additions could be implemented more simply
- **Check for duplication**: Look for code that duplicates existing functionality
- **Verify integration**: Ensure new code integrates well with existing patterns

#### File Organization
- **Validate module structure**: Ensure new modules follow the project's organization
- **Check import organization**: Verify imports follow the std/external/local grouping pattern
- **Review file placement**: Confirm files are in appropriate locations per ARCHITECTURE.md

## Comment Templates

### For Large PRs
```
This PR seems quite large with changes across multiple areas. Consider splitting it into smaller, focused PRs:
- Core functionality changes
- Documentation updates  
- Test additions
- Configuration changes

See our [contribution guidelines](https://github.com/ratatui/ratatui/blob/main/CONTRIBUTING.md#keep-prs-small-intentional-and-focused) for more details.
```

### For Breaking Changes
```
This appears to introduce breaking changes. Please consider:
- Adding deprecation warnings instead of immediate removal
- Providing a clear migration path in the PR description
- Following our [deprecation policy](https://github.com/ratatui/ratatui/blob/main/CONTRIBUTING.md#deprecation-notice)
```

### For Configuration Changes
```
Changes to project configuration (linting, formatting, build) should be discussed first. Please explain:
- Why this change is necessary
- What problem it solves
- Whether it affects contributor workflow
```

### For Style Issues
```
Please run `cargo xtask format` and `cargo xtask lint` to ensure code follows our style guidelines. See [CONTRIBUTING.md](https://github.com/ratatui/ratatui/blob/main/CONTRIBUTING.md#code-formatting) for details.
```
