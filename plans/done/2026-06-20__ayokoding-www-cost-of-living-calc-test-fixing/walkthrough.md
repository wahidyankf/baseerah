# Cognitive Walkthrough — Cost-of-Living Calculator (spec-blind)

Source: `web-usability-tester`. A first-time-user transcript captured during the spec-blind pass,
preserved so the friction points behind the `UWT-###` findings are traceable to a concrete task.

**Persona**: Alex — a software developer in Jakarta considering a move to Singapore. First visit, no
prior context, arrives via a direct link. **Goal**: find out whether a Singapore salary covers Alex's
costs.

## Step 1 — Land on the page

- **Q1 (try the right thing?)**: Goal is "find out if a Singapore salary covers my costs." The H1 reads
  "Salary Savings Calculator"; the URL says `cost-of-living-calculator`. Alex hesitates. → **UWT-002**.
  After a beat, decides the page is probably right and continues.
- **Q2 (notice the action?)**: Three tabs ("Cost of living", "Savings", "Minimum role") are visible;
  "Cost of living" is highlighted. Alex recognises tabs. Pass.
- **Q3 (associate the action?)**: "Cost of living" matches Alex's mental model. Stays on this tab. Pass.
- **Q4 (progress visible?)**: A Singapore cost breakdown shows immediately. Pass.

## Step 2 — Select Singapore

- Selects Country → Singapore; City auto-filters to Singapore; the summary card updates.
- Q1–Q4 pass. **Minor friction**: the URL did not update → **UWT-003**; Alex notes the view is not
  bookmarkable.

## Step 3 — Read the total cost

- Reads "Total SGD 4,328" on the card, then scans the table headers: Country, City, Healthcare scheme,
  Housing, Food, Transport, Utilities, Healthcare (OOP), Childcare, School — then nothing more visible.
- **Q3 friction → UWT-004**: At 1280 px the "Total"/"Essentials" columns are off-screen; Alex assumes
  the table only shows the breakdown and the card holds the total. Discovers the summary columns only by
  accidental horizontal scroll.
- **Q4**: Partial — card total visible, table summary hidden.

## Step 4 — Try the "Savings" tab

- Curious whether "Savings" shows how much of a Singapore salary Alex would keep after expenses.
- **Q3 friction → UWT-012**: "Savings" is ambiguous (amount? rate?).
- **Q4 → UWT-001 (conflict-flagged)**: In the spec-blind observation, clicking "Savings" activated the
  tab button but the panel appeared unchanged; Alex concludes "is this broken?"
  **Note for readers**: the exploratory pass _did_ drive the Savings tab successfully (entered a salary,
  used the sort button), so this step's failure is flagged as a probable observation artifact and must
  be re-verified before any tab rewrite — see the cross-reference note in `findings.md`.

## Step 5 — Read "Relocation (sunk)"

- Scrolls right, finds "Relocation (sunk)" and "Liquidity reserve"; does not know what they include.
- **Q3 friction → UWT-005**: "sunk" is finance jargon; no tooltip, no footnote (only "OOP" has one).
  Alex gives up trying to interpret the column.

## Verdict

Steps 1–3 work once the user resolves the H1/URL name conflict. Step 4 is the headline failure in the
spec-blind read (non-functional-appearing tabs), **conflict-flagged** against the exploratory evidence.
Step 5 fails on undefined jargon. The hidden summary columns (UWT-004) mean the single most important
datum — "Total" — requires accidental discovery.
