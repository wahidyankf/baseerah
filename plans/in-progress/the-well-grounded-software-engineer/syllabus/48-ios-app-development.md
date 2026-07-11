# 48 · iOS App Development ◆ (By Example, Swift †)

**prd row**: Pass 4 · Concurrency & Systems · By Example · Swift † ◆ · Learn 148 / Drill 248 ·
Nvim-ready Partial · VSCode-ready Partial. ([prd canonical table](../prd.md#the-61-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: `◆` app-domain — building a real iOS app: the app/scene lifecycle, SwiftUI (declarative
views + state/binding/observable), MVVM + the Observation framework, `Codable`/`URLSession` data, Swift
concurrency (`async`/`await`, actors), platform concerns, and applied testing. **Tooling note (DD-17)**:
Xcode is required (simulator, signing, SwiftUI previews); the topic uses Xcode where mandated and shows
the `swift`/`xcodebuild` CLI form where possible.

## Prerequisites

- **Prior topics**: [topic 47 Just Enough Swift](./47-just-enough-swift.md) (the language + `async`/`await`),
  [topic 12 Frontend Essentials](./12-frontend-essentials.md) (component + state UI), and
  [topic 46 Android App Development](./46-android-app-development.md) (the mobile app pattern to contrast).
- **Tools & environment**: a **macOS** machine with **Xcode** (simulator, signing, SwiftUI previews);
  `swift` / `xcodebuild` from the CLI where possible; a simulator or a device.
- **Assumed knowledge**: Swift syntax + `async`/`await` (topic 47); declarative UI + state (topics 12/46);
  calling an HTTP API (topic 09).

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: the **Observation framework** (`@Observable` macro, classes only, fine-grained
  dependency tracking, iOS 17+) is the current replacement for manual `ObservableObject`/`@Published`.
  SwiftUI declarative views, `Codable`/`URLSession`, Core Data/SwiftData, Swift concurrency (actors,
  structured concurrency), and XCTest/XCUITest + `xcodebuild test` are all current/unchanged.
  (developer.apple.com/documentation/swiftui/migrating-from-the-observable-object-protocol-to-the-observable-macro)
- 2026-07-12 — verified (TIME-SENSITIVE, re-check at authoring): Apple mandates apps be **built with the
  current iOS SDK / Xcode** for App Store submissions on a **recurring annual deadline** (the 2026 cycle
  required the iOS 26 SDK / Xcode 26 from 2026-04-28 — an SDK-build requirement, independent of your app's
  deployment target, which can still be iOS 16/17). Frame this as the _annual SDK-mandate pattern_ (the iOS
  analogue of topic 46's Android Play target-API deadline), not a fixed date; pull the then-current deadline
  at authoring time. (developer.apple.com/news/upcoming-requirements)

## Items

- App fundamentals: the app/scene lifecycle, the view hierarchy, the responder chain.
- UI: SwiftUI (declarative views, state/binding/observable); UIKit (survey/contrast).
- Architecture: MVVM; the Observation framework (`@Observable`, iOS 17+) with `@State`/`@Binding` (legacy
  `ObservableObject`/`@Published` as contrast); dependency-injection intuition.
- Data & persistence: `Codable`, `URLSession` networking, Core Data / SwiftData intuition.
- Concurrency: Swift `async`/`await`, actors, structured concurrency.
- Platform concerns: navigation, permissions, background tasks, lifecycle events.
- **Applied testing**: XCTest unit tests, XCUITest UI tests, `xcodebuild test` from the CLI.

## Worked examples

Colocated under `ios-app-development/learning/code/`; each runnable/testable via Xcode/`xcodebuild`
(DD-20/DD-30).

- **beginner** — a SwiftUI view driven by `@State`; a binding-based form.
- **intermediate** — an MVVM screen with an observable model + `async`/`await` networking with
  loading/error states (+ an XCTest).
- **advanced** — a navigation stack with passed state; an actor-isolated data cache; a persistence
  round-trip.

## Capstone spec — intra-topic (subject → full runnable app)

- **Goal**: build a small but complete iOS app — SwiftUI views under MVVM with an `@Observable` model,
  `URLSession`/`Codable` networking via `async`/`await` with loading/error states, an actor-isolated cache,
  navigation with passed state, and a persistence round-trip — covered by XCTest + an XCUITest,
  buildable/testable via `xcodebuild`.
- **Concepts exercised**: [ ] SwiftUI + `@State`/`@Binding` [ ] MVVM + `@Observable` [ ] `async`/`await`
  `URLSession`/`Codable` networking + loading/error states [ ] an actor-isolated cache [ ] navigation +
  passed state + a persistence round-trip [ ] XCTest + XCUITest.
- **Ordered steps**:
  1. `.../learning/capstone/code/` — a SwiftUI screen under MVVM with an `@Observable` model. Verify state
     changes update the view and `xcodebuild test` runs.
  2. Add `async`/`await` `URLSession` networking (`Codable`) with loading/error states + an actor cache.
     Verify a network error shows the error state and the cache is actor-isolated (an XCTest covers the
     model).
  3. Add navigation with passed state + a persistence round-trip. Verify navigating preserves state and data
     survives relaunch; add an XCUITest asserting the flow.
- **Acceptance criteria**: the app builds + runs on a simulator; networking has proper loading/error states;
  the cache is actor-isolated; navigation + persistence work; XCTest + XCUITest pass via `xcodebuild test`.
- **Done bar**: runnable end-to-end (simulator/device) + tests green + web-verified.

---

← Previous: [47 · Just Enough Swift](./47-just-enough-swift.md) · Next: [49 · Just Enough C#](./49-just-enough-csharp.md) →
