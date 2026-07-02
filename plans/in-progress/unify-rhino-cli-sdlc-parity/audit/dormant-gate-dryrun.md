# Dormant gate fixes — dry-run against current tree (Decision 11)

Simulates what each dormant gate would report once armed (trigger-path fixed for naming;
gherkin-cardinality wired into public's PR gate). Run directly (not via the buggy hook trigger),
since only the HOOK invocation is gated on the buggy path — the underlying validator logic runs
unconditionally when invoked directly, which is exactly what "armed" would look like.

## (a) Agent-naming validator — harness naming validate

### public

```
Agents naming validation: VALIDATION PASSED (0 violations)
```

### primer

```
Agents naming validation: VALIDATION PASSED (0 violations)
```

### infra

```
Agents naming validation: VALIDATION PASSED (0 violations)
```

## (b) Gherkin cardinality validator — specs gherkin-cardinality validate

### public

```
GHERKIN KEYWORD CARDINALITY AUDIT PASSED: every scenario uses each primary keyword at most once
```

### primer

```
GHERKIN KEYWORD CARDINALITY AUDIT PASSED: every scenario uses each primary keyword at most once
```

### infra

```
GHERKIN KEYWORD CARDINALITY AUDIT PASSED: every scenario uses each primary keyword at most once
```
