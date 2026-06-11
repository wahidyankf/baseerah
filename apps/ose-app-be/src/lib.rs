//! `ose-app-be` library crate — OSE Application backend REST API.
//!
//! Exposes the [`app`], [`config`], [`contexts`], [`errors`], and
//! [`messaging`] modules. Database lifecycle lives in [`contexts::db`].

#![forbid(unsafe_code)]

pub mod app;
pub mod config;
pub mod contexts;
pub mod errors;
pub mod messaging;
