//! Diagram kind detection — identifies the type of a Mermaid block from its header.

use super::types::DiagramKind;

/// Detects the kind of a Mermaid diagram from its raw source.
///
/// Returns [`DiagramKind::Flowchart`] for `flowchart`/`graph` headers,
/// [`DiagramKind::State`] for `stateDiagram-v2`/`stateDiagram` headers,
/// and [`DiagramKind::Other`] for all other types (sequence, pie, class, etc.).
///
/// Blank lines and Mermaid comment lines (`%%`, including `%%{init: ...}%%`
/// directives) above the type directive are skipped; the first line after them
/// is treated as the header. Skipping comments is required for correctness:
/// Mermaid permits a comment above the directive, and the repository's diagram
/// convention mandates one for the `TD` exception, so treating a comment as an
/// unrecognised header silently disabled every rule for such blocks.
pub fn detect_kind(source: &str) -> DiagramKind {
    for line in source.lines() {
        let t = line.trim();
        if t.is_empty() || t.starts_with("%%") {
            continue;
        }
        if t.starts_with("flowchart") || t.starts_with("graph ") || t == "graph" {
            return DiagramKind::Flowchart;
        }
        if t.starts_with("stateDiagram-v2") || t.starts_with("stateDiagram") {
            return DiagramKind::State;
        }
        // Any other non-empty first line is an unrecognised diagram type.
        return DiagramKind::Other;
    }
    DiagramKind::Other
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn detect_kind_returns_state_for_state_diagram_v2() {
        assert_eq!(
            detect_kind("stateDiagram-v2\nA --> B"),
            DiagramKind::State,
            "stateDiagram-v2 header must be detected as State"
        );
    }

    #[test]
    fn detect_kind_returns_state_for_state_diagram_v1() {
        assert_eq!(
            detect_kind("stateDiagram\nA --> B"),
            DiagramKind::State,
            "stateDiagram header must be detected as State"
        );
    }

    // -----------------------------------------------------------------------
    // Regression: a leading `%%` comment must not hide the diagram type.
    //
    // Mermaid treats `%%` as a comment anywhere in a block, including above the
    // type directive. Before this fix `detect_kind` skipped blank lines but
    // treated a comment as "some other diagram type", returned `Other`, and the
    // caller (`validate_one_block`) then returned early — silently bypassing
    // every label, width, depth and subgraph rule for that block.
    //
    // This mattered because the repo's own diagram convention *mandates* a `%%`
    // justification comment immediately above the directive for the `TD`
    // exception, and the shared colour-palette header is also a `%%` line — so
    // the diagrams most likely to need checking were exactly the ones skipped.
    // -----------------------------------------------------------------------

    #[test]
    fn detect_kind_skips_leading_comment_before_flowchart() {
        assert_eq!(
            detect_kind("%% Color palette: Blue #0173B2\nflowchart LR\n    A --> B"),
            DiagramKind::Flowchart,
            "a `%%` comment above a flowchart directive must not hide the diagram type"
        );
    }

    #[test]
    fn detect_kind_skips_leading_comment_before_graph_alias() {
        assert_eq!(
            detect_kind("%% TD required: inheritance direction\ngraph TD\n    A --> B"),
            DiagramKind::Flowchart,
            "a `%%` comment above a `graph` directive must not hide the diagram type"
        );
    }

    #[test]
    fn detect_kind_skips_leading_comment_before_state_diagram() {
        assert_eq!(
            detect_kind("%% state machine\nstateDiagram-v2\n    A --> B"),
            DiagramKind::State,
            "a `%%` comment above a stateDiagram directive must not hide the diagram type"
        );
    }

    #[test]
    fn detect_kind_skips_multiple_leading_comments_and_blank_lines() {
        assert_eq!(
            detect_kind("%% first\n\n   %% indented second\n\nflowchart LR\n    A --> B"),
            DiagramKind::Flowchart,
            "several comment and blank lines above the directive must all be skipped"
        );
    }

    #[test]
    fn detect_kind_returns_other_for_commented_sequence_diagram() {
        // Guard against over-correcting: skipping comments must not turn a
        // genuinely unsupported diagram type into a validated one.
        assert_eq!(
            detect_kind("%% a comment\nsequenceDiagram\n    A ->> B: hi"),
            DiagramKind::Other,
            "skipping comments must not reclassify an unsupported diagram type"
        );
    }

    #[test]
    fn detect_kind_returns_other_for_comment_only_block() {
        assert_eq!(
            detect_kind("%% just a comment\n%% and another"),
            DiagramKind::Other,
            "a block with no directive at all remains Other"
        );
    }
}
