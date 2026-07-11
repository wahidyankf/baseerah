# Syllabus — The Well-Grounded Software Engineer

The **per-topic detail layer** for this section. Start with **[overview.md](./overview.md)** — it
defines how to read a topic file, the legend, the cross-cutting authoring guarantees, the capstone
policy, and the per-topic file template. The [prd table](../prd.md#the-61-topics--canonical-table-spiral-order-identical-in-both-tracks)
remains the single source of truth for topic set, order, slug, format, primary language, and weights;
this folder adds the concrete **Items**, **Worked examples**, **Capstone specs**, and dated **Accuracy
notes** per topic.

One file per topic — `NN-<slug>.md`, `NN` = order of appearance (01..61). Ten inter-topic capstone
specs are appended inside their anchor topic files (see the
[Capstone Policy](./overview.md#capstone-policy-dd-27)).

## Pass 0 · Set Up Your Forge (topics 01–03)

| NN  | Topic                                  | File                                               |
| --- | -------------------------------------- | -------------------------------------------------- |
| 01  | Just Enough Nvim (vanilla, no plugins) | [01-just-enough-nvim.md](./01-just-enough-nvim.md) |
| 02  | Just Enough Lua                        | [02-just-enough-lua.md](./02-just-enough-lua.md)   |
| 03  | Extending Neovim                       | [03-extending-neovim.md](./03-extending-neovim.md) |

Anchored inter-topic capstone: `capstone-forge-ready` (in `03-extending-neovim.md`).

## Pass 1 · First Working Software (topics 04–14)

| NN  | Topic                                         | File                                                                                                 |
| --- | --------------------------------------------- | ---------------------------------------------------------------------------------------------------- |
| 04  | Just Enough Python                            | [04-just-enough-python.md](./04-just-enough-python.md)                                               |
| 05  | Just Enough Bash                              | [05-just-enough-bash.md](./05-just-enough-bash.md)                                                   |
| 06  | Data Structures & Algorithms Essentials       | [06-data-structures-and-algorithms-essentials.md](./06-data-structures-and-algorithms-essentials.md) |
| 07  | Object-Oriented Programming Essentials        | [07-object-oriented-programming-essentials.md](./07-object-oriented-programming-essentials.md)       |
| 08  | SQL Essentials                                | [08-sql-essentials.md](./08-sql-essentials.md)                                                       |
| 09  | Backend Essentials                            | [09-backend-essentials.md](./09-backend-essentials.md)                                               |
| 10  | Networking Essentials                         | [10-networking-essentials.md](./10-networking-essentials.md)                                         |
| 11  | Just Enough TypeScript                        | [11-just-enough-typescript.md](./11-just-enough-typescript.md)                                       |
| 12  | Frontend Essentials                           | [12-frontend-essentials.md](./12-frontend-essentials.md)                                             |
| 13  | Software Testing (incl. TDD + property-based) | [13-software-testing.md](./13-software-testing.md)                                                   |
| 14  | Security Essentials                           | [14-security-essentials.md](./14-security-essentials.md)                                             |

Anchored inter-topic capstones (both in `14-security-essentials.md`): `capstone-first-working-software`
(Pass-1 boundary), `capstone-full-stack-app` (cross-cutting).

## Pass 2 · Solidify the Core (topics 15–25)

| NN  | Topic                             | File                                                                                     |
| --- | --------------------------------- | ---------------------------------------------------------------------------------------- |
| 15  | Computer Science Foundations      | [15-computer-science-foundations.md](./15-computer-science-foundations.md)               |
| 16  | Object-Oriented Design & Patterns | [16-object-oriented-design-and-patterns.md](./16-object-oriented-design-and-patterns.md) |
| 17  | Programming Paradigms             | [17-programming-paradigms.md](./17-programming-paradigms.md)                             |
| 18  | Functional Programming (incl. CT) | [18-functional-programming.md](./18-functional-programming.md)                           |
| 19  | Concurrency & Parallelism (Core)  | [19-concurrency-and-parallelism.md](./19-concurrency-and-parallelism.md)                 |
| 20  | Advanced Algorithms               | [20-advanced-algorithms.md](./20-advanced-algorithms.md)                                 |
| 21  | Advanced Networking               | [21-advanced-networking.md](./21-advanced-networking.md)                                 |
| 22  | Software Engineering Practices    | [22-software-engineering-practices.md](./22-software-engineering-practices.md)           |
| 23  | Advanced SQL & Query Performance  | [23-advanced-sql-and-query-performance.md](./23-advanced-sql-and-query-performance.md)   |
| 24  | Software Product Engineering ▲    | [24-software-product-engineering.md](./24-software-product-engineering.md)               |
| 25  | Project Management ▲              | [25-project-management.md](./25-project-management.md)                                   |

Anchored inter-topic capstone: `capstone-solid-core` (in `25-project-management.md`).

## Pass 3 · Build for the Real World (topics 26–40)

| NN  | Topic                                   | File                                                                       |
| --- | --------------------------------------- | -------------------------------------------------------------------------- |
| 26  | NoSQL Databases (Valkey/Redis)          | [26-nosql-databases.md](./26-nosql-databases.md)                           |
| 27  | Graph Databases                         | [27-graph-databases.md](./27-graph-databases.md)                           |
| 28  | Backend at Scale (incl. Valkey caching) | [28-backend-at-scale.md](./28-backend-at-scale.md)                         |
| 29  | Advanced Frontend                       | [29-advanced-frontend.md](./29-advanced-frontend.md)                       |
| 30  | Software Architecture (incl. hexagonal) | [30-software-architecture.md](./30-software-architecture.md)               |
| 31  | Domain-Driven Design                    | [31-domain-driven-design.md](./31-domain-driven-design.md)                 |
| 32  | System Design                           | [32-system-design.md](./32-system-design.md)                               |
| 33  | Event-Driven Architecture               | [33-event-driven-architecture.md](./33-event-driven-architecture.md)       |
| 34  | Containers & Orchestration              | [34-containers-and-orchestration.md](./34-containers-and-orchestration.md) |
| 35  | Cloud & IaC                             | [35-cloud-and-iac.md](./35-cloud-and-iac.md)                               |
| 36  | Data Engineering                        | [36-data-engineering.md](./36-data-engineering.md)                         |
| 37  | Creating AI-Powered Apps                | [37-creating-ai-powered-apps.md](./37-creating-ai-powered-apps.md)         |
| 38  | IT Security (risk/asset/network)        | [38-it-security.md](./38-it-security.md)                                   |
| 39  | Offensive Security (red team, Kali)     | [39-offensive-security.md](./39-offensive-security.md)                     |
| 40  | Defensive Security (blue team, SOC/IR)  | [40-defensive-security.md](./40-defensive-security.md)                     |

Anchored inter-topic capstones (all in `40-defensive-security.md`): `capstone-real-world-delivery`
(Pass-3 boundary), `capstone-secure-service` (cross-cutting), `capstone-data-pipeline` (cross-cutting).

## Pass 4 · Concurrency & Systems (topics 41–59)

| NN  | Topic                                | File                                                                                 |
| --- | ------------------------------------ | ------------------------------------------------------------------------------------ |
| 41  | Just Enough Go                       | [41-just-enough-go.md](./41-just-enough-go.md)                                       |
| 42  | CSP-Style Concurrency                | [42-csp-style-concurrency.md](./42-csp-style-concurrency.md)                         |
| 43  | Just Enough Elixir                   | [43-just-enough-elixir.md](./43-just-enough-elixir.md)                               |
| 44  | Actor-Model Concurrency              | [44-actor-model-concurrency.md](./44-actor-model-concurrency.md)                     |
| 45  | Just Enough Kotlin                   | [45-just-enough-kotlin.md](./45-just-enough-kotlin.md)                               |
| 46  | Android App Development ◆            | [46-android-app-development.md](./46-android-app-development.md)                     |
| 47  | Just Enough Swift                    | [47-just-enough-swift.md](./47-just-enough-swift.md)                                 |
| 48  | iOS App Development ◆                | [48-ios-app-development.md](./48-ios-app-development.md)                             |
| 49  | Just Enough C#                       | [49-just-enough-csharp.md](./49-just-enough-csharp.md)                               |
| 50  | Windows App Development ◆            | [50-windows-app-development.md](./50-windows-app-development.md)                     |
| 51  | Linux App Development ◆              | [51-linux-app-development.md](./51-linux-app-development.md)                         |
| 52  | Just Enough C                        | [52-just-enough-c.md](./52-just-enough-c.md)                                         |
| 53  | Linux OS                             | [53-linux-os.md](./53-linux-os.md)                                                   |
| 54  | Windows OS (incl. PowerShell)        | [54-windows-os.md](./54-windows-os.md)                                               |
| 55  | System Programming                   | [55-system-programming.md](./55-system-programming.md)                               |
| 56  | Lisp (Scheme core + Clojure sidebar) | [56-lisp.md](./56-lisp.md)                                                           |
| 57  | Type Systems (OCaml + Haskell + F#)  | [57-type-systems.md](./57-type-systems.md)                                           |
| 58  | Compilers, Parsers & Transpilers     | [58-compilers-parsers-and-transpilers.md](./58-compilers-parsers-and-transpilers.md) |
| 59  | Site Reliability Engineering         | [59-site-reliability-engineering.md](./59-site-reliability-engineering.md)           |

Anchored inter-topic capstones (both in `59-site-reliability-engineering.md`):
`capstone-concurrency-and-systems` (Pass-4 boundary), `capstone-concurrency-showdown` (cross-cutting).

## Pass 5 · Lead at Altitude (topics 60–61)

| NN  | Topic                             | File                                                           |
| --- | --------------------------------- | -------------------------------------------------------------- |
| 60  | IT Governance & GRC (GDPR + NIST) | [60-it-governance-grc.md](./60-it-governance-grc.md)           |
| 61  | Engineering Management            | [61-engineering-management.md](./61-engineering-management.md) |

Anchored inter-topic capstone: `capstone-lead-at-altitude` (in `61-engineering-management.md`).

## Inter-topic capstone index

| Capstone slug                      | Anchor file                          |
| ---------------------------------- | ------------------------------------ |
| `capstone-forge-ready`             | `03-extending-neovim.md`             |
| `capstone-first-working-software`  | `14-security-essentials.md`          |
| `capstone-full-stack-app`          | `14-security-essentials.md`          |
| `capstone-solid-core`              | `25-project-management.md`           |
| `capstone-real-world-delivery`     | `40-defensive-security.md`           |
| `capstone-secure-service`          | `40-defensive-security.md`           |
| `capstone-data-pipeline`           | `40-defensive-security.md`           |
| `capstone-concurrency-and-systems` | `59-site-reliability-engineering.md` |
| `capstone-concurrency-showdown`    | `59-site-reliability-engineering.md` |
| `capstone-lead-at-altitude`        | `61-engineering-management.md`       |
