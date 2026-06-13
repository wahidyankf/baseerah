//! Port of `apps/rhino-cli/internal/doctor/tools.go`.
//!
//! Defines [`ToolDef`] (the per-tool check configuration) and
//! [`build_tool_defs`] (the ordered list of all known tools), together with
//! their install-step factories and version readers.

use std::path::{Path, PathBuf};

use super::ToolStatus;
use super::checker::{
    compare_exact, compare_gte, compare_major, compare_major_gte, compare_playwright,
    parse_actionlint_version, parse_cargo_llvm_cov, parse_clojure_version, parse_dart_version,
    parse_docker_version, parse_dotnet_version, parse_elixir_version, parse_erlang_version,
    parse_flutter_version, parse_golangci_lint_version, parse_hadolint_version, parse_java_version,
    parse_jq_version, parse_line_word, parse_playwright_version, parse_python_version,
    parse_rust_version, parse_shellcheck_version, parse_trim_version, read_dart_sdk_version,
    read_dotnet_version, read_flutter_version, read_java_version, read_node_version,
    read_npm_version, read_python_version, read_rust_version, read_tool_versions_entry,
};

/// A single step in an auto-install sequence.
pub struct InstallStep {
    /// Short description shown to the user (e.g. `"Install Node.js 24.11.1 via Volta"`).
    pub description: String,
    /// Command to run (e.g. `"volta"`).
    pub command: String,
    /// Arguments passed to `command`.
    pub args: Vec<String>,
}

/// Function pointer that returns platform-specific install steps.
///
/// `required` is the version string from the project config; `platform` is
/// `"darwin"`, `"linux"`, or another `std::env::consts::OS` value.
/// Returns an empty `Vec` when auto-install is not supported on `platform`.
pub type InstallFunc = fn(required: &str, platform: &str) -> Vec<InstallStep>;

/// Complete specification for checking one tool.
pub struct ToolDef {
    /// Human-readable name (e.g. `"node"`).
    pub name: String,
    /// Executable name passed to the runner (e.g. `"node"`, `"go"`).
    pub binary: String,
    /// Config file that provides the required version (for display only).
    pub source: String,
    /// Arguments appended to `binary` when querying the installed version.
    pub args: Vec<String>,
    /// When `true`, version information is parsed from stderr instead of stdout.
    pub use_stderr: bool,
    /// Extracts the version string from raw command output.
    pub parse_ver: fn(&str) -> String,
    /// Compares the installed and required versions and returns a status + note.
    pub compare: fn(&str, &str) -> (ToolStatus, String),
    /// Reads the required version from the project config.
    pub read_req: fn() -> String,
    /// Optional install function; `None` means auto-install is unavailable.
    pub install_cmd: Option<InstallFunc>,
}

// --- ToolDef builders ---

/// Returns an empty string indicating no version requirement for this tool.
fn no_req() -> String {
    String::new()
}

/// Extracts the Git version from `git --version` output
/// (e.g. `"git version 2.42.0"`).
fn parse_git_version(s: &str) -> String {
    parse_line_word(s, "git version ", 2, "")
}

/// Extracts the Maven version from `mvn --version` output
/// (e.g. `"Apache Maven 3.9.9 ..."`).
fn parse_maven_version(s: &str) -> String {
    parse_line_word(s, "Apache Maven ", 2, "")
}

/// Extracts the Go version from `go version` output
/// (e.g. `"go version go1.25.0 darwin/arm64"` → `"1.25.0"`).
// Per-binary readers using a path captured in a static OnceLock.
// Go's closures capture repo_root; in Rust we precompute paths and stash them via static
// once-locks keyed off PID-stable build_tool_defs(repo_root) call.
use std::sync::OnceLock;

/// Process-wide cached collection of config-file paths derived from the repo root.
static PATHS: OnceLock<Paths> = OnceLock::new();

