---
title: "Overview"
date: 2026-05-21T00:00:00+07:00
draft: false
weight: 10000000
description: "Hermes — Meta's open-source JavaScript engine for React Native with AOT compilation, Hades GC, and JSI integration"
tags: ["hermes", "react-native", "javascript-engine", "mobile", "aot-compilation", "meta", "overview"]
---

Hermes is Meta's open-source JavaScript engine built exclusively for React Native mobile
applications. It is not a general-purpose runtime — it does not run Node.js programs or
execute JavaScript in browsers. Its single design goal is to make React Native apps start
faster and use less memory on real-world Android and iOS hardware.

## What Hermes Is

Hermes is a JavaScript engine, which means it is the component responsible for parsing,
compiling, and executing JavaScript code inside a React Native app. Before Hermes, React
Native used JavaScriptCore (JSC), the same engine that powers WebKit and Safari. JSC was
designed for browser workloads — it assumes warm caches, fast CPUs, and ample RAM. Mobile
app cold-starts expose all of JSC's weak points.

Hermes takes a different approach: it compiles JavaScript to bytecode at build time rather
than at runtime. When the user launches the app, the engine loads pre-compiled bytecode
directly instead of parsing and compiling source JavaScript. This eliminates the largest
single cost of mobile app startup.

Hermes is open-source at [github.com/facebook/hermes](https://github.com/facebook/hermes)
under the MIT license. Meta created and maintains it. The project is written primarily in
C++ (approximately 36% of the codebase) and targets ARM processors found in Android and iOS
devices.

## Why Hermes Exists: The Cold-Start Problem

A "cold start" occurs when the operating system launches an app for the first time after a
reboot or after the process has been killed. For a React Native app using JSC, cold start
involves:

1. Loading the JavaScript bundle from disk into memory
2. Parsing the raw JavaScript source text into an AST (Abstract Syntax Tree)
3. Compiling the AST — potentially with JIT optimizations — into executable code
4. Running the JavaScript initialization code (module loading, component registration)
5. Triggering the first render cycle

Steps 2 and 3 are the bottleneck. On a low-end Android device, parsing a production
JavaScript bundle can take hundreds of milliseconds before a single React component renders.
This delay translates directly into lower app store ratings and higher abandonment rates.

Hermes eliminates steps 2 and 3 from the runtime path by moving them to build time. The
Metro bundler compiles JavaScript to Hermes Bytecode (`.hbc` format) when you build the
app. At launch, the engine loads binary bytecode — a format the interpreter reads directly —
with no parsing or compilation required.

## Hermes V1 vs. Prior Hermes

Hermes V1, which ships with React Native 0.84 (released February 11, 2026), marks a
significant maturity milestone. Before V1, Hermes was the default engine on Android (since
React Native 0.70) but optional on iOS, and it relied heavily on polyfills for modern
JavaScript features because its native ES support was limited.

**What changed in Hermes V1:**

- Native support for `let` and `const` (previously required transpilation)
- Native `async`/`await` without polyfills
- Native ES2022 classes with proper prototype chains
- Native `Map`, `Set`, `WeakMap`, `WeakSet` implementations
- Removal of property count limits per object (previously capped)
- Reduced polyfill bundle size, shrinking the total app binary
- Experimental JIT compiler for hot functions (opt-in, not enabled by default)
- React Native 0.84+ ships Hermes as precompiled binaries on iOS, reducing build time

Before V1, developers using modern JavaScript had to rely on Babel transforms or runtime
polyfills that added to bundle size and runtime overhead. V1 eliminates most of that
dependency while maintaining backward compatibility.

## Key Differentiators vs. JavaScriptCore

| Property                          | Hermes                                | JavaScriptCore (JSC) |
| --------------------------------- | ------------------------------------- | -------------------- |
| **Compilation strategy**          | AOT (build time)                      | JIT (runtime)        |
| **Cold-start time**               | ~40% faster than JSC                  | Baseline             |
| **Memory footprint**              | Lower (compact bytecode, Hades GC)    | Higher               |
| **`eval()` support**              | No                                    | Yes                  |
| **Dynamic code generation**       | No (`new Function` blocked)           | Yes                  |
| **ES2022 native support**         | Yes (V1+)                             | Partial              |
| **Garbage collector**             | Hades (concurrent, no stop-the-world) | Conservative GC      |
| **React Native New Architecture** | Full support via JSI                  | Partial/legacy       |
| **Designed for mobile**           | Yes (primary target)                  | No (browser origin)  |

The lack of `eval()` and `new Function` is not a limitation — it is a security and
predictability feature. Dynamic code generation is a common attack vector in mobile apps and
makes static analysis impossible. Hermes enforces that all executable code must come from
the pre-compiled bytecode included in the app binary.

## Learning Path

This content covers Hermes across three progression levels. Choose the starting point that
matches your current knowledge:

**Start at Beginner if:** you are new to React Native internals, you know JavaScript but
have not thought about how engines work, or you want to understand what Hermes is before
enabling it in a project.

**Start at Intermediate if:** you have enabled Hermes in a React Native project, you want
to understand Hades GC or JSI in depth, or you are migrating to the New Architecture and
need to understand how Hermes fits in.

**Start at Advanced if:** you are debugging production performance regressions, building
custom native modules with JSI, or evaluating Static Hermes and the experimental JIT for
specialized workloads.

**Prerequisites**: A React Native project (0.70 or later) and basic JavaScript knowledge
(functions, async/await, modules). Understanding of Android and iOS build systems helps for
the build configuration sections but is not required to follow the conceptual explanations.

Official documentation: [reactnative.dev/docs/hermes](https://reactnative.dev/docs/hermes)
and [github.com/facebook/hermes](https://github.com/facebook/hermes).
