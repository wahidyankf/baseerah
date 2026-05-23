// Command modules mirror `apps/rhino-cli/cmd/*.go` (one file per command).
// Populated phase-by-phase as the port progresses.

pub mod agents_detect_duplication;
pub mod agents_sync;
pub mod agents_validate_claude;
pub mod agents_validate_naming;
pub mod agents_validate_sync;
pub mod ddd_bc;
pub mod ddd_ul;
pub mod docs_validate_frontmatter;
pub mod docs_validate_heading_hierarchy;
pub mod docs_validate_links;
pub mod docs_validate_mermaid;
pub mod docs_validate_naming;
pub mod git_pre_commit;
pub mod governance_agents_md_size;
pub mod governance_audit;
pub mod governance_emoji_audit;
pub mod governance_frontmatter_audit;
pub mod governance_layer_coherence;
pub mod governance_license_audit;
pub mod governance_readme_index_audit;
pub mod governance_traceability_audit;
pub mod governance_vendor_audit;
pub mod naming_reporter;
pub mod spec_coverage_validate;
pub mod specs_validate_adoption;
pub mod specs_validate_counts;
pub mod specs_validate_links;
pub mod specs_validate_tree;
pub mod test_coverage_validate;
pub mod workflows_validate_naming;
