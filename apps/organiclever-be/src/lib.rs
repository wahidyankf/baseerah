//! `organiclever-be` library crate — `OrganicLever` backend REST API.
//!
//! Exposes the [`app`], [`config`], [`contexts`], and [`errors`] modules.
//! Database lifecycle lives in [`contexts::db`]; messaging in
//! [`contexts::messaging`].

#![forbid(unsafe_code)]

pub mod app;
pub mod config;
pub mod contexts;
pub mod errors;
