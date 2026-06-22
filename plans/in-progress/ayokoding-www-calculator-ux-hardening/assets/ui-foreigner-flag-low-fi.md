# Lo-fi: Foreigner public-school flag (DWT-006 / UWT-002 / EWT-003)

Affects the **School** column cell in the cost-of-living table and the school row in the city-detail view,
for any city whose country does not open public school to foreign residents (8 cities) when school-age
children ≥ 1 and school type = public.

## Before (current runtime — the defect)

```
 School
 ┌─────────────────────────────┐
 │ SGD 3,500 / $2,728          │   ← private rate substituted (correct number)
 │ public n/a → private        │   ← text-muted-foreground, same weight as a caption
 └─────────────────────────────┘
        ▲ cryptic wording (UWT-002); no visual hierarchy (DWT-006);
          and in city-detail the school-foreigner-flag-<id> testid is missing (EWT-003)
```

## After (target)

Desktop table cell and city-detail row, both locales:

```
 School
 ┌─────────────────────────────────────┐
 │ SGD 3,500 / $2,728                  │
 │ ▸ [ Private · public not open to    │   ← Badge, variant=outline, warning/honey hue token
 │     foreigners ]                    │      (text-warning border-warning), reads as a flag
 └─────────────────────────────────────┘
```

- **en** label: `Private — public not open to foreigners` (concise badge form).
- **id** label: `Swasta — negeri tak terbuka untuk WNA`.
- Styling: design-system `Badge` (`variant="outline"`), warning/honey hue token — **not** raw hex, **not**
  `text-muted-foreground`. Distinct hierarchy above ordinary caption text.
- `data-testid="school-foreigner-flag-<cityId>"` present in **both** the table cell (already) and the
  city-detail row (newly added — fixes EWT-003).
- The longer explanatory note above the controls (`foreigner-public-school-note`) stays as-is.

## Responsive

- **Desktop/tablet (≥768px)**: badge sits on its own line under the amount in the School cell.
- **Mobile (<768px)**: the table collapses to cards; the badge renders under the school amount in the card
  body, full-width-wrapping, keeping the warning hue.

## Token usage

`Badge` outline + `text-warning` / `border-warning` (honey hue) · amount stays `text-foreground` · note
stays `text-muted-foreground`. No raw hex.
