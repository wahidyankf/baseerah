# Business Requirements — AyoKoding Calculator URL-State Exploratory

## Business Context

The AyoKoding cost-of-living calculator is the flagship interactive tool on ayokoding.com. It is
used by software engineers evaluating relocation options and salary negotiations across tech hubs
in ASEAN, Europe, Americas, and beyond.

The URL state reflection feature (Phase 4 of the calculator delivery plan) enables users to share
a precise calculator configuration via URL — a city deep link, a household setup, a tab — and have
a recipient land in exactly the same view. This is a high-value capability for a tool whose primary
use case is comparison and discussion.

## Who Is Affected

- **Primary users**: Software engineers visiting ayokoding.com to evaluate relocation or salary data.
- **Secondary users**: Readers who receive a shared calculator link from a colleague or from social
  media, and who expect to see the same configuration the sender described.
- **Indonesian-language users**: Users on the `/id/` locale who rely on localized labels throughout
  the UI — including breadcrumb navigation.

## Cost of Leaving Defects Unfixed

The single finding (EWT-001 — breadcrumb "Calculator" label hardcoded in English on the id locale)
affects every Indonesian-language user who navigates the breadcrumb. The breadcrumb is the primary
escape route back to the Tools index and the site home. An English label in an otherwise Indonesian
interface breaks the locale contract and degrades trust in the translation quality of the site.

## Business-Level Success Metrics

- EWT-001 is resolved and verified: the breadcrumb "Calculator" crumb renders in Indonesian on
  `/id/tools/cost-of-living-calculator`.
- All URL-state scenarios (URL-001 through URL-013) remain passing after the fix.
- No regression introduced to the en locale breadcrumb or the id locale calculator content.
