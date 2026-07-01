# rust-commons — Product Overview

`rust-commons` provides shared Rust utilities consumed by the workspace's Rust CLIs. Today it
exposes one module, `links` (`rust_commons::links`), whose `check_links` function walks a content
directory, extracts internal markdown links (`[text](/path)`) from every `.md` file, resolves
each target against the content root (including the `/c/` URL routing namespace used by the
`-www` sites), and returns a `CheckResult` — broken links, error count, and per-file diagnostics —
that callers format as text, JSON, or Markdown.

See [README.md](./README.md) for C4 L1 product framing.
