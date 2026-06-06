# rhino-cli — CLI Component

See [README.md](./README.md) for C4 L3 CLI internals.

## docs validate-links flags

| Flag                 | Type              | Description                                                                                                                                                                 |
| -------------------- | ----------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `--staged-only`      | bool              | Only validate files staged in the Git index.                                                                                                                                |
| `--exclude <prefix>` | repeatable string | Skip any markdown file whose repo-relative path starts with `<prefix>`. May be specified multiple times (e.g. `--exclude plans/done --exclude apps/ayokoding-web/content`). |