/// Pre-computed absolute paths to project config files used by version readers.
struct Paths {
    /// Path to the root `package.json` (for `volta.node` / `volta.npm`).
    package_json: PathBuf,
    /// Path to `apps/organiclever-be/pom.xml` (for `<java.version>`).
    pom_xml: PathBuf,
    /// Path to the Python `.python-version` file.
    python_version: PathBuf,
    /// Path to the root `.tool-versions` file (for Elixir / Erlang).
    tool_versions: PathBuf,
    /// Path to `apps/ose-app-be/global.json` (for .NET `sdk.version`).
    global_json: PathBuf,
    /// Path to the Flutter `pubspec.yaml` (for Dart SDK / Flutter versions).
    pubspec: PathBuf,
    /// Path to the Rust demo `Cargo.toml` (for `rust-version`).
    cargo_toml: PathBuf,
}

/// Initialises [`PATHS`] from `repo_root`.
///
/// The [`OnceLock`] guarantees only the first call has any effect; subsequent
/// calls with a different root are silently ignored.
fn set_paths(repo_root: &Path) {
    let p = Paths {
        package_json: repo_root.join("package.json"),
        pom_xml: repo_root
            .join("apps")
            .join("organiclever-be")
            .join("pom.xml"),
        python_version: repo_root
            .join("apps")
            .join("a-demo-be-python-fastapi")
            .join(".python-version"),
        tool_versions: repo_root.join(".tool-versions"),
        global_json: repo_root
            .join("apps")
            .join("ose-app-be")
            .join("global.json"),
        pubspec: repo_root
            .join("apps")
            .join("a-demo-fe-dart-flutterweb")
            .join("pubspec.yaml"),
        cargo_toml: repo_root
            .join("apps")
            .join("a-demo-be-rust-axum")
            .join("Cargo.toml"),
    };
    // OnceLock — only the first writer wins. For tests we reset via reset_paths.
    let _ = PATHS.set(p);
}

/// Returns a reference to the global [`Paths`] instance.
///
/// # Panics
///
/// Panics when [`set_paths`] has not been called (i.e. [`PATHS`] is still
/// uninitialised), which should never happen in normal usage because
/// [`build_tool_defs`] always calls [`set_paths`] first.
fn p() -> &'static Paths {
    PATHS.get().expect("PATHS not initialized")
}

/// Reads the `node` version from the cached `package.json`.
fn read_node_v() -> String {
    read_node_version(&p().package_json).unwrap_or_default()
}
/// Reads the `npm` version from the cached `package.json`.
fn read_npm_v() -> String {
    read_npm_version(&p().package_json).unwrap_or_default()
}
/// Reads the Java version from the cached `pom.xml`.
fn read_java_v() -> String {
    read_java_version(&p().pom_xml).unwrap_or_default()
}
/// Reads the Python version from the cached `.python-version` file.
fn read_python_v() -> String {
    read_python_version(&p().python_version).unwrap_or_default()
}
/// Reads the .NET SDK version from the cached `global.json`.
fn read_dotnet_v() -> String {
    read_dotnet_version(&p().global_json).unwrap_or_default()
}
/// Reads the Dart SDK minimum version from the cached `pubspec.yaml`.
fn read_dart_v() -> String {
    read_dart_sdk_version(&p().pubspec).unwrap_or_default()
}
/// Reads the `rust-version` (MSRV) from the cached `Cargo.toml`.
fn read_rust_v() -> String {
    read_rust_version(&p().cargo_toml).unwrap_or_default()
}
/// Reads the Flutter minimum version from the cached `pubspec.yaml`.
fn read_flutter_v() -> String {
    read_flutter_version(&p().pubspec).unwrap_or_default()
}
/// Reads the Elixir version from the cached `.tool-versions`, stripping the
/// `-otp-XX` suffix (e.g. `"1.19.5-otp-27"` → `"1.19.5"`).
fn read_elixir_v() -> String {
    let v = read_tool_versions_entry(&p().tool_versions, "elixir").unwrap_or_default();
    // Strip -otp-XX suffix: "1.19.5-otp-27" → "1.19.5"
    if let Some(idx) = v.find("-otp-") {
        return v[..idx].to_string();
    }
    v
}
/// Reads the Erlang OTP version from the cached `.tool-versions`.
fn read_erlang_v() -> String {
    read_tool_versions_entry(&p().tool_versions, "erlang").unwrap_or_default()
}
/// Returns the pinned `golangci-lint` version required by this project.
fn read_golangci_v() -> String {
    "2.11.3".into()
}

