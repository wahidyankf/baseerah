-- Document corpora for the regulatory-source and internal-policy contexts.
-- regulatory_documents: regulator-published rule documents with provenance.
CREATE TABLE IF NOT EXISTS regulatory_documents (
    id UUID PRIMARY KEY,
    title TEXT NOT NULL,
    issuer TEXT NOT NULL,
    jurisdiction TEXT NOT NULL,
    document_type TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- internal_policy_documents: company-internal documents with version + scope.
CREATE TABLE IF NOT EXISTS internal_policy_documents (
    id UUID PRIMARY KEY,
    title TEXT NOT NULL,
    version TEXT NOT NULL,
    scope TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
