// Port of `apps/rhino-cli/internal/doctor/tools.go`.

use std::path::{Path, PathBuf};

use super::ToolStatus;
use super::checker::{
    compare_exact, compare_gte, compare_major, compare_major_gte, compare_playwright,
    parse_cargo_llvm_cov, parse_clojure_version, parse_dart_version, parse_docker_version,
    parse_dotnet_version, parse_elixir_version, parse_erlang_version, parse_flutter_version,
    parse_golangci_lint_version, parse_java_version, parse_jq_version, parse_line_word,
    parse_playwright_version, parse_python_version, parse_rust_version, parse_trim_version,
    read_dart_sdk_version, read_dotnet_version, read_flutter_version, read_go_version,
    read_java_version, read_node_version, read_npm_version, read_python_version, read_rust_version,
    read_tool_versions_entry,
};

/// Single installation step.
pub struct InstallStep {
    pub description: String,
    pub command: String,
    pub args: Vec<String>,
}

/// Closure that returns install steps for `(required, platform)`.
/// Returns empty when the tool cannot be auto-installed on that platform.
pub type InstallFunc = fn(required: &str, platform: &str) -> Vec<InstallStep>;

/// Tool check definition.
pub struct ToolDef {
    pub name: String,
    pub binary: String,
    pub source: String,
    pub args: Vec<String>,
    pub use_stderr: bool,
    pub parse_ver: fn(&str) -> String,
    pub compare: fn(&str, &str) -> (ToolStatus, String),
    pub read_req: fn() -> String,
    pub install_cmd: Option<InstallFunc>,
}

// --- ToolDef builders ---

fn no_req() -> String {
    String::new()
}

fn parse_git_version(s: &str) -> String {
    parse_line_word(s, "git version ", 2, "")
}

fn parse_maven_version(s: &str) -> String {
    parse_line_word(s, "Apache Maven ", 2, "")
}

fn parse_go_version(s: &str) -> String {
    parse_line_word(s, "go version ", 2, "go")
}

// Per-binary readers using a path captured in a static OnceLock.
// Go's closures capture repo_root; in Rust we precompute paths and stash them via static
// once-locks keyed off PID-stable build_tool_defs(repo_root) call.
use std::sync::OnceLock;

static PATHS: OnceLock<Paths> = OnceLock::new();

struct Paths {
    package_json: PathBuf,
    pom_xml: PathBuf,
    go_mod: PathBuf,
    python_version: PathBuf,
    tool_versions: PathBuf,
    global_json: PathBuf,
    pubspec: PathBuf,
    cargo_toml: PathBuf,
}

fn set_paths(repo_root: &Path) {
    let p = Paths {
        package_json: repo_root.join("package.json"),
        pom_xml: repo_root
            .join("apps")
            .join("organiclever-be")
            .join("pom.xml"),
        go_mod: repo_root.join("apps").join("rhino-cli").join("go.mod"),
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

fn p() -> &'static Paths {
    PATHS.get().expect("PATHS not initialized")
}

fn read_node_v() -> String {
    read_node_version(&p().package_json).unwrap_or_default()
}
fn read_npm_v() -> String {
    read_npm_version(&p().package_json).unwrap_or_default()
}
fn read_java_v() -> String {
    read_java_version(&p().pom_xml).unwrap_or_default()
}
fn read_go_v() -> String {
    read_go_version(&p().go_mod).unwrap_or_default()
}
fn read_python_v() -> String {
    read_python_version(&p().python_version).unwrap_or_default()
}
fn read_dotnet_v() -> String {
    read_dotnet_version(&p().global_json).unwrap_or_default()
}
fn read_dart_v() -> String {
    read_dart_sdk_version(&p().pubspec).unwrap_or_default()
}
fn read_rust_v() -> String {
    read_rust_version(&p().cargo_toml).unwrap_or_default()
}
fn read_flutter_v() -> String {
    read_flutter_version(&p().pubspec).unwrap_or_default()
}
fn read_elixir_v() -> String {
    let v = read_tool_versions_entry(&p().tool_versions, "elixir").unwrap_or_default();
    // Strip -otp-XX suffix: "1.19.5-otp-27" → "1.19.5"
    if let Some(idx) = v.find("-otp-") {
        return v[..idx].to_string();
    }
    v
}
fn read_erlang_v() -> String {
    read_tool_versions_entry(&p().tool_versions, "erlang").unwrap_or_default()
}
fn read_golangci_v() -> String {
    "2.11.3".into()
}

// --- Install commands ---

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

fn install_volta(_req: &str, _platform: &str) -> Vec<InstallStep> {
    vec![InstallStep {
        description: "Install Volta".into(),
        command: "bash".into(),
        args: vec!["-c".into(), "curl https://get.volta.sh | bash".into()],
    }]
}

fn install_node(req: &str, _platform: &str) -> Vec<InstallStep> {
    vec![InstallStep {
        description: format!("Install Node.js {req} via Volta"),
        command: "volta".into(),
        args: vec!["install".into(), format!("node@{req}")],
    }]
}

fn install_npm(req: &str, _platform: &str) -> Vec<InstallStep> {
    vec![InstallStep {
        description: format!("Install npm {req} via Volta"),
        command: "volta".into(),
        args: vec!["install".into(), format!("npm@{req}")],
    }]
}

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

fn install_golang(req: &str, platform: &str) -> Vec<InstallStep> {
    if platform == "darwin" {
        vec![InstallStep {
            description: "Install Go via Homebrew".into(),
            command: "brew".into(),
            args: vec!["install".into(), "go".into()],
        }]
    } else {
        vec![InstallStep {
            description: "Install Go from go.dev".into(),
            command: "bash".into(),
            args: vec![
                "-c".into(),
                format!(
                    "curl -L https://go.dev/dl/go{req}.linux-amd64.tar.gz | sudo tar -xz -C /usr/local"
                ),
            ],
        }]
    }
}

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
        ToolDef {
            name: "golang".into(),
            binary: "go".into(),
            source: "apps/rhino-cli/go.mod → go directive".into(),
            args: vec!["version".into()],
            use_stderr: false,
            parse_ver: parse_go_version,
            compare: compare_gte,
            read_req: read_go_v,
            install_cmd: Some(install_golang),
        },
    ]
}

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
    fn build_returns_twenty_tools() {
        let dir = tempfile::tempdir().unwrap();
        let defs = build_tool_defs(dir.path());
        assert_eq!(defs.len(), 20);
        assert_eq!(defs[0].name, "git");
        assert_eq!(defs.last().unwrap().name, "playwright");
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
