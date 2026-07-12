# 65 · Android App Development ◆ (By Example, Kotlin †)

**prd row**: Pass 4 · Concurrency & Systems · By Example · Kotlin † ◆ · Learn 165 / Drill 265 ·
Nvim-ready Partial · VSCode-ready Partial. ([prd canonical table](../prd.md#the-90-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: `◆` app-domain — building a real Android app: fundamentals (activities/lifecycle/manifest/
intents), Jetpack Compose UI + state, ViewModel/unidirectional-data-flow architecture, Room/DataStore/
Retrofit data, coroutines/flows, platform concerns, and applied testing. **Tooling note (DD-17)**: Android
Studio + the Android SDK/Gradle are the practical baseline (emulator/AVD); the topic favours the Gradle CLI
(`./gradlew`) for the raw build/test form.

## Why this exists · the big idea

- **The problem before the solution**: an Android app juggles UI, lifecycle, background work, local and
  remote data, and config changes at once — without a disciplined architecture these concerns tangle and
  the app loses its state on every screen rotation.
- **Keep-this-if-you-forget-everything**: hoist state out of the UI into a ViewModel and drive the screen
  with unidirectional data flow — the UI becomes a pure function of state that survives the platform yanking
  it around.
- **Big ideas touched**: `coupling-vs-cohesion` — ViewModel and repository layering keep what changes
  together (the UI) apart from what changes on its own schedule (data, lifecycle); `layering-and-leaks` —
  the Android platform (activities, config changes, permissions) bleeds into your app, and the architecture
  exists to seal those leaks.

## Prerequisites

- **Prior topics**: [topic 64 Just Enough Kotlin](./64-just-enough-kotlin.md) (the language + coroutines),
  [topic 14 Frontend Essentials](./14-frontend-essentials.md) (components, state, accessible UI), and
  [topic 47 Advanced Frontend](./47-advanced-frontend.md) (declarative UI, state management, optimistic
  updates).
- **Tools & environment**: a macOS/Linux (or Windows) machine; **Android Studio** + the **Android SDK** +
  **Gradle** (`./gradlew`); an emulator/AVD or a device; a JDK. Favour the Gradle CLI for build/test.
- **Assumed knowledge**: Kotlin syntax + coroutines (topic 64); component + state UI thinking (topics
  12/29); calling an HTTP API (topic 11).

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: Jetpack Compose BOM ~**2026.06.01** (Compose core ~1.11.x); Android Studio stable
  line is "Quail" (2026.1.1 Patch 2, stable 2026-04-28). Room, DataStore, Retrofit, coroutines/flows, and
  JUnit + Compose UI testing via `./gradlew test` remain Google's/Square's current recommended stack.
- 2026-07-12 — verified (TIME-SENSITIVE, re-check at authoring): **minimum/target SDK** — starting
  **2026-08-31**, all **new** Google Play submissions/updates must **target Android 16 (API 36)**; existing
  published apps must target at least **Android 15 (API 35)** to stay visible on Android 16/17 devices
  (extensions to 2026-11-01 available). This deadline falls near the authoring window and Google has moved
  it before — re-confirm the exact date + API numbers immediately before publishing.
  (developer.android.com/google/play/requirements/target-sdk)

## Items

- App fundamentals: activities, fragments, the lifecycle, the manifest, intents.
- UI: Jetpack Compose (declarative UI), state hoisting, recomposition; views (survey).
- Architecture: ViewModel, unidirectional data flow, repository, dependency-injection intuition.
- Data & persistence: Room, DataStore, networking (Retrofit / coroutines).
- Concurrency: Kotlin coroutines & flows on Android.
- Platform concerns: permissions, background work, navigation, config changes.
- **Applied testing**: JUnit unit tests, instrumented tests, Compose UI tests via `./gradlew test`.

## Worked examples

Colocated under `android-app-development/learning/code/`; each runnable/testable via Gradle (DD-20/DD-30).

- **beginner** — a Compose screen with state; lifecycle-aware logging.
- **intermediate** — a ViewModel-backed list with a repository + Room; a coroutine network call with
  loading/error states (+ a unit test).
- **advanced** — navigation across screens with saved state; a flow-driven reactive UI; a config-change
  survival test.

## Capstone spec — intra-topic (subject → full runnable app)

- **Goal**: build a small but complete Android app — a Compose UI backed by a ViewModel + unidirectional
  data flow, a repository over Room (local) and a Retrofit coroutine call (remote) with loading/error
  states, navigation with saved state, and survival across a config change — covered by JUnit + a Compose
  UI test, buildable and testable from `./gradlew`.
- **Concepts exercised**: [ ] Compose UI + state hoisting [ ] a ViewModel + unidirectional data flow
  [ ] a repository over Room + Retrofit/coroutines [ ] loading/error states [ ] navigation + saved state +
  config-change survival [ ] JUnit + a Compose UI test.
- **Ordered steps**:
  1. `.../learning/capstone/code/` — a Compose screen driven by a ViewModel + state. Verify state changes
     recompose the UI and `./gradlew test` runs.
  2. Add a repository over Room + a Retrofit coroutine call with loading/error states. Verify data persists
     locally and a network error shows the error state (a unit test covers the ViewModel logic).
  3. Add navigation with saved state + config-change survival. Verify navigating and rotating preserves
     state; add a Compose UI test asserting the flow.
- **Acceptance criteria**: the app builds + runs on an emulator; local + remote data work with proper
  loading/error states; navigation + config-change preserve state; JUnit + Compose UI tests pass via
  `./gradlew test`.
- **Done bar**: runnable end-to-end (emulator/device) + tests green + web-verified.

## Read more

**Books**

- **Android Programming: The Big Nerd Ranch Guide**, 5th ed. — Bryan Sills, Brian Gardner, Kristin Marsicano & Chris Stewart (2022, Addison-Wesley). Long-running, widely respected hands-on Android primer, now covering Kotlin and Jetpack Compose.

**Papers & articles**

- **Guide to app architecture** — Android Developers, official (Google). Google's canonical recommended architecture for modern Android apps. <https://developer.android.com/topic/architecture>
- **Jetpack Compose documentation** — Android Developers, official (Google). The authoritative reference for Android's modern declarative UI toolkit. <https://developer.android.com/develop/ui/compose/documentation>
- **Kotlin overview for Android** — Android Developers, official (Google). Google's own framing of Kotlin as the primary Android language. <https://developer.android.com/kotlin/overview>

---

← Previous: [64 · Just Enough Kotlin](./64-just-enough-kotlin.md) · Next: [66 · Just Enough Swift](./66-just-enough-swift.md) →
