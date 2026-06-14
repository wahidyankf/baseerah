-- Mirrors the PGlite client migration
-- 2026_05_01T03_33_00__add_typed_payload_columns
-- (apps/organiclever-app-web/src/contexts/journal/infrastructure/migrations/).
-- Adds typed payload columns to journal_entries plus the routines and settings
-- tables. The matching EF entities + CRUD land in Phase 4.
ALTER TABLE journal_entries
    ADD COLUMN IF NOT EXISTS started_at  TIMESTAMPTZ,
    ADD COLUMN IF NOT EXISTS finished_at TIMESTAMPTZ,
    ADD COLUMN IF NOT EXISTS labels      TEXT[] NOT NULL DEFAULT '{}';

UPDATE journal_entries
    SET started_at  = created_at,
        finished_at = updated_at
    WHERE started_at IS NULL;

ALTER TABLE journal_entries
    ALTER COLUMN started_at  SET NOT NULL,
    ALTER COLUMN finished_at SET NOT NULL;

ALTER TABLE journal_entries
    ADD CONSTRAINT journal_entries_kind_v0
    CHECK (name IN ('workout', 'reading', 'learning', 'meal', 'focus') OR name LIKE 'custom-%');

CREATE TABLE IF NOT EXISTS routines (
    id          TEXT PRIMARY KEY,
    name        TEXT NOT NULL,
    hue         TEXT NOT NULL,
    type        TEXT NOT NULL,
    created_at  TIMESTAMPTZ NOT NULL,
    groups      JSONB NOT NULL DEFAULT '[]'::jsonb
);

CREATE TABLE IF NOT EXISTS settings (
    id            TEXT PRIMARY KEY DEFAULT 'singleton',
    name          TEXT NOT NULL,
    rest_seconds  TEXT NOT NULL,
    dark_mode     BOOLEAN NOT NULL DEFAULT false,
    lang          TEXT NOT NULL DEFAULT 'en',
    CHECK (id = 'singleton')
);
