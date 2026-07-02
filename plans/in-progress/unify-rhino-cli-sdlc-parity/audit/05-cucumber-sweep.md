# Cucumber / .feature / Cargo.lock isolation sweep — per repo

## public

- cucumber version: cucumber = "0.23.0"
- [[test]] block count: 0
  0
- tests/\*.rs count: 4
- .feature file count: 41
- .feature dirs: agents ddd docs env git repo-governance spec-coverage specs system workflows
- root Cargo.toml exists: NO
- apps/rhino-cli/Cargo.toml has [workspace]: NO

## primer

- cucumber version: cucumber = "0.22.1"
- [[test]] block count: 11
- tests/\*.rs count: 12
- .feature file count: 26
- .feature dirs: agents contracts docs env git java repo-governance spec-coverage system test-coverage workflows
- root Cargo.toml exists: NO
- apps/rhino-cli/Cargo.toml has [workspace]: NO

## infra

- cucumber version: cucumber = "0.23.0"
- [[test]] block count: 0
  0
- tests/\*.rs count: 4
- .feature file count: 22
- .feature dirs: agents contracts docs env git java repo-governance spec-coverage system test-coverage
- root Cargo.toml exists: NO
- apps/rhino-cli/Cargo.toml has [workspace]: NO