// --- Install commands ---

/// Returns install steps for `git`.
///
/// On macOS: `xcode-select --install`.
/// On Linux: `sudo apt-get install -y git`.
fn install_git(_req: &str, platform: &str) -> Vec<InstallStep> {
    if platform == "darwin" {
        vec![InstallStep {
            description: "Install Xcode Command Line Tools".into(),
            command: "xcode-select".into(),
            args: vec!["--install".into()],
        }]
    } else {
        vec![InstallStep {
            description: "Install git".into(),
            command: "sudo".into(),
            args: vec![
                "apt-get".into(),
                "install".into(),
                "-y".into(),
                "git".into(),
            ],
        }]
    }
}

/// Returns install steps for Volta (the Node.js version manager).
fn install_volta(_req: &str, _platform: &str) -> Vec<InstallStep> {
    vec![InstallStep {
        description: "Install Volta".into(),
        command: "bash".into(),
        args: vec!["-c".into(), "curl https://get.volta.sh | bash".into()],
    }]
}

/// Returns install steps for Node.js via `volta install node@<req>`.
fn install_node(req: &str, _platform: &str) -> Vec<InstallStep> {
    vec![InstallStep {
        description: format!("Install Node.js {req} via Volta"),
        command: "volta".into(),
        args: vec!["install".into(), format!("node@{req}")],
    }]
}

/// Returns install steps for npm via `volta install npm@<req>`.
fn install_npm(req: &str, _platform: &str) -> Vec<InstallStep> {
    vec![InstallStep {
        description: format!("Install npm {req} via Volta"),
        command: "volta".into(),
        args: vec!["install".into(), format!("npm@{req}")],
    }]
}

/// Returns install steps for Java via SDKMAN (`sdk install java <req>-tem`).
fn install_java(req: &str, _platform: &str) -> Vec<InstallStep> {
    vec![InstallStep {
        description: format!("Install Java {req} via SDKMAN"),
        command: "bash".into(),
        args: vec![
            "-c".into(),
            format!("source \"$HOME/.sdkman/bin/sdkman-init.sh\" && sdk install java {req}-tem"),
        ],
    }]
}

/// Returns install steps for Maven via SDKMAN (`sdk install maven`).
fn install_maven(_req: &str, _platform: &str) -> Vec<InstallStep> {
    vec![InstallStep {
        description: "Install Maven via SDKMAN".into(),
        command: "bash".into(),
        args: vec![
            "-c".into(),
            "source \"$HOME/.sdkman/bin/sdkman-init.sh\" && sdk install maven".into(),
        ],
    }]
}

/// Returns install steps for Go.
///
/// Returns install steps for Python via pyenv.
fn install_python(req: &str, platform: &str) -> Vec<InstallStep> {
    if platform == "darwin" {
        vec![
            InstallStep {
                description: "Install pyenv via Homebrew".into(),
                command: "brew".into(),
                args: vec!["install".into(), "pyenv".into()],
            },
            InstallStep {
                description: format!("Install Python {req}"),
                command: "bash".into(),
                args: vec![
                    "-c".into(),
                    format!("pyenv install {req} && pyenv global {req}"),
                ],
            },
        ]
    } else {
        vec![
            InstallStep {
                description: "Install pyenv".into(),
                command: "bash".into(),
                args: vec!["-c".into(), "curl https://pyenv.run | bash".into()],
            },
            InstallStep {
                description: format!("Install Python {req}"),
                command: "bash".into(),
                args: vec![
                    "-c".into(),
                    format!("pyenv install {req} && pyenv global {req}"),
                ],
            },
        ]
    }
}

