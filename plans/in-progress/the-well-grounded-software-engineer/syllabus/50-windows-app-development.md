# 50 · Windows App Development ◆ (By Example, C# †)

**prd row**: Pass 4 · Concurrency & Systems · By Example · C# † ◆ · Learn 150 / Drill 250 ·
Nvim-ready Partial · VSCode-ready Partial. ([prd canonical table](../prd.md#the-61-topics--canonical-table-spiral-order-identical-in-both-tracks))

**Scope note**: `◆` app-domain — building a real Windows desktop app: .NET fundamentals, WinUI/WPF (XAML +
data binding + MVVM), async on the UI thread (dispatcher/cancellation), local persistence, packaging
intuition (MSIX), and applied testing. **Tooling note (DD-17)**: Visual Studio / the .NET SDK is the
practical baseline for WinUI/WPF; the topic favours the `dotnet` CLI for build/test/run where possible.

## Prerequisites

- **Prior topics**: [topic 49 Just Enough C#](./49-just-enough-csharp.md) (the language + `async`/`await`),
  [topic 12 Frontend Essentials](./12-frontend-essentials.md) (component + state UI, MVVM intuition), and
  [topic 29 Advanced Frontend](./29-advanced-frontend.md) (data binding, state management).
- **Tools & environment**: a **Windows** machine with **Visual Studio** / the **.NET SDK** (WinUI/WPF
  workloads); `dotnet` from the CLI where possible. (WinUI/WPF are Windows-only.)
- **Assumed knowledge**: C# syntax + `async`/`await` (topic 49); MVVM + data-binding thinking (topics
  12/29); local file/DB persistence (topic 08).

## Accuracy notes (web-verified)

> Verified in the pre-authoring `web-researcher` sweep (#48, DD-28).

- 2026-07-12 — verified: WinUI 3 / WPF / WinForms (XAML, data binding, MVVM) remain actively-supported
  Windows desktop UI stacks; MSIX is still Microsoft's standard packaging format; `dotnet` CLI + xUnit/NUnit
  via `dotnet test`, and SQLite-on-.NET (Microsoft.Data.Sqlite) are current/unchanged.
- 2026-07-12 — verified (TIME-SENSITIVE, re-check at authoring): the **Windows App SDK** (platform under
  WinUI 3) is mid-transition from the 1.x line to 2.x (**2.2.0** released 2026-06-09), licensed **MIT**.
  This moves fast — avoid pinning a specific SDK version in authored content; re-pull at authoring time.
  (github.com/microsoft/WindowsAppSDK)

## Items

- .NET fundamentals: the runtime, the project/build model, NuGet.
- Desktop UI: WinUI/WPF (XAML, data binding, MVVM); WinForms (survey).
- Async on the UI thread: `async`/`await`, the dispatcher, cancellation.
- Data & persistence: file I/O, settings, a local DB / SQLite.
- Packaging & deployment: MSIX intuition, app lifecycle.
- **Applied testing**: xUnit/NUnit unit tests via `dotnet test`; UI-test intuition.

## Worked examples

Colocated under `windows-app-development/learning/code/`; each runnable/testable via `dotnet`
(DD-20/DD-30).

- **beginner** — a XAML window with data binding; a simple command handler.
- **intermediate** — an MVVM view with an observable model + async data load (+ a `dotnet test` unit test).
- **advanced** — cancellation + progress reporting on a long task; a persistence + settings round-trip.

## Capstone spec — intra-topic (subject → full runnable app)

- **Goal**: build a small but complete Windows desktop app — a WinUI/WPF XAML UI under MVVM with data
  binding, an async data load off the UI thread with cancellation + progress reporting, and a local
  SQLite/settings persistence round-trip — covered by xUnit/NUnit tests, buildable/testable via
  `dotnet`.
- **Concepts exercised**: [ ] XAML + data binding [ ] MVVM + an observable model [ ] `async`/`await` off
  the UI thread + the dispatcher [ ] cancellation + progress reporting [ ] a persistence + settings
  round-trip [ ] `dotnet test` unit tests.
- **Ordered steps**:
  1. `.../learning/capstone/code/` — a XAML window under MVVM with data binding + a command. Verify the UI
     binds to the model and `dotnet test` runs.
  2. Add an async data load with cancellation + progress. Verify the UI stays responsive, progress updates,
     and a cancel stops the work (a unit test covers the view-model logic).
  3. Add a SQLite + settings persistence round-trip. Verify data + settings survive relaunch.
- **Acceptance criteria**: the app builds + runs on Windows; the UI binds via MVVM; async work is
  cancellable and non-blocking; persistence survives relaunch; `dotnet test` passes.
- **Done bar**: runnable end-to-end (Windows) + tests green + web-verified.

---

← Previous: [49 · Just Enough C#](./49-just-enough-csharp.md) · Next: [51 · Linux App Development](./51-linux-app-development.md) →
