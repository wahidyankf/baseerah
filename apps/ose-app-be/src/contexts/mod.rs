//! Bounded contexts for the `ose-app-be` application.

#[path = "ai-orchestration/mod.rs"]
pub mod ai_orchestration;
#[path = "gap-analysis/mod.rs"]
pub mod gap_analysis;
pub mod health;
#[path = "internal-policy/mod.rs"]
pub mod internal_policy;
#[path = "regulatory-source/mod.rs"]
pub mod regulatory_source;