/// Returns install steps for Rust via `rustup`.
fn install_rust(_req: &str, _platform: &str) -> Vec<InstallStep> {
    vec![InstallStep {
        description: "Install Rust via rustup".into(),
        command: "bash".into(),
        args: vec![
            "-c".into(),
            "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y".into(),
        ],
    }]
}

/// Returns install steps for `cargo-llvm-cov` via `cargo install`.
fn install_cargo_llvm_cov(_req: &str, _platform: &str) -> Vec<InstallStep> {
    vec![InstallStep {
        description: "Install cargo-llvm-cov".into(),
        command: "bash".into(),
        args: vec![
            "-c".into(),
            "source \"$HOME/.cargo/env\" && cargo install cargo-llvm-cov".into(),
        ],
    }]
}

/// Returns install steps for Elixir via asdf.
fn install_elixir(req: &str, _platform: &str) -> Vec<InstallStep> {
    vec![InstallStep {
        description: format!("Install Elixir {req} via asdf"),
        command: "bash".into(),
        args: vec![
            "-c".into(),
            format!(
                "asdf plugin add elixir 2>/dev/null; asdf install elixir {req} && asdf global elixir {req}"
            ),
        ],
    }]
}

/// Returns install steps for Erlang via asdf.
fn install_erlang(req: &str, _platform: &str) -> Vec<InstallStep> {
    vec![InstallStep {
        description: format!("Install Erlang {req} via asdf"),
        command: "bash".into(),
        args: vec![
            "-c".into(),
            format!(
                "asdf plugin add erlang 2>/dev/null; asdf install erlang {req} && asdf global erlang {req}"
            ),
        ],
    }]
}

/// Returns install steps for .NET SDK.
///
/// On macOS: `brew install dotnet`.
/// On Linux: `sudo snap install dotnet-sdk --classic --channel=10.0`.
fn install_dotnet(_req: &str, platform: &str) -> Vec<InstallStep> {
    if platform == "darwin" {
        vec![InstallStep {
            description: "Install .NET via Homebrew".into(),
            command: "brew".into(),
            args: vec!["install".into(), "dotnet".into()],
        }]
    } else {
        vec![InstallStep {
            description: "Install .NET via snap".into(),
            command: "sudo".into(),
            args: vec![
                "snap".into(),
                "install".into(),
                "dotnet-sdk".into(),
                "--classic".into(),
                "--channel=10.0".into(),
            ],
        }]
    }
}

/// Returns install steps for Clojure CLI.
///
/// On macOS: `brew install clojure/tools/clojure`.
/// On Linux: downloads and runs the official install script.
fn install_clojure(_req: &str, platform: &str) -> Vec<InstallStep> {
    if platform == "darwin" {
        vec![InstallStep {
            description: "Install Clojure via Homebrew".into(),
            command: "brew".into(),
            args: vec!["install".into(), "clojure/tools/clojure".into()],
        }]
    } else {
        vec![InstallStep {
            description: "Install Clojure CLI".into(),
            command: "bash".into(),
            args: vec![
                "-c".into(),
                "curl -L -O https://github.com/clojure/brew-install/releases/latest/download/linux-install.sh && chmod +x linux-install.sh && sudo ./linux-install.sh && rm linux-install.sh".into(),
            ],
        }]
    }
}

/// Returns install steps for Flutter.
///
/// On macOS: `brew install --cask flutter`.
/// On Linux: `sudo snap install flutter --classic`.
fn install_flutter(_req: &str, platform: &str) -> Vec<InstallStep> {
    if platform == "darwin" {
        vec![InstallStep {
            description: "Install Flutter via Homebrew".into(),
            command: "brew".into(),
            args: vec!["install".into(), "--cask".into(), "flutter".into()],
        }]
    } else {
        vec![InstallStep {
            description: "Install Flutter via snap".into(),
            command: "sudo".into(),
            args: vec![
                "snap".into(),
                "install".into(),
                "flutter".into(),
                "--classic".into(),
            ],
        }]
    }
}

