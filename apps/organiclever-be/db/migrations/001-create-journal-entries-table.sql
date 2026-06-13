-- Mirrors the PGlite client migration
-- 2026_04_28T14_05_30__create_journal_entries_table
-- (apps/organiclever-web/src/contexts/journal/infrastructure/migrations/).
-- The journal EF entity + CRUD endpoints are implemented in Phase 4; this
-- migration only establishes the schema so DbUp records it on boot.
CREATE TABLE IF NOT EXISTS journal_entries (
    id          TEXT PRIMARY KEY,
    name        TEXT NOT NULL CHECK (length(name) > 0),
    payload     JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at  TIMESTAMPTZ NOT NULL,
    updated_at  TIMESTAMPTZ NOT NULL,
    storage_seq BIGSERIAL
);

CREATE INDEX IF NOT EXISTS journal_entries_created_at_desc
    ON journal_entries (created_at DESC, storage_seq ASC);
