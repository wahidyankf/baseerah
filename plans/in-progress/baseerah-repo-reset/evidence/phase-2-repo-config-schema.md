# Phase 2 — `repo-config.yml` Schema Verdict

Source read: `apps/rhino-cli/src/application/repo_config/mod.rs`,
`apps/rhino-cli/src/application/env/validate.rs` (`Contract`), and
`apps/rhino-cli/src/application/env/injection.rs` (`Manifest`).

| Key                     | Field type           | `#[serde(default)]`? | Accepts `[]`? | May omit key entirely?                                                                                                                                                                                                           |
| ----------------------- | -------------------- | -------------------- | ------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `specs.ddd-areas`       | `Vec<String>`        | Yes (field-level)    | Yes           | Yes                                                                                                                                                                                                                              |
| `specs.domain-areas`    | `Vec<String>`        | Yes (field-level)    | Yes           | Yes                                                                                                                                                                                                                              |
| `env-contract.surfaces` | `Vec<SurfaceConfig>` | **No** (field-level) | Yes           | **No**, if the parent `env-contract:` key is present. The whole `env-contract:` top-level section CAN be omitted (`RepoConfig.env_contract: Option<EnvContract>` is `#[serde(default)]`), which skips this requirement entirely. |
| `env-injection.apps`    | `Vec<InjectionApp>`  | Yes (field-level)    | Yes           | Yes                                                                                                                                                                                                                              |

## Verdict

- `specs.ddd-areas` and `specs.domain-areas`: use `[]` (empty list) — confirmed safe by
  `repo_config/mod.rs`'s own regression test comment, which states these are "legitimately empty in
  some repos".
- `env-contract.surfaces`: use `[]` if the `env-contract:` top-level key is kept in the file. If the
  plan instead omits the whole `env-contract:` section, that is also valid (the field is `Option` at
  the `RepoConfig` level) — but Phase 2's delivery item only calls for clearing `env-contract.surfaces`,
  not removing the section, so this plan keeps `env-contract:` present with `surfaces: []`.
- `env-injection.apps`: use `[]` or omit the key — both parse. This plan uses `[]` for consistency
  with the other three cleared keys, keeping the section header for future Baseerah entries.

**Conclusion for the two `repo-config.yml` edit steps below**: set all four keys
(`specs.ddd-areas`, `specs.domain-areas`, `env-contract.surfaces`, `env-injection.apps`) to `[]`
rather than omitting them — uniform, explicit, and verified safe against the actual deserializer, not
just inferred from convention.