/// Returns install steps for Docker.
///
/// On macOS: returns an empty `Vec` (Docker Desktop must be installed manually).
/// On Linux: `sudo apt-get install -y docker.io docker-compose-v2`.
fn install_docker(_req: &str, platform: &str) -> Vec<InstallStep> {
    if platform == "darwin" {
        // Docker Desktop must be installed manually on macOS.
        Vec::new()
    } else {
        vec![InstallStep {
            description: "Install Docker".into(),
            command: "sudo".into(),
            args: vec![
                "apt-get".into(),
                "install".into(),
                "-y".into(),
                "docker.io".into(),
                "docker-compose-v2".into(),
            ],
        }]
    }
}

/// Returns install steps for `jq`.
///
/// On macOS: `brew install jq`.
/// On Linux: `sudo apt-get install -y jq`.
fn install_jq(_req: &str, platform: &str) -> Vec<InstallStep> {
    if platform == "darwin" {
        vec![InstallStep {
            description: "Install jq via Homebrew".into(),
            command: "brew".into(),
            args: vec!["install".into(), "jq".into()],
        }]
    } else {
        vec![InstallStep {
            description: "Install jq".into(),
            command: "sudo".into(),
            args: vec!["apt-get".into(), "install".into(), "-y".into(), "jq".into()],
        }]
    }
}

/// Returns install steps for `shellcheck` (Homebrew on macOS, apt otherwise).
fn install_shellcheck(_req: &str, platform: &str) -> Vec<InstallStep> {
    if platform == "darwin" {
        vec![InstallStep {
            description: "Install shellcheck via Homebrew".into(),
            command: "brew".into(),
            args: vec!["install".into(), "shellcheck".into()],
        }]
    } else {
        vec![InstallStep {
            description: "Install shellcheck".into(),
            command: "sudo".into(),
            args: vec![
                "apt-get".into(),
                "install".into(),
                "-y".into(),
                "shellcheck".into(),
            ],
        }]
    }
}

/// Returns install steps for `actionlint` (Homebrew on macOS; pinned download
/// script on Linux, where no apt package is published).
fn install_actionlint(_req: &str, platform: &str) -> Vec<InstallStep> {
    if platform == "darwin" {
        vec![InstallStep {
            description: "Install actionlint via Homebrew".into(),
            command: "brew".into(),
            args: vec!["install".into(), "actionlint".into()],
        }]
    } else {
        vec![InstallStep {
            description: "Install actionlint via the official download script".into(),
            command: "sudo".into(),
            args: vec![
                "bash".into(),
                "-c".into(),
                "curl -sSL https://raw.githubusercontent.com/rhysd/actionlint/v1.7.12/scripts/download-actionlint.bash | bash -s -- 1.7.12 /usr/local/bin".into(),
            ],
        }]
    }
}

/// Returns install steps for `hadolint` (Homebrew on macOS; pinned binary
/// download on Linux, where no apt package is published).
fn install_hadolint(_req: &str, platform: &str) -> Vec<InstallStep> {
    if platform == "darwin" {
        vec![InstallStep {
            description: "Install hadolint via Homebrew".into(),
            command: "brew".into(),
            args: vec!["install".into(), "hadolint".into()],
        }]
    } else {
        vec![
            InstallStep {
                description: "Download hadolint binary".into(),
                command: "sudo".into(),
                args: vec![
                    "curl".into(),
                    "-sSL".into(),
                    "-o".into(),
                    "/usr/local/bin/hadolint".into(),
                    "https://github.com/hadolint/hadolint/releases/download/v2.14.0/hadolint-Linux-x86_64".into(),
                ],
            },
            InstallStep {
                description: "Make hadolint executable".into(),
                command: "sudo".into(),
                args: vec!["chmod".into(), "+x".into(), "/usr/local/bin/hadolint".into()],
            },
        ]
    }
}

