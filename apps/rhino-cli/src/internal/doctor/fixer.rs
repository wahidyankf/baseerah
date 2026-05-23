// Port of `apps/rhino-cli/internal/doctor/fixer.go`.

use std::process::Command;

use super::tools::{build_tool_defs, InstallStep, ToolDef};
use super::{is_minimal_tool, CheckOptions, DoctorResult, Scope, ToolStatus};

/// Run a fix command. Return Err on failure.
pub type FixRunnerFunc<'a> = &'a dyn Fn(&str, &[&str]) -> Result<(), String>;

/// Fix invocation options.
#[derive(Default)]
pub struct FixOptions<'a> {
    pub dry_run: bool,
    pub runner: Option<FixRunnerFunc<'a>>,
}

/// Fix attempt summary.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FixResult {
    pub fixed: usize,
    pub failed: usize,
    pub already_ok: usize,
    pub skipped: usize,
}

fn real_fix_runner(command: &str, args: &[&str]) -> Result<(), String> {
    let status = Command::new(command)
        .args(args)
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .status()
        .map_err(|e| e.to_string())?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("exit {}", status.code().unwrap_or(-1)))
    }
}

fn current_platform() -> &'static str {
    if cfg!(target_os = "macos") {
        "darwin"
    } else if cfg!(target_os = "linux") {
        "linux"
    } else {
        std::env::consts::OS
    }
}

/// Attempt to install missing tools.
pub fn fix<F>(
    result: &DoctorResult,
    defs: &[ToolDef],
    opts: &FixOptions<'_>,
    mut printf: F,
) -> FixResult
where
    F: FnMut(&str),
{
    let platform = current_platform();
    let runner: FixRunnerFunc<'_> = opts.runner.unwrap_or(&real_fix_runner);
    let mut fr = FixResult::default();

    for (i, check) in result.checks.iter().enumerate() {
        if check.status == ToolStatus::Ok || check.status == ToolStatus::Warning {
            fr.already_ok += 1;
            continue;
        }
        // Missing
        if i >= defs.len() || defs[i].install_cmd.is_none() {
            printf(&format!(
                "Skip: {} — no auto-install available\n",
                check.name
            ));
            fr.skipped += 1;
            continue;
        }
        let install_fn = defs[i].install_cmd.unwrap();
        let steps: Vec<InstallStep> = install_fn(&check.required_version, platform);
        if steps.is_empty() {
            printf(&format!(
                "Skip: {} — no install steps for platform {}\n",
                check.name, platform
            ));
            fr.skipped += 1;
            continue;
        }

        let mut tool_failed = false;
        for step in &steps {
            if opts.dry_run {
                printf(&format!(
                    "Would install: {} via {} {}\n",
                    check.name,
                    step.command,
                    step.args.join(" ")
                ));
                continue;
            }
            printf(&format!(
                "Installing {}: {}\n",
                check.name, step.description
            ));
            let arg_refs: Vec<&str> = step.args.iter().map(|s| s.as_str()).collect();
            if let Err(e) = runner(&step.command, &arg_refs) {
                printf(&format!("  Failed: {e}\n"));
                fr.failed += 1;
                tool_failed = true;
                break;
            }
        }
        if !opts.dry_run && !tool_failed {
            fr.fixed += 1;
        }
    }

    fr
}

/// Rebuild defs from check options then call fix.
pub fn fix_all<F>(
    result: &DoctorResult,
    opts: &CheckOptions<'_>,
    fix_opts: &FixOptions<'_>,
    printf: F,
) -> FixResult
where
    F: FnMut(&str),
{
    let mut defs = build_tool_defs(&opts.repo_root);
    if opts.scope == Scope::Minimal {
        defs.retain(|d| is_minimal_tool(&d.name));
    }
    fix(result, &defs, fix_opts, printf)
}

