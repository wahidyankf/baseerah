# System Context — web-ui-token

C4 Level 1 system context for `web-ui-token`.

## Actors and consumers

- **`web-ui`** — imports token constants for component styling.
- **Consuming apps** — import `src/tokens.css` in their `globals.css` and override brand tokens
  (optionally layering a per-app brand file such as `organiclever.css`).

`web-ui-token` has no runtime dependency on any backend; it is a pure, build-time constants
package.

See [context.md](./context.md) for the C4 context diagram placeholder.
