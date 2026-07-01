# Containers — web-ui

C4 Level 2 containers for `web-ui`.

`web-ui` ships as a single container: a TypeScript/React component package
(`@open-sharia-enterprise/web-ui`) built with Tailwind CSS and Radix UI primitives, consumed at
build time by every frontend app in the workspace. It has no separate deployable runtime of its
own — Storybook (`nx run web-ui:storybook`) is a development-time preview container, not a
production deployable.

See [container.md](./container.md) for the C4 container diagram placeholder.
