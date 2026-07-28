//! Shared field-action types for Claude-agent frontmatter conversion policies.

/// How a Claude frontmatter field should be handled during conversion.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldAction {
    /// Copy to target output unchanged.
    Preserve,
    /// Transform the value before writing to target output.
    Translate,
    /// Silently discard the field.
    Drop,
    /// Discard the field and emit a conversion warning.
    DropWarn,
}

/// Per-field conversion policy entry.
pub struct FieldPolicy {
    /// What to do with this field.
    pub action: FieldAction,
    /// Human-readable reason, used in conversion warnings.
    pub reason: &'static str,
}
