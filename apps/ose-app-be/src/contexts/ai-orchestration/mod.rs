//! AI orchestration bounded context — wraps LLM calls (`OpenRouter`), prompt management,
//! retry/backoff, and token-budget accounting.
//! Detailed domain types and services added in ai-orchestration feature plan.

pub mod api;
pub mod application;
pub mod domain;
pub mod infrastructure;
