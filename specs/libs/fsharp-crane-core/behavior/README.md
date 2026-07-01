# Behavior — fsharp-crane-core

Gherkin behavioral specifications for
[fsharp-crane-core](../../../../libs/fsharp-crane-core/project.json), the shared F# domain/logic
core for PDF-to-Markdown conversion and verification.

## Structure

```
specs/libs/fsharp-crane-core/behavior/
└── gherkin/
    └── convert/
        └── pdf-to-markdown-routing.feature
```

## Status

No Cucumber/Gherkin runner currently consumes these scenarios — `fsharp-crane-core` is exercised
via xUnit tests (`dotnet test`) under `tests/unit/Tests/` (see the top-level
[README.md](../README.md#status)).