/// Returns install steps for `golangci-lint` via `go install`.
fn install_golangci_lint(req: &str, _platform: &str) -> Vec<InstallStep> {
    vec![InstallStep {
        description: format!("Install golangci-lint v{req} via go install"),
        command: "bash".into(),
        args: vec![
            "-c".into(),
            format!("go install github.com/golangci/golangci-lint/cmd/golangci-lint@v{req}"),
        ],
    }]
}

/// Returns install steps for Playwright browsers.
///
/// On macOS: `npx playwright install`.
/// On Linux: `npx playwright install` followed by `npx playwright install-deps`.
fn install_playwright(_req: &str, platform: &str) -> Vec<InstallStep> {
    if platform == "darwin" {
        vec![InstallStep {
            description: "Install Playwright browsers".into(),
            command: "npx".into(),
            args: vec!["playwright".into(), "install".into()],
        }]
    } else {
        vec![
            InstallStep {
                description: "Install Playwright browsers".into(),
                command: "npx".into(),
                args: vec!["playwright".into(), "install".into()],
            },
            InstallStep {
                description: "Install Playwright system deps".into(),
                command: "npx".into(),
                args: vec!["playwright".into(), "install-deps".into()],
            },
        ]
    }
}

/// Build the ordered list of tool defs for the given repo root.
pub fn build_tool_defs(repo_root: &Path) -> Vec<ToolDef> {
    // PATHS is a OnceLock — only set once per process. Tests use isolated runners.
    set_paths(repo_root);
    let mut defs = tool_defs_core();
    defs.extend(tool_defs_jvm_and_go());
    defs.extend(tool_defs_scripting_and_beam());
    defs.extend(tool_defs_dotnet_and_mobile());
    defs.extend(tool_defs_infra());
    defs
}

/// Returns the core tool definitions: `git`, `volta`, `node`, `npm`.
fn tool_defs_core() -> Vec<ToolDef> {
    vec![
        ToolDef {
            name: "git".into(),
            binary: "git".into(),
            source: "(no config file)".into(),
            args: vec!["--version".into()],
            use_stderr: false,
            parse_ver: parse_git_version,
            compare: compare_exact,
            read_req: no_req,
            install_cmd: Some(install_git),
        },
        ToolDef {
            name: "volta".into(),
            binary: "volta".into(),
            source: "(no config file)".into(),
            args: vec!["--version".into()],
            use_stderr: false,
            parse_ver: parse_trim_version,
            compare: compare_exact,
            read_req: no_req,
            install_cmd: Some(install_volta),
        },
        ToolDef {
            name: "node".into(),
            binary: "node".into(),
            source: "package.json → volta.node".into(),
            args: vec!["--version".into()],
            use_stderr: false,
            parse_ver: parse_trim_version,
            compare: compare_exact,
            read_req: read_node_v,
            install_cmd: Some(install_node),
        },
        ToolDef {
            name: "npm".into(),
            binary: "npm".into(),
            source: "package.json → volta.npm".into(),
            args: vec!["--version".into()],
            use_stderr: false,
            parse_ver: parse_trim_version,
            compare: compare_exact,
            read_req: read_npm_v,
            install_cmd: Some(install_npm),
        },
    ]
}

/// Returns tool definitions for JVM and Go: `java`, `maven`, `golang`.
fn tool_defs_jvm_and_go() -> Vec<ToolDef> {
    vec![
        ToolDef {
            name: "java".into(),
            binary: "java".into(),
            source: "apps/organiclever-be/pom.xml → <java.version>".into(),
            args: vec!["-version".into()],
            use_stderr: true,
            parse_ver: parse_java_version,
            compare: compare_major,
            read_req: read_java_v,
            install_cmd: Some(install_java),
        },
        ToolDef {
            name: "maven".into(),
            binary: "mvn".into(),
            source: "(no config file)".into(),
            args: vec!["--version".into()],
            use_stderr: false,
            parse_ver: parse_maven_version,
            compare: compare_exact,
            read_req: no_req,
            install_cmd: Some(install_maven),
        },
    ]
}

