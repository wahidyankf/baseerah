# Components — rust-commons

C4 Level 3 components for `rust-commons`.

| Module  | Export                              | Purpose                                                  |
| ------- | ----------------------------------- | -------------------------------------------------------- |
| `links` | `check_links(content_dir)`          | Walks a content dir and checks internal markdown links   |
| `links` | `CheckResult`, `BrokenLink`         | Result types: checked/error counts, per-link diagnostics |
| `links` | `output_links_text/_json/_markdown` | Formats a `CheckResult` as text, JSON, or Markdown       |

See [../behavior/gherkin/links/](../behavior/gherkin/links/) for the behavioral spec.
See [component-rust-commons.md](./component-rust-commons.md) for the C4 component diagram
placeholder.
