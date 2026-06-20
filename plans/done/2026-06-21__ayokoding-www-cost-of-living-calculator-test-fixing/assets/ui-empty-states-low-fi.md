# Lo-Fi Wireframes — Empty States (net-new)

The only net-new UI in this plan. Two tabs need an instructional empty state instead of a table computed
against blank input (fixes UWT-003 / UWT-007). Mobile-first; desktop shown too. Colors at hi-fi use design
tokens (`bg-muted`, `text-muted-foreground`, `text-foreground`), never raw hex.

## Savings tab — empty state (gross salary blank/zero)

### Mobile (375 px)

```
┌─────────────────────────────┐
│  Cost of living · Savings ·  │   ← tab bar (Savings active)
│  Minimum role               │
├─────────────────────────────┤
│  Gross monthly salary        │
│  ┌───────────────────────┐  │   ← styled Input primitive (DWT-003)
│  │                       │  │
│  └───────────────────────┘  │
│                              │
│        ╭───────────╮         │
│        │   $ icon  │         │   ← muted illustrative glyph
│        ╰───────────╯         │
│                              │
│   Enter your gross monthly   │
│   salary above to see your   │   ← instructional prompt, centered
│   savings per city.          │
│                              │
└─────────────────────────────┘
   (no table, no red numbers)
```

### Desktop (≥1024 px)

```
┌──────────────────────────────────────────────────────────────┐
│  [ Cost of living ]  [ Savings* ]  [ Minimum role ]           │
├──────────────────────────────────────────────────────────────┤
│  Gross monthly salary (before tax)  [ Input……… ]   USD        │
│                                                                │
│            ╭──────╮                                            │
│            │  $   │   Enter your gross monthly salary above    │
│            ╰──────╯   to see how much you'd save in each city. │
│                                                                │
└──────────────────────────────────────────────────────────────┘
```

## Minimum-role tab — empty state (savings target blank/zero)

### Mobile (375 px)

```
┌─────────────────────────────┐
│  …· Minimum role (active)    │
├─────────────────────────────┤
│  How to set the target       │
│  [ My salary | Ref role |    │   ← SegmentedControl (DWT-006)
│    Savings target* ]         │
│                              │
│  Monthly savings target      │
│  ┌───────────────────────┐  │   ← styled Input
│  └───────────────────────┘  │
│                              │
│        ╭───────────╮         │
│        │  ladder   │         │
│        ╰───────────╯         │
│   Enter a monthly savings    │
│   target above to see which  │
│   roles would meet it.       │
└─────────────────────────────┘
```

### Desktop (≥1024 px)

```
┌──────────────────────────────────────────────────────────────┐
│  [ Cost of living ]  [ Savings ]  [ Minimum role* ]           │
├──────────────────────────────────────────────────────────────┤
│  How to set the target  [ My salary | Ref role | Target* ]    │
│  Monthly savings target [ Input……… ]                          │
│                                                                │
│        ╭──────╮   Enter a monthly savings target above to     │
│        │ ▤▤▤  │   see which roles would meet it.               │
│        ╰──────╯                                                │
└──────────────────────────────────────────────────────────────┘
```

## Behaviour

- The empty state replaces the table only; the input row and (min-role) the segmented control remain visible
  and usable above it.
- As soon as the input parses to a positive number, the prompt is replaced by the populated table.
- Both prompt strings are localized (en + id) — see the `id` equivalents in `delivery.md`.