/// Returns tool definitions for scripting and BEAM: `python`, `rust`, `cargo-llvm-cov`,
/// `elixir`, `erlang`.
fn tool_defs_scripting_and_beam() -> Vec<ToolDef> {
    vec![
        ToolDef {
            name: "python".into(),
            binary: "python3".into(),
            source: "(demo extracted to ose-primer — no local requirement)".into(),
            args: vec!["--version".into()],
            use_stderr: false,
            parse_ver: parse_python_version,
            compare: compare_gte,
            read_req: read_python_v,
            install_cmd: Some(install_python),
        },
        ToolDef {
            name: "rust".into(),
            binary: "rustc".into(),
            source: "(demo extracted to ose-primer — no local requirement)".into(),
            args: vec!["--version".into()],
            use_stderr: false,
            parse_ver: parse_rust_version,
            compare: compare_gte,
            read_req: read_rust_v,
            install_cmd: Some(install_rust),
        },
        ToolDef {
            name: "cargo-llvm-cov".into(),
            binary: "cargo".into(),
            source: "(no config file)".into(),
            args: vec!["llvm-cov".into(), "--version".into()],
            use_stderr: false,
            parse_ver: parse_cargo_llvm_cov,
            compare: compare_exact,
            read_req: no_req,
            install_cmd: Some(install_cargo_llvm_cov),
        },
        ToolDef {
            name: "elixir".into(),
            binary: "elixir".into(),
            source: ".tool-versions → elixir".into(),
            args: vec!["--version".into()],
            use_stderr: false,
            parse_ver: parse_elixir_version,
            compare: compare_gte,
            read_req: read_elixir_v,
            install_cmd: Some(install_elixir),
        },
        ToolDef {
            name: "erlang".into(),
            binary: "erl".into(),
            source: ".tool-versions → erlang".into(),
            args: vec![
                "-noshell".into(),
                "-eval".into(),
                "io:format(\"~s\",[erlang:system_info(otp_release)]),halt().".into(),
            ],
            use_stderr: false,
            parse_ver: parse_erlang_version,
            compare: compare_major_gte,
            read_req: read_erlang_v,
            install_cmd: Some(install_erlang),
        },
    ]
}

/// Returns tool definitions for .NET and mobile: `dotnet`, `clojure`, `dart`, `flutter`.
fn tool_defs_dotnet_and_mobile() -> Vec<ToolDef> {
    vec![
        ToolDef {
            name: "dotnet".into(),
            binary: "dotnet".into(),
            source: "apps/ose-app-be/global.json → sdk.version".into(),
            args: vec!["--version".into()],
            use_stderr: false,
            parse_ver: parse_dotnet_version,
            compare: compare_major_gte,
            read_req: read_dotnet_v,
            install_cmd: Some(install_dotnet),
        },
        ToolDef {
            name: "clojure".into(),
            binary: "clj".into(),
            source: "(no config file)".into(),
            args: vec!["--version".into()],
            use_stderr: false,
            parse_ver: parse_clojure_version,
            compare: compare_exact,
            read_req: no_req,
            install_cmd: Some(install_clojure),
        },
        ToolDef {
            name: "dart".into(),
            binary: "dart".into(),
            source: "(demo extracted to ose-primer — no local requirement)".into(),
            args: vec!["--version".into()],
            use_stderr: false,
            parse_ver: parse_dart_version,
            compare: compare_gte,
            read_req: read_dart_v,
            install_cmd: None,
        },
        ToolDef {
            name: "flutter".into(),
            binary: "flutter".into(),
            source: "(demo extracted to ose-primer — no local requirement)".into(),
            args: vec!["--version".into()],
            use_stderr: false,
            parse_ver: parse_flutter_version,
            compare: compare_gte,
            read_req: read_flutter_v,
            install_cmd: Some(install_flutter),
        },
    ]
}

