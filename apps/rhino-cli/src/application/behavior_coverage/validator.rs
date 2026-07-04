//! Per-level @covers behavior coverage engine — GREEN implementation.

use std::collections::HashMap;

use super::types::{BehaviorCoverageViolation, CoversMarker, ProjectEnvelope, ScenarioSpec};

/// Validates @covers coverage for the given scenarios and markers.
///
/// Rules enforced:
/// - Untagged non-wip scenario → `UntaggedScenario`
/// - Scenario tag outside project envelope P → `LevelOutsideEnvelope`
/// - Missing marker at a required level → `MissingCoverage`
/// - Marker at a level not in the scenario's own tags S → `CoverageAtUndeclaredLevel`
/// - Marker referencing an unknown scenario → `OrphanMarker`
/// - `@wip` scenarios are fully exempt.
pub fn validate(
    scenarios: &[ScenarioSpec],
    markers: &[CoversMarker],
    envelope: &ProjectEnvelope,
) -> Vec<BehaviorCoverageViolation> {
    let mut violations = Vec::new();

    // Build lookup: (feature_path, scenario_title) → &ScenarioSpec
    let scenario_lookup: HashMap<(&str, &str), &ScenarioSpec> = scenarios
        .iter()
        .map(|s| ((s.feature_path.as_str(), s.title.as_str()), s))
        .collect();

    // Validate each scenario.
    for scenario in scenarios {
        if scenario.is_wip {
            continue; // @wip is fully exempt
        }

        if scenario.level_tags.is_empty() {
            violations.push(BehaviorCoverageViolation::UntaggedScenario {
                feature_path: scenario.feature_path.clone(),
                title: scenario.title.clone(),
            });
            continue; // Cannot check coverage without a declared level set
        }

        for level in &scenario.level_tags {
            // S ⊆ P check
            if !envelope.levels.contains(level) {
                violations.push(BehaviorCoverageViolation::LevelOutsideEnvelope {
                    feature_path: scenario.feature_path.clone(),
                    title: scenario.title.clone(),
                    required_level: *level,
                });
            }

            // Every level in S must have at least one marker.
            let covered = markers.iter().any(|m| {
                m.feature_path == scenario.feature_path
                    && m.scenario_title == scenario.title
                    && m.level == *level
            });
            if !covered {
                violations.push(BehaviorCoverageViolation::MissingCoverage {
                    feature_path: scenario.feature_path.clone(),
                    title: scenario.title.clone(),
                    missing_level: *level,
                });
            }
        }
    }

    // Validate each marker.
    for marker in markers {
        match scenario_lookup.get(&(marker.feature_path.as_str(), marker.scenario_title.as_str())) {
            None => {
                violations.push(BehaviorCoverageViolation::OrphanMarker {
                    source_file: marker.source_file.clone(),
                    feature_path: marker.feature_path.clone(),
                    scenario_title: marker.scenario_title.clone(),
                });
            }
            Some(scenario) => {
                // Marker at a level not declared in S is over-coverage.
                if !scenario.is_wip && !scenario.level_tags.contains(&marker.level) {
                    violations.push(BehaviorCoverageViolation::CoverageAtUndeclaredLevel {
                        source_file: marker.source_file.clone(),
                        feature_path: marker.feature_path.clone(),
                        title: scenario.title.clone(),
                        extra_level: marker.level,
                    });
                }
            }
        }
    }

    violations
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::behavior_coverage::types::*;
    use std::collections::HashSet;

    // @covers specs/apps/rhino/behavior/rhino-cli/gherkin/specs/behavior-coverage.feature:An untagged scenario fails the gate
    #[test]
    fn untagged_scenario_fails_gate() {
        let scenarios = vec![ScenarioSpec {
            feature_path: "specs/apps/example/foo.feature".to_string(),
            title: "Untagged scenario".to_string(),
            level_tags: HashSet::new(), // no tags
            is_wip: false,
        }];
        let envelope = ProjectEnvelope {
            levels: [TestLevel::Unit].into_iter().collect(),
        };
        let violations = validate(&scenarios, &[], &envelope);
        assert!(
            !violations.is_empty(),
            "Expected UntaggedScenario violation, got none"
        );
        assert!(
            violations
                .iter()
                .any(|v| matches!(v, BehaviorCoverageViolation::UntaggedScenario { .. })),
            "Expected UntaggedScenario variant, got: {violations:?}"
        );
    }

    // @covers specs/apps/rhino/behavior/rhino-cli/gherkin/specs/behavior-coverage.feature:A scenario requiring a level outside the project envelope fails
    #[test]
    fn scenario_requiring_level_outside_envelope_fails() {
        let scenarios = vec![ScenarioSpec {
            feature_path: "specs/apps/example/foo.feature".to_string(),
            title: "Integration scenario".to_string(),
            level_tags: [TestLevel::Integration].into_iter().collect(),
            is_wip: false,
        }];
        // Project envelope only declares unit
        let envelope = ProjectEnvelope {
            levels: [TestLevel::Unit].into_iter().collect(),
        };
        let violations = validate(&scenarios, &[], &envelope);
        assert!(
            violations
                .iter()
                .any(|v| matches!(v, BehaviorCoverageViolation::LevelOutsideEnvelope { .. })),
            "Expected LevelOutsideEnvelope violation, got: {violations:?}"
        );
    }

    // @covers specs/apps/rhino/behavior/rhino-cli/gherkin/specs/behavior-coverage.feature:A scenario not covered at a required level fails
    #[test]
    fn scenario_not_covered_at_required_level_fails() {
        let feature_path = "specs/apps/example/foo.feature".to_string();
        let title = "Multi-level scenario".to_string();
        let scenarios = vec![ScenarioSpec {
            feature_path: feature_path.clone(),
            title: title.clone(),
            // Requires unit AND e2e
            level_tags: [TestLevel::Unit, TestLevel::E2e].into_iter().collect(),
            is_wip: false,
        }];
        // Only unit marker provided
        let markers = vec![CoversMarker {
            source_file: "apps/example/src/test.rs".to_string(),
            level: TestLevel::Unit,
            feature_path: feature_path.clone(),
            scenario_title: title.clone(),
        }];
        let envelope = ProjectEnvelope {
            levels: [TestLevel::Unit, TestLevel::E2e].into_iter().collect(),
        };
        let violations = validate(&scenarios, &markers, &envelope);
        assert!(
            violations.iter().any(|v| matches!(
                v,
                BehaviorCoverageViolation::MissingCoverage {
                    missing_level: TestLevel::E2e,
                    ..
                }
            )),
            "Expected MissingCoverage(e2e) violation, got: {violations:?}"
        );
    }

    // @covers specs/apps/rhino/behavior/rhino-cli/gherkin/specs/behavior-coverage.feature:An @covers at an undeclared level fails
    #[test]
    fn covers_at_undeclared_level_fails() {
        let feature_path = "specs/apps/example/foo.feature".to_string();
        let title = "Unit-only scenario".to_string();
        let scenarios = vec![ScenarioSpec {
            feature_path: feature_path.clone(),
            title: title.clone(),
            level_tags: [TestLevel::Unit].into_iter().collect(), // only @unit
            is_wip: false,
        }];
        // Marker at e2e level — but scenario only requires unit
        let markers = vec![CoversMarker {
            source_file: "apps/example-e2e/tests/test.spec.ts".to_string(),
            level: TestLevel::E2e,
            feature_path: feature_path.clone(),
            scenario_title: title.clone(),
        }];
        let envelope = ProjectEnvelope {
            levels: [TestLevel::Unit, TestLevel::E2e].into_iter().collect(),
        };
        let violations = validate(&scenarios, &markers, &envelope);
        assert!(
            violations.iter().any(|v| matches!(
                v,
                BehaviorCoverageViolation::CoverageAtUndeclaredLevel { .. }
            )),
            "Expected CoverageAtUndeclaredLevel violation, got: {violations:?}"
        );
    }

    // @covers specs/apps/rhino/behavior/rhino-cli/gherkin/specs/behavior-coverage.feature:An orphan @covers marker fails the gate
    #[test]
    fn orphan_covers_marker_fails_gate() {
        // No scenarios — so any marker is orphaned
        let markers = vec![CoversMarker {
            source_file: "apps/example/src/test.rs".to_string(),
            level: TestLevel::Unit,
            feature_path: "specs/apps/example/foo.feature".to_string(),
            scenario_title: "Non-existent scenario".to_string(),
        }];
        let envelope = ProjectEnvelope {
            levels: [TestLevel::Unit].into_iter().collect(),
        };
        let violations = validate(&[], &markers, &envelope);
        assert!(
            violations
                .iter()
                .any(|v| matches!(v, BehaviorCoverageViolation::OrphanMarker { .. })),
            "Expected OrphanMarker violation, got: {violations:?}"
        );
    }

    // @covers specs/apps/rhino/behavior/rhino-cli/gherkin/specs/behavior-coverage.feature:A @wip scenario is exempt from coverage
    #[test]
    fn wip_scenario_is_exempt_from_coverage() {
        let scenarios = vec![ScenarioSpec {
            feature_path: "specs/apps/example/foo.feature".to_string(),
            title: "WIP scenario".to_string(),
            level_tags: HashSet::new(), // no tags needed — @wip exempts
            is_wip: true,
        }];
        let envelope = ProjectEnvelope {
            levels: [TestLevel::Unit].into_iter().collect(),
        };
        let violations = validate(&scenarios, &[], &envelope);
        // @wip scenarios must NOT produce violations
        assert!(
            violations.is_empty(),
            "Expected no violations for @wip scenario, got: {violations:?}"
        );
    }
}
