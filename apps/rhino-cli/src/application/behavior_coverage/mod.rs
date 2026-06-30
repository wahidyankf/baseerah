//! Per-level @covers behavior coverage engine.
//!
//! Validates that every Gherkin scenario is explicitly covered at each test level
//! it is tagged for. Scenarios self-tag required levels (@unit/@integration/@e2e);
//! tests declare coverage via `// @covers <repo-path>:<scenario-title>` markers;
//! coverage is correct when marker-levels == S exactly.

pub mod types;
pub mod validator;

// @covers specs/apps/rhino/behavior/rhino-cli/gherkin/specs/behavior-coverage.feature:An untagged scenario fails the gate
// @covers specs/apps/rhino/behavior/rhino-cli/gherkin/specs/behavior-coverage.feature:A scenario requiring a level outside the project envelope fails
// @covers specs/apps/rhino/behavior/rhino-cli/gherkin/specs/behavior-coverage.feature:A scenario not covered at a required level fails
// @covers specs/apps/rhino/behavior/rhino-cli/gherkin/specs/behavior-coverage.feature:An @covers at an undeclared level fails
// @covers specs/apps/rhino/behavior/rhino-cli/gherkin/specs/behavior-coverage.feature:An orphan @covers marker fails the gate
// @covers specs/apps/rhino/behavior/rhino-cli/gherkin/specs/behavior-coverage.feature:A @wip scenario is exempt from coverage
