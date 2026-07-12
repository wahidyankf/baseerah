# 69 · Hybrid App Development (By Example, Dart †)

**prd row**: Pass 4 · Concurrency & Systems · By Example · Dart † · Learn 169 / Drill 269 · Nvim-ready Yes · VSCode-ready Yes. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: one codebase, many targets — Flutter's widget model, state management, and building for
mobile plus desktop from a single Dart codebase, with the cross-platform trade-offs kept explicit rather
than hidden. The usable language slice is the prerequisite [`68-just-enough-dart`](./68-just-enough-dart.md),
and the UI-composition instincts carry over from [`14-frontend-essentials`](./14-frontend-essentials.md).
`†`: Dart driving the Flutter framework and the `flutter` CLI.

## Why this exists · the big idea

- **The problem before the solution**: shipping the same app to iOS, Android, and desktop meant writing and
  maintaining it two or three times over, in three languages and toolchains, with the platforms drifting out
  of sync — a tax on every feature and a source of "it works on Android but not iOS" bugs.
- **Keep-this-if-you-forget-everything**: Flutter renders its own pixels instead of wrapping each
  platform's native widgets, so one Dart codebase looks and behaves identically everywhere — you trade the
  last mile of native fidelity for a single UI you build once. Know exactly what that trade buys and what
  it costs.
- **Big ideas touched**: `abstraction-and-its-cost` (the single-codebase abstraction hides three platforms,
  and the hidden thing — platform-specific behavior, native look, plugin gaps — leaks precisely where the
  abstraction is thinnest), `coupling-vs-cohesion` (the widget tree keeps a screen's structure and behavior
  cohesive, while state management decides how tightly UI couples to the data that drives it).

## Prerequisites

- **Prior topics**: [topic 68 Just Enough Dart](./68-just-enough-dart.md) (null-safe Dart, async/await,
  classes/mixins) and [topic 14 Frontend Essentials](./14-frontend-essentials.md) (declarative UI,
  component composition, layout thinking).
- **Tools & environment**: a macOS/Linux/Windows machine; the **Flutter SDK** (`flutter`) and Dart SDK
  pinned to a current stable; at least one target toolchain (an Android emulator, an iOS simulator on macOS,
  or a desktop target); Neovim/VSCode with the Dart LSP (DD-17).
- **Assumed knowledge**: writing null-safe Dart with async/await and classes (topic 68); thinking in a
  declarative component tree (topic 14); running a CLI build/run tool (topic 05).

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: keep the Flutter/Dart SDK at "a current stable" in shipped text — the widget model,
  `StatelessWidget`/`StatefulWidget`, `BuildContext`, the `flutter` CLI (`create`/`run`/`build`/`test`), and
  the multi-target build story (mobile + desktop) are stable and correctly unpinned. Flutter and Dart
  release on a moving cadence, so a pinned number would go stale fast.
- 2026-07-12 — verified (GAP for plan owner): the body names Flutter's built-in state options but does not
  commit to a specific third-party state-management package — re-verify the chosen package name/version once
  the worked examples are drafted, and keep the vanilla-first (`setState`) tier as the baseline.

## Items

- The widget tree: everything is a widget; `StatelessWidget` vs `StatefulWidget`; composition over
  inheritance; `BuildContext`.
- Layout & rendering: the constraints-down/sizes-up layout model, and why Flutter draws its own pixels
  instead of using native controls.
- State management, three tiers: `setState` (vanilla) → an inherited/provider approach (practical) → a
  reactive store for larger apps, choosing by app size.
- Navigation & app structure: routes, nested navigation, and keeping screens cohesive.
- Platform integration: platform channels and plugins for reaching native APIs, and where the
  single-codebase abstraction stops.
- Building for multiple targets: `flutter build` for mobile + desktop from one codebase, and the
  responsive/adaptive layout differences between form factors.

## Tensions & trade-offs — when NOT to reach for this

- **When native fidelity is the product**: an app whose value is deep platform integration, the exact
  native look-and-feel, or bleeding-edge OS APIs is fighting Flutter's own-rendering model. If your users
  will notice the difference, a native codebase (or a platform-native design system) may win despite the
  duplication.
