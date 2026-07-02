//! Domain types for Java null-safety annotation validation.

/// The category of a package violation.
///
/// The string codes (`missing_package_info`, `missing_annotation`) are the
/// JSON-serialized `violation_type` values and must match the Go constants
/// exactly.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViolationType {
    /// Directory has `.java` files but no `package-info.java`.
    MissingPackageInfo,
    /// `package-info.java` exists but lacks the required annotation.
    MissingAnnotation,
}

impl ViolationType {
    /// Returns the Go constant string code used in JSON output.
    pub fn code(self) -> &'static str {
        match self {
            ViolationType::MissingPackageInfo => "missing_package_info",
            ViolationType::MissingAnnotation => "missing_annotation",
        }
    }
}

/// A single Java package with its validation status.
#[derive(Debug, Clone)]
pub struct PackageEntry {
    /// Relative path from the source root.
    pub package_dir: String,
    /// Whether the package passed validation.
    pub valid: bool,
    /// Only set when `valid` is false.
    pub violation_type: Option<ViolationType>,
}

/// Complete results of a null-safety validation scan.
#[derive(Debug, Clone, Default)]
pub struct ValidationResult {
    /// Total number of Java packages scanned.
    pub total_packages: usize,
    /// Number of packages that passed validation.
    pub valid_packages: usize,
    /// All packages, in sorted order.
    pub all_packages: Vec<PackageEntry>,
    /// Annotation that was required.
    pub annotation: String,
}

/// Configures how validation runs.
#[derive(Debug, Clone)]
pub struct ValidationOptions {
    /// Absolute path to the Java source root.
    pub source_root: String,
    /// Annotation name to require (e.g. `NullMarked`).
    pub annotation: String,
}
