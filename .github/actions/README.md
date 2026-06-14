# Composite Actions

Reusable composite actions that set up the toolchains the workflows depend on. Each is
referenced as `uses: ./.github/actions/<name>`.

| Action               | Purpose                                                                                                                        |
| -------------------- | ------------------------------------------------------------------------------------------------------------------------------ |
| `setup-node`         | Install Node.js via Volta, run `npm ci`, and configure the Nx cache                                                            |
| `setup-dotnet`       | Install the .NET SDK, Fantomas, and fsharplint with NuGet cache (F# backends)                                                  |
| `setup-rust`         | Install the Rust toolchain pinned by `rust-toolchain.toml`, with build cache, `cargo-llvm-cov`, `cargo-deny`, and `cargo-hack` |
| `setup-playwright`   | Install Playwright browsers and OS dependencies with browser cache (E2E)                                                       |
| `setup-docker-cache` | Configure Docker Buildx with GitHub Actions layer cache (integration/E2E Docker stacks)                                        |

The repository's active languages are TypeScript, Rust, and F#. The former
`setup-golang`, `setup-jvm`, `setup-python`, `setup-language`, and `install-language-deps`
actions were removed when Go and the polyglot demo apps left this repo — see the
[CI Conventions](../../repo-governance/development/infra/ci-conventions.md) and
[model/language tiers](../../repo-governance/development/agents/model-selection.md).

## See also

- [workflows/README.md](../workflows/README.md) — workflows that consume these actions
- [CI/CD Pipeline reference](../../docs/reference/system-architecture/ci-cd.md)
