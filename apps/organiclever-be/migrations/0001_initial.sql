-- Initial schema migration for organiclever-be.
-- Creates the schema_version table as a lightweight placeholder
-- for the migrations tracking infrastructure.
CREATE TABLE IF NOT EXISTS schema_version (
    id SERIAL PRIMARY KEY,
    applied_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
