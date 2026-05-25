//! `organiclever-be` library crate — `OrganicLever` backend REST API.
//!
//! Exposes the [`app`], [`config`], [`errors`], and [`health`] modules.

#![forbid(unsafe_code)]

pub mod app;
pub mod config;
pub mod errors;
pub mod health;