/// One-line summary.
pub fn format_fix_summary(fr: &FixResult) -> String {
    format!(
        "\nFix summary: {} fixed, {} failed, {} already OK\n",
        fr.fixed, fr.failed, fr.already_ok
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::internal::doctor::ToolCheck;
    use std::cell::RefCell;
    use std::time::Duration;

    fn miss(name: &str) -> ToolCheck {
        ToolCheck {
            name: name.into(),
            binary: name.into(),
            status: ToolStatus::Missing,
            installed_version: "".into(),
            required_version: "".into(),
            source: "".into(),
            note: "not found in PATH".into(),
        }
    }

    fn ok(name: &str) -> ToolCheck {
        ToolCheck {
            name: name.into(),
            binary: name.into(),
            status: ToolStatus::Ok,
            installed_version: "1".into(),
            required_version: "".into(),
            source: "".into(),
            note: "no version requirement".into(),
        }
    }

    fn def(name: &str, install: Option<crate::internal::doctor::tools::InstallFunc>) -> ToolDef {
        ToolDef {
            name: name.into(),
            binary: name.into(),
            source: "".into(),
            args: vec![],
            use_stderr: false,
            parse_ver: |s| s.into(),
            compare: |_, _| (ToolStatus::Ok, "".into()),
            read_req: || "".into(),
            install_cmd: install,
        }
    }

    fn install_echo(_req: &str, _platform: &str) -> Vec<InstallStep> {
        vec![InstallStep {
            description: "echo".into(),
            command: "/bin/echo".into(),
            args: vec!["x".into()],
        }]
    }

    #[test]
    fn already_ok_counted() {
        let res = DoctorResult {
            checks: vec![ok("a")],
            ok_count: 1,
            warn_count: 0,
            missing_count: 0,
            duration: Duration::ZERO,
            scope: Scope::Full,
        };
        let defs = vec![def("a", None)];
        let mut buf = String::new();
        let fr = fix(&res, &defs, &FixOptions::default(), |m| buf.push_str(m));
        assert_eq!(fr.already_ok, 1);
        assert_eq!(fr.fixed, 0);
    }

    #[test]
    fn missing_no_install_skipped() {
        let res = DoctorResult {
            checks: vec![miss("a")],
            ok_count: 0,
            warn_count: 0,
            missing_count: 1,
            duration: Duration::ZERO,
            scope: Scope::Full,
        };
        let defs = vec![def("a", None)];
        let mut log = String::new();
        let fr = fix(&res, &defs, &FixOptions::default(), |m| log.push_str(m));
        assert_eq!(fr.skipped, 1);
        assert!(log.contains("no auto-install"));
    }

    #[test]
    fn missing_empty_steps_skipped() {
        let res = DoctorResult {
            checks: vec![miss("a")],
            ok_count: 0,
            warn_count: 0,
            missing_count: 1,
            duration: Duration::ZERO,
            scope: Scope::Full,
        };
        fn empty(_req: &str, _platform: &str) -> Vec<InstallStep> {
            Vec::new()
        }
        let defs = vec![def("a", Some(empty))];
        let mut log = String::new();
        let fr = fix(&res, &defs, &FixOptions::default(), |m| log.push_str(m));
        assert_eq!(fr.skipped, 1);
        assert!(log.contains("no install steps"));
    }

    #[test]
    fn dry_run_does_not_invoke_runner() {
        let res = DoctorResult {
            checks: vec![miss("a")],
            ok_count: 0,
            warn_count: 0,
            missing_count: 1,
            duration: Duration::ZERO,
            scope: Scope::Full,
        };
        let defs = vec![def("a", Some(install_echo))];
        let invoked = RefCell::new(0usize);
        let runner = |_cmd: &str, _args: &[&str]| -> Result<(), String> {
            *invoked.borrow_mut() += 1;
            Ok(())
        };
        let mut log = String::new();
        let fr = fix(
            &res,
            &defs,
            &FixOptions {
                dry_run: true,
                runner: Some(&runner),
            },
            |m| log.push_str(m),
        );
        assert_eq!(*invoked.borrow(), 0);
        assert_eq!(fr.fixed, 0);
        assert!(log.contains("Would install"));
    }

    #[test]
    fn live_runner_success() {
        let res = DoctorResult {
            checks: vec![miss("a")],
            ok_count: 0,
            warn_count: 0,
            missing_count: 1,
            duration: Duration::ZERO,
            scope: Scope::Full,
        };
        let defs = vec![def("a", Some(install_echo))];
        let runner = |_cmd: &str, _args: &[&str]| -> Result<(), String> { Ok(()) };
        let mut log = String::new();
        let fr = fix(
            &res,
            &defs,
            &FixOptions {
                dry_run: false,
                runner: Some(&runner),
            },
            |m| log.push_str(m),
        );
        assert_eq!(fr.fixed, 1);
        assert_eq!(fr.failed, 0);
    }

    #[test]
    fn live_runner_failure_counted() {
        let res = DoctorResult {
            checks: vec![miss("a")],
            ok_count: 0,
            warn_count: 0,
            missing_count: 1,
            duration: Duration::ZERO,
            scope: Scope::Full,
        };
        let defs = vec![def("a", Some(install_echo))];
        let runner = |_cmd: &str, _args: &[&str]| -> Result<(), String> { Err("boom".into()) };
        let mut log = String::new();
        let fr = fix(
            &res,
            &defs,
            &FixOptions {
                dry_run: false,
                runner: Some(&runner),
            },
            |m| log.push_str(m),
        );
        assert_eq!(fr.failed, 1);
        assert_eq!(fr.fixed, 0);
    }

    #[test]
    fn format_summary_pattern() {
        let s = format_fix_summary(&FixResult {
            fixed: 3,
            failed: 1,
            already_ok: 5,
            skipped: 0,
        });
        assert!(s.contains("3 fixed, 1 failed, 5 already OK"));
    }
}
