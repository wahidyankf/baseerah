-- Initial schema migration for ose-app-be.
-- Creates the schema_version table as a lightweight placeholder
-- for the migrations tracking infrastructure.
CREATE TABLE IF NOT EXISTS schema_version (
    id SERIAL PRIMARY KEY,
    applied_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
