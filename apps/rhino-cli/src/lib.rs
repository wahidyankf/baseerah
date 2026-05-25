//! `rhino-cli` library crate — Repository Hygiene & `INtegration` Orchestrator.
//!
//! Exposes the [`cli`] entry point, the [`commands`] dispatch layer,
//! and the [`internal`] implementation modules.
#![forbid(unsafe_code)]

pub mod cli;
pub mod commands;
pub mod internal;
