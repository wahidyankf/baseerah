//! Domain types for contract codegen post-processing.

/// Configures the `java-clean-imports` command.
#[derive(Debug, Clone)]
pub struct JavaCleanImportsOptions {
    /// Absolute path to the generated-contracts directory.
    pub dir: String,
}

/// Results of cleaning Java imports.
#[derive(Debug, Clone, Default)]
pub struct JavaCleanImportsResult {
    /// Number of `.java` files found.
    pub total_files: usize,
    /// Number of files that were modified.
    pub modified_files: usize,
    /// Relative paths of modified files.
    pub modified: Vec<String>,
}

/// Configures the `dart-scaffold` command.
#[derive(Debug, Clone)]
pub struct DartScaffoldOptions {
    /// Absolute path to the generated-contracts directory.
    pub dir: String,
}

/// Results of Dart scaffolding.
#[derive(Debug, Clone, Default)]
pub struct DartScaffoldResult {
    /// Whether `pubspec.yaml` was written.
    pub pubspec_created: bool,
    /// Whether the barrel library was written.
    pub barrel_created: bool,
    /// Basenames of model files found.
    pub model_files: Vec<String>,
}