/// Returns tool definitions for infrastructure: `docker`, `jq`, `golangci-lint`,
/// `shellcheck`, `hadolint`, `actionlint`, `playwright`.
fn tool_defs_infra() -> Vec<ToolDef> {
    vec![
        ToolDef {
            name: "docker".into(),
            binary: "docker".into(),
            source: "(no config file)".into(),
            args: vec!["--version".into()],
            use_stderr: false,
            parse_ver: parse_docker_version,
            compare: compare_exact,
            read_req: no_req,
            install_cmd: Some(install_docker),
        },
        ToolDef {
            name: "jq".into(),
            binary: "jq".into(),
            source: "(no config file)".into(),
            args: vec!["--version".into()],
            use_stderr: false,
            parse_ver: parse_jq_version,
            compare: compare_exact,
            read_req: no_req,
            install_cmd: Some(install_jq),
        },
        ToolDef {
            name: "golangci-lint".into(),
            binary: "golangci-lint".into(),
            source: "(pinned: v2.11.3)".into(),
            args: vec!["version".into()],
            use_stderr: false,
            parse_ver: parse_golangci_lint_version,
            compare: compare_gte,
            read_req: read_golangci_v,
            install_cmd: Some(install_golangci_lint),
        },
        ToolDef {
            name: "shellcheck".into(),
            binary: "shellcheck".into(),
            source: "(no config file)".into(),
            args: vec!["--version".into()],
            use_stderr: false,
            parse_ver: parse_shellcheck_version,
            compare: compare_exact,
            read_req: no_req,
            install_cmd: Some(install_shellcheck),
        },
        ToolDef {
            name: "hadolint".into(),
            binary: "hadolint".into(),
            source: "(no config file)".into(),
            args: vec!["--version".into()],
            use_stderr: false,
            parse_ver: parse_hadolint_version,
            compare: compare_exact,
            read_req: no_req,
            install_cmd: Some(install_hadolint),
        },
        ToolDef {
            name: "actionlint".into(),
            binary: "actionlint".into(),
            source: "(no config file)".into(),
            args: vec!["--version".into()],
            use_stderr: false,
            parse_ver: parse_actionlint_version,
            compare: compare_exact,
            read_req: no_req,
            install_cmd: Some(install_actionlint),
        },
        ToolDef {
            name: "playwright".into(),
            binary: "npx".into(),
            source: "node_modules (npx playwright)".into(),
            args: vec!["playwright".into(), "--version".into()],
            use_stderr: false,
            parse_ver: parse_playwright_version,
            compare: compare_playwright,
            read_req: no_req,
            install_cmd: Some(install_playwright),
        },
    ]
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    #[test]
    fn build_returns_all_known_tools() {
        let dir = tempfile::tempdir().unwrap();
        let defs = build_tool_defs(dir.path());
        assert_eq!(defs.len(), 22);
        assert_eq!(defs[0].name, "git");
        assert_eq!(defs.last().unwrap().name, "playwright");
        assert!(defs.iter().any(|d| d.name == "shellcheck"));
        assert!(defs.iter().any(|d| d.name == "hadolint"));
        assert!(defs.iter().any(|d| d.name == "actionlint"));
    }

    #[test]
    fn install_git_macos() {
        let steps = install_git("", "darwin");
        assert_eq!(steps[0].command, "xcode-select");
    }

    #[test]
    fn install_git_linux() {
        let steps = install_git("", "linux");
        assert_eq!(steps[0].command, "sudo");
        assert!(steps[0].args.contains(&"git".to_string()));
    }

    #[test]
    fn install_docker_macos_empty() {
        assert!(install_docker("", "darwin").is_empty());
    }

    #[test]
    fn install_node_formats_required() {
        let s = install_node("24.11.1", "darwin");
        assert_eq!(s[0].args[1], "node@24.11.1");
    }

    #[test]
    fn install_java_formats_sdk_cmd() {
        let s = install_java("21", "darwin");
        assert!(s[0].args[1].contains("sdk install java 21-tem"));
    }
}