- **The abstraction leaks at the edges**: date pickers, keyboards, permissions, background execution, and
  new OS features surface platform-specific behavior right through the "write once" promise. Every serious
  Flutter app carries some platform-channel and per-platform code — budget for it rather than being
  surprised by it.
- **Binary size and cold start**: bundling a rendering engine costs app size and startup time versus a thin
  native app. For a tiny utility or an app where install size is a conversion metric, that overhead may not
  be worth the shared codebase.

## Lineage — why it beat the alternative

- Cross-platform UI has a graveyard of approaches: write-native-twice (correct but expensive),
  WebView-in-a-shell hybrids like early Cordova/PhoneGap (one codebase but sluggish and un-native), and
  bridge-to-native-widgets like React Native (native controls but a serialization bridge and per-platform
  quirks). Flutter took a different bet — ship a rendering engine and draw every pixel itself — which
  eliminated the bridge and delivered pixel-identical, high-frame-rate UI across targets, at the cost of
  native-widget fidelity. That bet won for teams who value one consistent UI and a single codebase over
  last-mile nativeness. The widget-composition and state-management instincts built here transfer directly
  to the next primer's platform work in [`70-just-enough-csharp`](./70-just-enough-csharp.md) and to any
  declarative-UI framework you meet later.

## Worked examples

Colocated under `hybrid-app-development/learning/code/`; each runnable via the `flutter` CLI on at least
one target (DD-20/DD-30).

- **beginner** — a `StatelessWidget` screen composed from smaller widgets, plus a `StatefulWidget` counter
  driven by `setState`.
- **intermediate** — the same app refactored to a provider/inherited-state approach with navigation between
  two screens, run on two form factors (phone + desktop) to show adaptive layout.
- **advanced** — reach a native capability through a platform channel/plugin and handle the
  platform-specific fallback, making the abstraction's leak explicit and contained.

## Capstone spec — intra-topic (subject → full runnable)

- **Goal**: build one Flutter app from a single Dart codebase that runs on mobile and desktop — a multi-
  screen app with a chosen state-management approach, adaptive layout across form factors, and one native
  capability reached through a platform channel with a documented per-platform fallback.
- **Concepts exercised**: [ ] composed widget tree (Stateless + Stateful) [ ] a state-management approach
  beyond `setState` [ ] navigation across screens [ ] adaptive layout for phone + desktop [ ] a
  platform-channel/plugin call [ ] `flutter build` for two targets.
- **Ordered steps**:
  1. `.../learning/capstone/code/` — a multi-screen app with a composed widget tree and navigation. Verify
     `flutter run` renders and navigates on one target.
  2. Introduce a state-management approach (provider/inherited or a reactive store) driving shared state.
     Verify state updates propagate across screens without manual `setState` plumbing.
  3. Add adaptive layout so the UI reflows between phone and desktop. Verify `flutter run` looks correct on
     both form factors.
  4. Reach one native capability via a platform channel/plugin with a fallback. Verify the feature works on
     one platform and degrades gracefully where unsupported, and that `flutter build` succeeds for two
     targets.
- **Acceptance criteria**: one codebase builds and runs on two targets; state management works across
  screens; layout adapts to form factor; the native call succeeds with a documented fallback; the
  platform-specific leak is contained, not hidden.
- **Done bar**: runnable end-to-end + web-verified.

## Read more

**Books**

- **Flutter in Action** — Eric Windmill (2020, Manning). A well-regarded, widely recommended book-length
  primer on Flutter and Dart for building cross-platform apps.

**Papers & articles**

- **Flutter documentation** — official (docs.flutter.dev). The authoritative source for widgets, guides,
  and platform integration. <https://docs.flutter.dev/>
- **Flutter API reference** — official (api.flutter.dev). The canonical framework API reference.
  <https://api.flutter.dev/>

---

← Previous: [68 · Just Enough Dart](./68-just-enough-dart.md) · Next: [70 · Just Enough C#](./70-just-enough-csharp.md) →
