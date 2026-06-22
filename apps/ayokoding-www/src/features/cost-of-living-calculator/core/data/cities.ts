// CITY + COUNTRY DATASET — tech-hub cities worldwide (excl. Israel), snapshotDate 2026-06-18.
// Sources: Numbeo Jun 2026, PwC Worldwide Tax Summaries 2025, OECD 2025, ECB/Xe.com 2026-06-17.
// FX rates NOT stored here — all USD conversion via fx.ts (city.currency → fxToUsd(fx, currency)).
// Confidence tiers: high = primary source, moderate = secondary / corroborated, proxy = derived.
// To (re)source this data, see the prompts in
// ../../../../../docs/cost-of-living-calculator/data-sourcing-prompt.md

import type { FxTable } from "./fx";
import { fx } from "./fx";

// ─── Types ──────────────────────────────────────────────────────────────────

type Confidence = "high" | "moderate" | "proxy";

type Money = {
  amount: number;
  confidence: Confidence;
  note?: string;
};

type ExpenseCategories = {
  housing: Money;
  food: Money;
  transport: Money;
  utilities: Money;
  healthcare: Money;
  childcare: Money;
  lifestyle: Money;
};

type Relocation = {
  sunkCosts: {
    deposit: Money;
    keyMoney: Money;
    moving: Money;
    visaAdmin: Money;
  };
  liquidityReserve: {
    cashCushion: Money;
  };
};

type IncomeBand = "low" | "mid" | "high";
type Area = "center" | "rural";

export type Household = {
  adults: 1 | 2;
  preschoolKids: 0 | 1 | 2 | 3;
  schoolKids: 0 | 1 | 2 | 3;
};

export type Country = {
  id: string;
  name: { en: string; id: string };
  bandThresholdsUsd: { lowToMid: number; midToHigh: number };
  effectiveRate: Record<IncomeBand, Money>;
  healthcareModelType: "oop" | "tax-funded" | "mixed";
  compulsoryInsurance: {
    health: boolean;
    socialSecurity: boolean;
    note?: string;
  };
};

export type City = {
  id: string;
  name: { en: string; id: string };
  countryId: string;
  currency: string;
  region: "asean" | "japan" | "europe" | "nordics" | "americas" | "mena" | "asia" | "oceania" | "africa";
  expenses: ExpenseCategories;
  childcareMedianLocal: Money;
  schoolMedianLocal: { public: Money; private: Money };
  relocation: Relocation;
  subNational?: {
    name: { en: string; id: string };
    effectiveRate: Record<IncomeBand, Money>;
  };
};

export type Dataset = {
  snapshotDate: string;
  fx: FxTable;
  countries: Country[];
  cities: City[];
};

// ─── OECD Multiplier Helpers ─────────────────────────────────────────────────

// OECD modified equivalence scale: first adult=1.0, additional adult=+0.5, each child=+0.3.
export function equivalisedSize(h: Household): number {
  return 1.0 + 0.5 * (h.adults - 1) + 0.3 * (h.preschoolKids + h.schoolKids);
}

// Damping factor for sub-linear household categories (housing, utilities share economies of scale).
export const SUBLINEAR_DAMPING = 0.5;

// Sub-linear multiplier: housing + utilities grow slower than household size.
export const subLinear = (h: Household): number => 1 + SUBLINEAR_DAMPING * (equivalisedSize(h) - 1);

// Per-capita multiplier: food, healthcare scale with equivalised size.
export const perCapita = (h: Household): number => equivalisedSize(h);

// Area discount for housing outside city center.
export const AREA_MULTIPLIERS: Record<Area, number> = {
  center: 1.0,
  rural: 0.75,
};

// ─── Helper ─────────────────────────────────────────────────────────────────

function m(amount: number, confidence: Confidence, note?: string): Money {
  return { amount, confidence, note };
}

// ─── Countries ──────────────────────────────────────────────────────────────

export const countries: Country[] = [
  // ── ASEAN ──
  {
    id: "sg",
    name: { en: "Singapore", id: "Singapura" },
    bandThresholdsUsd: { lowToMid: 3500, midToHigh: 8000 },
    effectiveRate: {
      low: m(0.04, "high", "PwC 2025: effective ~4% incl. CPF at low band"),
      mid: m(0.1, "high", "PwC 2025: effective ~10% incl. CPF at mid band"),
      high: m(0.17, "high", "PwC 2025: effective ~17% incl. CPF at high band"),
    },
    healthcareModelType: "mixed",
    compulsoryInsurance: {
      health: true,
      socialSecurity: true,
      note: "CPF covers retirement/housing/healthcare; MediShield Life mandatory health insurance",
    },
  },
  {
    id: "th",
    name: { en: "Thailand", id: "Thailand" },
    bandThresholdsUsd: { lowToMid: 2000, midToHigh: 6000 },
    effectiveRate: {
      low: m(0.08, "moderate", "PwC 2025: income tax + SSF ~8% at low band"),
      mid: m(0.18, "moderate", "PwC 2025: ~18% at mid band"),
      high: m(0.31, "moderate", "PwC 2025: ~31% at high band (35% marginal)"),
    },
    healthcareModelType: "mixed",
    compulsoryInsurance: {
      health: true,
      socialSecurity: true,
      note: "SSF (Social Security Fund) mandatory; Universal Coverage Scheme (UCS) public coverage",
    },
  },
  {
    id: "id",
    name: { en: "Indonesia", id: "Indonesia" },
    bandThresholdsUsd: { lowToMid: 1500, midToHigh: 5000 },
    effectiveRate: {
      low: m(0.06, "moderate", "PwC 2025: income tax + BPJS ~6% at low band"),
      mid: m(0.17, "moderate", "PwC 2025: ~17% at mid band"),
      high: m(0.3, "moderate", "PwC 2025: ~30% at high band"),
    },
    healthcareModelType: "mixed",
    compulsoryInsurance: {
      health: true,
      socialSecurity: true,
      note: "BPJS Kesehatan (health) + BPJS Ketenagakerjaan (work) mandatory for employed",
    },
  },
  {
    id: "my",
    name: { en: "Malaysia", id: "Malaysia" },
    bandThresholdsUsd: { lowToMid: 1800, midToHigh: 5500 },
    effectiveRate: {
      low: m(0.05, "high", "PwC 2025: income tax + EPF 11% ~5% effective at low"),
      mid: m(0.17, "high", "PwC 2025: ~17% at mid band"),
      high: m(0.29, "high", "PwC 2025: ~29% at high band"),
    },
    healthcareModelType: "mixed",
    compulsoryInsurance: {
      health: false,
      socialSecurity: true,
      note: "EPF 11% employee + SOCSO mandatory; public health (MOH) not insurance-based",
    },
  },
  {
    id: "vn",
    name: { en: "Vietnam", id: "Vietnam" },
    bandThresholdsUsd: { lowToMid: 1000, midToHigh: 3500 },
    effectiveRate: {
      low: m(0.08, "moderate", "PwC 2025: PIT + VSS ~8% at low band"),
      mid: m(0.18, "moderate", "PwC 2025: ~18% at mid band"),
      high: m(0.3, "moderate", "PwC 2025: ~30% at high band"),
    },
    healthcareModelType: "mixed",
    compulsoryInsurance: {
      health: true,
      socialSecurity: true,
      note: "Vietnamese Social Insurance + Health Insurance (VSS) mandatory for employed",
    },
  },
  {
    id: "ph",
    name: { en: "Philippines", id: "Filipina" },
    bandThresholdsUsd: { lowToMid: 1000, midToHigh: 3500 },
    effectiveRate: {
      low: m(0.07, "moderate", "PwC 2025: TRAIN law PIT + SSS/PhilHealth ~7%"),
      mid: m(0.19, "moderate", "PwC 2025: ~19% at mid band"),
      high: m(0.33, "moderate", "PwC 2025: ~33% at high band"),
    },
    healthcareModelType: "mixed",
    compulsoryInsurance: {
      health: true,
      socialSecurity: true,
      note: "SSS + PhilHealth + Pag-IBIG mandatory contributions for formal employment",
    },
  },
  // ── Japan ──
  {
    id: "jp",
    name: { en: "Japan", id: "Jepang" },
    bandThresholdsUsd: { lowToMid: 3000, midToHigh: 8000 },
    effectiveRate: {
      low: m(0.14, "high", "PwC 2025: national + local + JHI/NPS ~14% at low"),
      mid: m(0.29, "high", "PwC 2025: ~29% at mid band"),
      high: m(0.44, "high", "PwC 2025: ~44% at high band"),
    },
    healthcareModelType: "mixed",
    compulsoryInsurance: {
      health: true,
      socialSecurity: true,
      note: "Shakai hoken (health ~5% employee) + NPS pension mandatory; 70% coverage",
    },
  },
  // ── Europe (non-Nordic) ──
  {
    id: "gb",
    name: { en: "United Kingdom", id: "Inggris" },
    bandThresholdsUsd: { lowToMid: 3500, midToHigh: 8500 },
    effectiveRate: {
      low: m(0.12, "high", "HMRC 2025/26: income tax + NI ~12% at lower band"),
      mid: m(0.28, "high", "HMRC 2025/26: ~28% at mid band"),
      high: m(0.42, "high", "HMRC 2025/26: ~42% at high band"),
    },
    healthcareModelType: "tax-funded",
    compulsoryInsurance: {
      health: false,
      socialSecurity: true,
      note: "NI contributions fund NHS + state pension; NHS free at point of use",
    },
  },
  {
    id: "de",
    name: { en: "Germany", id: "Jerman" },
    bandThresholdsUsd: { lowToMid: 4000, midToHigh: 9000 },
    effectiveRate: {
      low: m(0.2, "high", "PwC 2025: income tax + GKV + pension + care ~20%"),
      mid: m(0.36, "high", "PwC 2025: ~36% at mid band"),
      high: m(0.48, "high", "PwC 2025: ~48% at high band"),
    },
    healthcareModelType: "mixed",
    compulsoryInsurance: {
      health: true,
      socialSecurity: true,
      note: "GKV statutory health (7.3% employee) + pension + care + unemployment mandatory",
    },
  },
  {
    id: "nl",
    name: { en: "Netherlands", id: "Belanda" },
    bandThresholdsUsd: { lowToMid: 4000, midToHigh: 9000 },
    effectiveRate: {
      low: m(0.18, "high", "PwC 2025: income tax box 1 + ZVW + AOW ~18%"),
      mid: m(0.36, "high", "PwC 2025: ~36% at mid band"),
      high: m(0.48, "high", "PwC 2025: ~48% at high band"),
    },
    healthcareModelType: "mixed",
    compulsoryInsurance: {
      health: true,
      socialSecurity: true,
      note: "ZVW mandatory health insurance (~1,700 EUR/yr deductible) + AOW pension via employer",
    },
  },
  {
    id: "pt",
    name: { en: "Portugal", id: "Portugal" },
    bandThresholdsUsd: { lowToMid: 2500, midToHigh: 6500 },
    effectiveRate: {
      low: m(0.14, "high", "PwC 2025: IRS + Segurança Social 11% ~14%"),
      mid: m(0.28, "high", "PwC 2025: ~28% at mid band"),
      high: m(0.43, "high", "PwC 2025: ~43% at high band"),
    },
    healthcareModelType: "tax-funded",
    compulsoryInsurance: {
      health: false,
      socialSecurity: true,
      note: "Segurança Social 11% employee; SNS (Serviço Nacional de Saúde) tax-funded",
    },
  },
  {
    id: "ch",
    name: { en: "Switzerland", id: "Swiss" },
    bandThresholdsUsd: { lowToMid: 5000, midToHigh: 12000 },
    effectiveRate: {
      low: m(0.12, "high", "EFD 2025: federal income tax + AHV ~12% at low"),
      mid: m(0.23, "high", "EFD 2025: ~23% at mid band"),
      high: m(0.36, "high", "EFD 2025: ~36% at high band (incl. canton avg)"),
    },
    healthcareModelType: "mixed",
    compulsoryInsurance: {
      health: true,
      socialSecurity: true,
      note: "LAMal individual health insurance mandatory (~400-500 CHF/mo); AHV/AVS pension mandatory",
    },
  },
  {
    id: "pl",
    name: { en: "Poland", id: "Polandia" },
    bandThresholdsUsd: { lowToMid: 2000, midToHigh: 6000 },
    effectiveRate: {
      low: m(0.1, "high", "PwC 2025: PIT + ZUS health 9% + pension ~10%"),
      mid: m(0.21, "high", "PwC 2025: ~21% at mid band"),
      high: m(0.33, "high", "PwC 2025: ~33% at high band"),
    },
    healthcareModelType: "mixed",
    compulsoryInsurance: {
      health: true,
      socialSecurity: true,
      note: "ZUS: NFZ health 9% + pension + disability + sick pay mandatory",
    },
  },
  {
    id: "cz",
    name: { en: "Czech Republic", id: "Ceko" },
    bandThresholdsUsd: { lowToMid: 2000, midToHigh: 5500 },
    effectiveRate: {
      low: m(0.11, "high", "PwC 2025: PIT + health 4.5% + social 6.5% ~11%"),
      mid: m(0.22, "high", "PwC 2025: ~22% at mid band"),
      high: m(0.34, "high", "PwC 2025: ~34% at high band"),
    },
    healthcareModelType: "mixed",
    compulsoryInsurance: {
      health: true,
      socialSecurity: true,
      note: "Health insurance 4.5% + social security 6.5% employee mandatory",
    },
  },
  {
    id: "fr",
    name: { en: "France", id: "Prancis" },
    bandThresholdsUsd: { lowToMid: 3500, midToHigh: 9000 },
    effectiveRate: {
      low: m(0.22, "high", "PwC 2025: IR + CSG/CRDS + sécu ~22% at low"),
      mid: m(0.39, "high", "PwC 2025: ~39% at mid band"),
      high: m(0.53, "high", "PwC 2025: ~53% at high band"),
    },
    healthcareModelType: "mixed",
    compulsoryInsurance: {
      health: true,
      socialSecurity: true,
      note: "URSSAF: CSG 9.2% + health + pension mandatory; Assurance maladie covers 70-80%",
    },
  },
  // ── Nordics ──
  {
    id: "se",
    name: { en: "Sweden", id: "Swedia" },
    bandThresholdsUsd: { lowToMid: 3500, midToHigh: 8000 },
    effectiveRate: {
      low: m(0.29, "high", "Skatteverket 2025: kommunalskatt + statlig ~29%"),
      mid: m(0.39, "high", "Skatteverket 2025: ~39% at mid band"),
      high: m(0.53, "high", "Skatteverket 2025: ~53% at high band"),
    },
    healthcareModelType: "tax-funded",
    compulsoryInsurance: {
      health: false,
      socialSecurity: true,
      note: "Employer pays social contributions; minimal employee pension fee; health tax-funded",
    },
  },
  {
    id: "dk",
    name: { en: "Denmark", id: "Denmark" },
    bandThresholdsUsd: { lowToMid: 4000, midToHigh: 9000 },
    effectiveRate: {
      low: m(0.3, "high", "SKAT 2025: bundskat + kommuneskat ~30%"),
      mid: m(0.41, "high", "SKAT 2025: ~41% at mid band"),
      high: m(0.56, "high", "SKAT 2025: ~56% at high band"),
    },
    healthcareModelType: "tax-funded",
    compulsoryInsurance: {
      health: false,
      socialSecurity: false,
      note: "Entirely tax-funded (ATP pension minimal); no separate mandatory employee health contribution",
    },
  },
  {
    id: "no",
    name: { en: "Norway", id: "Norwegia" },
    bandThresholdsUsd: { lowToMid: 4000, midToHigh: 9000 },
    effectiveRate: {
      low: m(0.22, "high", "Skatteetaten 2025: income tax + NI 7.9% ~22%"),
      mid: m(0.34, "high", "Skatteetaten 2025: ~34% at mid band"),
      high: m(0.46, "high", "Skatteetaten 2025: ~46% at high band"),
    },
    healthcareModelType: "tax-funded",
    compulsoryInsurance: {
      health: false,
      socialSecurity: true,
      note: "National insurance 7.9% employee; Helseforetak health tax-funded via general taxes",
    },
  },
  {
    id: "fi",
    name: { en: "Finland", id: "Finlandia" },
    bandThresholdsUsd: { lowToMid: 3500, midToHigh: 8000 },
    effectiveRate: {
      low: m(0.18, "high", "Vero 2025: income tax + Kela 1.53% + pension 7.15% ~18%"),
      mid: m(0.31, "high", "Vero 2025: ~31% at mid band"),
      high: m(0.48, "high", "Vero 2025: ~48% at high band"),
    },
    healthcareModelType: "tax-funded",
    compulsoryInsurance: {
      health: false,
      socialSecurity: true,
      note: "Kela: 1.53% health + 7.15% pension employee mandatory; health primarily tax-funded",
    },
  },
  // ── Americas ──
  {
    id: "us",
    name: { en: "United States", id: "Amerika Serikat" },
    bandThresholdsUsd: { lowToMid: 3500, midToHigh: 9000 },
    effectiveRate: {
      low: m(0.15, "high", "IRS 2025: federal income tax + FICA 7.65% ~15%"),
      mid: m(0.26, "high", "IRS 2025: ~26% at mid band"),
      high: m(0.36, "high", "IRS 2025: ~36% at high band (37% marginal)"),
    },
    healthcareModelType: "mixed",
    compulsoryInsurance: {
      health: false,
      socialSecurity: true,
      note: "FICA: 6.2% SS + 1.45% Medicare mandatory; employer health near-universal for tech",
    },
  },
  {
    id: "ca",
    name: { en: "Canada", id: "Kanada" },
    bandThresholdsUsd: { lowToMid: 3500, midToHigh: 8000 },
    effectiveRate: {
      low: m(0.18, "high", "CRA 2025: federal + CPP 5.95% + EI 1.66% ~18%"),
      mid: m(0.29, "high", "CRA 2025: ~29% at mid band"),
      high: m(0.4, "high", "CRA 2025: ~40% at high band"),
    },
    healthcareModelType: "tax-funded",
    compulsoryInsurance: {
      health: false,
      socialSecurity: true,
      note: "CPP 5.95% + EI 1.66% employee mandatory; provincial health (OHIP/MSP) tax-funded",
    },
  },
  {
    id: "br",
    name: { en: "Brazil", id: "Brasil" },
    bandThresholdsUsd: { lowToMid: 1500, midToHigh: 4500 },
    effectiveRate: {
      low: m(0.1, "moderate", "PwC 2025: IRPF + INSS 7.5% ~10% at low band"),
      mid: m(0.23, "moderate", "PwC 2025: ~23% at mid band"),
      high: m(0.35, "moderate", "PwC 2025: ~35% at high band"),
    },
    healthcareModelType: "mixed",
    compulsoryInsurance: {
      health: false,
      socialSecurity: true,
      note: "INSS 7.5-14% employee; SUS public health exists; private plano dominates middle class",
    },
  },
  {
    id: "mx",
    name: { en: "Mexico", id: "Meksiko" },
    bandThresholdsUsd: { lowToMid: 1500, midToHigh: 4000 },
    effectiveRate: {
      low: m(0.08, "moderate", "PwC 2025: ISR + IMSS employee ~8% at low"),
      mid: m(0.21, "moderate", "PwC 2025: ~21% at mid band"),
      high: m(0.33, "moderate", "PwC 2025: ~33% at high band"),
    },
    healthcareModelType: "mixed",
    compulsoryInsurance: {
      health: true,
      socialSecurity: true,
      note: "IMSS health + pension contributions mandatory for formal employment",
    },
  },
  // ── Middle East ──
  {
    id: "ae",
    name: { en: "United Arab Emirates", id: "Uni Emirat Arab" },
    bandThresholdsUsd: { lowToMid: 5000, midToHigh: 15000 },
    effectiveRate: {
      low: m(0.0, "high", "UAE: no personal income tax"),
      mid: m(0.0, "high", "UAE: no personal income tax"),
      high: m(0.0, "high", "UAE: no personal income tax"),
    },
    healthcareModelType: "mixed",
    compulsoryInsurance: {
      health: true,
      socialSecurity: false,
      note: "Employer mandatory health insurance (DHA/HAAD regulation); no public pension for expats",
    },
  },
  // ── South & East Asia (non-ASEAN) ──
  {
    id: "in",
    name: { en: "India", id: "India" },
    bandThresholdsUsd: { lowToMid: 1000, midToHigh: 3500 },
    effectiveRate: {
      low: m(0.05, "moderate", "ITD 2025: new tax regime ~5% at low band"),
      mid: m(0.2, "moderate", "ITD 2025: ~20% at mid band"),
      high: m(0.33, "moderate", "ITD 2025: ~33% at high band"),
    },
    healthcareModelType: "mixed",
    compulsoryInsurance: {
      health: false,
      socialSecurity: true,
      note: "EPF 12% employee; ESI for lower income; tech workers typically above ESI threshold",
    },
  },
  {
    id: "kr",
    name: { en: "South Korea", id: "Korea Selatan" },
    bandThresholdsUsd: { lowToMid: 2500, midToHigh: 7000 },
    effectiveRate: {
      low: m(0.1, "high", "NTS 2025: income tax + NHI 3.545% + NPS 4.5% ~10%"),
      mid: m(0.23, "high", "NTS 2025: ~23% at mid band"),
      high: m(0.38, "high", "NTS 2025: ~38% at high band"),
    },
    healthcareModelType: "mixed",
    compulsoryInsurance: {
      health: true,
      socialSecurity: true,
      note: "NHI (NHIS) 3.545% employee + NPS 4.5% mandatory; 60-70% coverage",
    },
  },
  // ── Oceania ──
  {
    id: "au",
    name: { en: "Australia", id: "Australia" },
    bandThresholdsUsd: { lowToMid: 3500, midToHigh: 8500 },
    effectiveRate: {
      low: m(0.19, "high", "ATO 2025: income tax + Medicare Levy 2% ~19%"),
      mid: m(0.33, "high", "ATO 2025: ~33% at mid band"),
      high: m(0.47, "high", "ATO 2025: ~47% at high band"),
    },
    healthcareModelType: "tax-funded",
    compulsoryInsurance: {
      health: false,
      socialSecurity: true,
      note: "Superannuation 11.5% employer-paid; Medicare Levy 2% employee; Medicare tax-funded",
    },
  },
  // ── Africa ──
  {
    id: "ke",
    name: { en: "Kenya", id: "Kenya" },
    bandThresholdsUsd: { lowToMid: 800, midToHigh: 2500 },
    effectiveRate: {
      low: m(0.1, "moderate", "KRA 2025: PAYE + SHIF + NSSF ~10% at low"),
      mid: m(0.22, "moderate", "KRA 2025: ~22% at mid band"),
      high: m(0.31, "moderate", "KRA 2025: ~31% at high band"),
    },
    healthcareModelType: "mixed",
    compulsoryInsurance: {
      health: true,
      socialSecurity: true,
      note: "SHIF (Social Health Insurance Fund) + NSSF mandatory; private insurance common in formal sector",
    },
  },
];

// ─── Cities ──────────────────────────────────────────────────────────────────

export const cities: City[] = [
  // ══════════════════════════════════════════
  // ASEAN
  // ══════════════════════════════════════════
  {
    id: "singapore",
    name: { en: "Singapore", id: "Singapura" },
    countryId: "sg",
    currency: "SGD",
    region: "asean",
    expenses: {
      // 1BR city-center; Numbeo Jun 2026
      housing: m(3500, "high", "Numbeo Jun 2026: 1BR city center ~3,300–3,700 SGD/mo"),
      food: m(400, "high", "Numbeo Jun 2026: mid-range resto + groceries ~400 SGD/mo"),
      transport: m(128, "high", "LTA 2026: monthly concession/adult MRT+bus pass ~128 SGD"),
      utilities: m(180, "high", "Numbeo Jun 2026: elec+water+gas ~170–190 SGD/mo"),
      healthcare: m(
        120,
        "moderate",
        "OOP only: GP copay + dental avg ~120 SGD/mo; MediShield premiums in effectiveRate",
      ),
      childcare: m(1500, "high", "MOE 2026: private preschool ~1,200–1,800 SGD/mo per child"),
      lifestyle: m(250, "moderate", "Numbeo Jun 2026: gym+entertainment+clothing ~250 SGD/mo"),
    },
    childcareMedianLocal: m(1500, "high", "MOE 2026 median private preschool"),
    schoolMedianLocal: {
      public: m(150, "high", "MOE 2026: primary school miscellaneous fees ~150 SGD/mo"),
      private: m(3500, "moderate", "Numbeo Jun 2026: international school ~3,000–4,000 SGD/mo"),
    },
    relocation: {
      sunkCosts: {
        deposit: m(7000, "moderate", "2× month rent; standard for Singapore private rental"),
        keyMoney: m(0, "high", "N/A: no key money custom in Singapore"),
        moving: m(3500, "moderate", "International shipping + local transport estimate"),
        visaAdmin: m(600, "moderate", "EP/S Pass application + dependant pass fees ~600 SGD"),
      },
      liquidityReserve: {
        cashCushion: m(15000, "moderate", "3× monthly essentials (~4,300 SGD/mo) = ~13,000; rounded up"),
      },
    },
  },
  {
    id: "bangkok",
    name: { en: "Bangkok", id: "Bangkok" },
    countryId: "th",
    currency: "THB",
    region: "asean",
    expenses: {
      housing: m(25000, "high", "Numbeo Jun 2026: 1BR city center ~22,000–28,000 THB/mo"),
      food: m(8000, "high", "Numbeo Jun 2026: restaurants + groceries ~8,000 THB/mo"),
      transport: m(1500, "high", "BTS/MRT monthly pass ~1,500 THB/mo"),
      utilities: m(2500, "high", "Numbeo Jun 2026: elec+water+internet ~2,500 THB/mo"),
      healthcare: m(2000, "moderate", "OOP: clinic visit + meds; SSF covers formal employed"),
      childcare: m(15000, "moderate", "Expat preschool estimate ~12,000–18,000 THB/mo"),
      lifestyle: m(4000, "moderate", "Numbeo Jun 2026: gym+social+clothing ~4,000 THB/mo"),
    },
    childcareMedianLocal: m(15000, "moderate", "Expat private preschool Bangkok median"),
    schoolMedianLocal: {
      public: m(1000, "moderate", "Thai public school misc fees ~1,000 THB/mo (non-citizens pay more)"),
      private: m(40000, "moderate", "International school Bangkok ~35,000–45,000 THB/mo"),
    },
    relocation: {
      sunkCosts: {
        deposit: m(50000, "moderate", "2× month rent typical"),
        keyMoney: m(0, "high", "N/A: no key money in Thailand"),
        moving: m(30000, "moderate", "International shipping estimate in THB"),
        visaAdmin: m(10000, "moderate", "Non-immigrant B visa + work permit ~10,000 THB"),
      },
      liquidityReserve: {
        cashCushion: m(100000, "moderate", "3× monthly essentials (~33,000 THB/mo)"),
      },
    },
  },
  {
    id: "jakarta",
    name: { en: "Jakarta", id: "Jakarta" },
    countryId: "id",
    currency: "IDR",
    region: "asean",
    expenses: {
      housing: m(8000000, "high", "Numbeo Jun 2026: 1BR city center ~7M–9M IDR/mo"),
      food: m(3000000, "high", "Numbeo Jun 2026: restaurants + warung + groceries ~3M IDR/mo"),
      transport: m(700000, "high", "MRT + TransJakarta monthly ~500K; car-free assumption"),
      utilities: m(1500000, "high", "Numbeo Jun 2026: elec+water+internet ~1.5M IDR/mo"),
      healthcare: m(500000, "moderate", "OOP: clinic; BPJS Kesehatan covers formal employed"),
      childcare: m(4000000, "moderate", "Expat/bilingual preschool ~3.5M–5M IDR/mo"),
      lifestyle: m(2000000, "moderate", "Numbeo Jun 2026: gym+social ~2M IDR/mo"),
    },
    childcareMedianLocal: m(4000000, "moderate", "Jakarta bilingual preschool median"),
    schoolMedianLocal: {
      public: m(200000, "moderate", "Indonesian public school misc fees ~200K IDR/mo"),
      private: m(10000000, "moderate", "International school Jakarta ~8M–12M IDR/mo"),
    },
    relocation: {
      sunkCosts: {
        deposit: m(16000000, "moderate", "2× month rent"),
        keyMoney: m(0, "high", "N/A: no key money in Indonesia"),
        moving: m(10000000, "moderate", "International shipping estimate in IDR"),
        visaAdmin: m(5000000, "moderate", "KITAS work visa processing ~5M IDR"),
      },
      liquidityReserve: {
        cashCushion: m(30000000, "moderate", "3× monthly essentials (~10M IDR/mo)"),
      },
    },
  },
  {
    id: "kuala-lumpur",
    name: { en: "Kuala Lumpur", id: "Kuala Lumpur" },
    countryId: "my",
    currency: "MYR",
    region: "asean",
    expenses: {
      housing: m(2500, "high", "Numbeo Jun 2026: 1BR city center ~2,200–2,800 MYR/mo"),
      food: m(800, "high", "Numbeo Jun 2026: hawker + groceries ~800 MYR/mo"),
      transport: m(150, "high", "Rapid KL monthly pass ~150 MYR"),
      utilities: m(250, "high", "Numbeo Jun 2026: elec+water+internet ~250 MYR/mo"),
      healthcare: m(200, "moderate", "OOP: private clinic; public subsidized but queues"),
      childcare: m(1200, "moderate", "Private preschool KL ~1,000–1,500 MYR/mo"),
      lifestyle: m(400, "moderate", "Numbeo Jun 2026: gym+social ~400 MYR/mo"),
    },
    childcareMedianLocal: m(1200, "moderate", "KL private preschool median"),
    schoolMedianLocal: {
      public: m(100, "moderate", "Malaysian public school misc fees ~100 MYR/mo"),
      private: m(3000, "moderate", "International school KL ~2,500–3,500 MYR/mo"),
    },
    relocation: {
      sunkCosts: {
        deposit: m(5000, "moderate", "2× month rent"),
        keyMoney: m(0, "high", "N/A"),
        moving: m(3000, "moderate", "International shipping estimate MYR"),
        visaAdmin: m(2000, "moderate", "Employment pass fees ~2,000 MYR"),
      },
      liquidityReserve: {
        cashCushion: m(9000, "moderate", "3× monthly essentials (~3,000 MYR/mo)"),
      },
    },
  },
  {
    id: "ho-chi-minh-city",
    name: { en: "Ho Chi Minh City", id: "Ho Chi Minh City" },
    countryId: "vn",
    currency: "VND",
    region: "asean",
    expenses: {
      housing: m(15000000, "high", "Numbeo Jun 2026: 1BR city center ~13M–17M VND/mo"),
      food: m(5000000, "high", "Numbeo Jun 2026: restaurants + pho + groceries ~5M VND/mo"),
      transport: m(500000, "high", "Grab monthly + metro pass estimate ~500K VND"),
      utilities: m(1500000, "high", "Numbeo Jun 2026: elec+water+internet ~1.5M VND/mo"),
      healthcare: m(500000, "moderate", "OOP: private clinic visit; VSS covers formal"),
      childcare: m(8000000, "moderate", "Expat preschool HCMC ~7M–9M VND/mo"),
      lifestyle: m(2000000, "moderate", "Numbeo Jun 2026: gym+social ~2M VND/mo"),
    },
    childcareMedianLocal: m(8000000, "moderate", "HCMC expat preschool median"),
    schoolMedianLocal: {
      public: m(500000, "moderate", "Vietnamese public school misc fees"),
      private: m(20000000, "moderate", "International school HCMC ~18M–22M VND/mo"),
    },
    relocation: {
      sunkCosts: {
        deposit: m(30000000, "moderate", "2× month rent"),
        keyMoney: m(0, "high", "N/A"),
        moving: m(15000000, "moderate", "International shipping estimate VND"),
        visaAdmin: m(5000000, "moderate", "Work permit + TRC fees ~5M VND"),
      },
      liquidityReserve: {
        cashCushion: m(60000000, "moderate", "3× monthly essentials (~20M VND/mo)"),
      },
    },
  },
  {
    id: "manila",
    name: { en: "Manila", id: "Manila" },
    countryId: "ph",
    currency: "PHP",
    region: "asean",
    expenses: {
      housing: m(35000, "high", "Numbeo Jun 2026: 1BR BGC/Makati ~30,000–40,000 PHP/mo"),
      food: m(12000, "high", "Numbeo Jun 2026: restaurants + tindahan + groceries ~12K PHP/mo"),
      transport: m(1500, "high", "MRT + jeepney monthly estimate ~1,500 PHP"),
      utilities: m(5000, "high", "Numbeo Jun 2026: elec+water+internet ~5,000 PHP/mo"),
      healthcare: m(3000, "moderate", "OOP: private clinic; PhilHealth covers some"),
      childcare: m(15000, "moderate", "Private preschool Manila ~12K–18K PHP/mo"),
      lifestyle: m(6000, "moderate", "Numbeo Jun 2026: gym+social ~6,000 PHP/mo"),
    },
    childcareMedianLocal: m(15000, "moderate", "Manila private preschool median"),
    schoolMedianLocal: {
      public: m(500, "moderate", "Philippine public school misc fees ~500 PHP/mo"),
      private: m(20000, "moderate", "International school Manila ~18K–22K PHP/mo"),
    },
    relocation: {
      sunkCosts: {
        deposit: m(70000, "moderate", "2× month rent"),
        keyMoney: m(0, "high", "N/A"),
        moving: m(40000, "moderate", "International shipping estimate PHP"),
        visaAdmin: m(10000, "moderate", "9(g) work visa fees ~10,000 PHP"),
      },
      liquidityReserve: {
        cashCushion: m(150000, "moderate", "3× monthly essentials (~50K PHP/mo)"),
      },
    },
  },
  // ══════════════════════════════════════════
  // Japan
  // ══════════════════════════════════════════
  {
    id: "tokyo",
    name: { en: "Tokyo", id: "Tokyo" },
    countryId: "jp",
    currency: "JPY",
    region: "japan",
    expenses: {
      housing: m(130000, "high", "Numbeo Jun 2026: 1BR city center ~120K–140K JPY/mo"),
      food: m(50000, "high", "Numbeo Jun 2026: restaurants + supermarket ~50K JPY/mo"),
      transport: m(15000, "high", "Tokyo Metro commuter pass estimate ~15K JPY/mo"),
      utilities: m(20000, "high", "Numbeo Jun 2026: elec+gas+water ~20K JPY/mo"),
      healthcare: m(10000, "moderate", "OOP copay only (30%); JHI premiums inside effectiveRate"),
      childcare: m(50000, "moderate", "Private hoikuen ~40K–60K JPY/mo; public wait-listed"),
      lifestyle: m(30000, "moderate", "Numbeo Jun 2026: gym+izakaya+clothing ~30K JPY/mo"),
    },
    childcareMedianLocal: m(50000, "moderate", "Tokyo private hoikuen/preschool median"),
    schoolMedianLocal: {
      public: m(3000, "high", "MEXT 2026: public school misc fees ~3,000 JPY/mo"),
      private: m(80000, "moderate", "Private/international school Tokyo ~70K–90K JPY/mo"),
    },
    relocation: {
      sunkCosts: {
        deposit: m(260000, "high", "2× rent typical for Tokyo shikikin"),
        keyMoney: m(130000, "high", "1× rent reikin (non-refundable) — still common in central Tokyo"),
        moving: m(100000, "moderate", "International + local move estimate JPY"),
        visaAdmin: m(50000, "moderate", "Engineer/Specialist Humanities visa COE + agent fees"),
      },
      liquidityReserve: {
        cashCushion: m(600000, "moderate", "3× monthly essentials (~200K JPY/mo)"),
      },
    },
  },
  {
    id: "osaka",
    name: { en: "Osaka", id: "Osaka" },
    countryId: "jp",
    currency: "JPY",
    region: "japan",
    expenses: {
      housing: m(100000, "high", "Numbeo Jun 2026: 1BR city center ~90K–110K JPY/mo"),
      food: m(45000, "high", "Numbeo Jun 2026: Osaka food culture slightly cheaper than Tokyo"),
      transport: m(10000, "high", "Osaka Metro monthly pass estimate ~10K JPY"),
      utilities: m(18000, "high", "Numbeo Jun 2026: elec+gas+water ~18K JPY/mo"),
      healthcare: m(8000, "moderate", "OOP copay 30%; JHI premiums in effectiveRate"),
      childcare: m(45000, "moderate", "Private hoikuen Osaka ~40K–50K JPY/mo"),
      lifestyle: m(25000, "moderate", "Numbeo Jun 2026: gym+social ~25K JPY/mo"),
    },
    childcareMedianLocal: m(45000, "moderate", "Osaka private hoikuen median"),
    schoolMedianLocal: {
      public: m(3000, "high", "MEXT 2026: public school misc fees ~3,000 JPY/mo"),
      private: m(70000, "moderate", "Private school Osaka ~60K–80K JPY/mo"),
    },
    relocation: {
      sunkCosts: {
        deposit: m(200000, "high", "2× rent shikikin Osaka"),
        keyMoney: m(100000, "high", "1× rent reikin — still encountered in central Osaka"),
        moving: m(80000, "moderate", "International + local move estimate JPY"),
        visaAdmin: m(50000, "moderate", "Engineer visa COE + agent fees"),
      },
      liquidityReserve: {
        cashCushion: m(480000, "moderate", "3× monthly essentials (~160K JPY/mo)"),
      },
    },
  },
  // ══════════════════════════════════════════
  // Europe (non-Nordic)
  // ══════════════════════════════════════════
  {
    id: "london",
    name: { en: "London", id: "London" },
    countryId: "gb",
    currency: "GBP",
    region: "europe",
    expenses: {
      housing: m(2200, "high", "Numbeo Jun 2026: 1BR zone 1-2 ~2,000–2,400 GBP/mo"),
      food: m(500, "high", "Numbeo Jun 2026: restaurants + Tesco ~500 GBP/mo"),
      transport: m(180, "high", "TfL 2026: monthly zones 1-2 Travelcard ~180 GBP"),
      utilities: m(250, "high", "Numbeo Jun 2026: elec+gas+water ~240–260 GBP/mo"),
      healthcare: m(50, "high", "OOP: prescriptions ~£9.90 each + dental; NHS free at point of use"),
      childcare: m(1400, "high", "Numbeo Jun 2026: private nursery ~1,200–1,600 GBP/mo"),
      lifestyle: m(300, "moderate", "Numbeo Jun 2026: gym+pub+clothing ~300 GBP/mo"),
    },
    childcareMedianLocal: m(1400, "high", "London private nursery median"),
    schoolMedianLocal: {
      public: m(50, "high", "State school misc extras ~50 GBP/mo; tuition free"),
      private: m(2000, "moderate", "Independent day school London ~1,800–2,200 GBP/mo"),
    },
    relocation: {
      sunkCosts: {
        deposit: m(4400, "moderate", "2× month rent deposit (tenancy deposit scheme)"),
        keyMoney: m(0, "high", "N/A: banned under Tenant Fees Act 2019"),
        moving: m(1500, "moderate", "International shipping + van hire estimate"),
        visaAdmin: m(1200, "moderate", "Skilled Worker visa + IHS surcharge (year 1) ~1,200 GBP"),
      },
      liquidityReserve: {
        cashCushion: m(9000, "moderate", "3× monthly essentials (~3,000 GBP/mo)"),
      },
    },
  },
  {
    id: "berlin",
    name: { en: "Berlin", id: "Berlin" },
    countryId: "de",
    currency: "EUR",
    region: "europe",
    expenses: {
      housing: m(1500, "high", "Numbeo Jun 2026: 1BR Mitte/Prenzlauer Berg ~1,400–1,600 EUR/mo"),
      food: m(400, "high", "Numbeo Jun 2026: restaurants + Rewe ~400 EUR/mo"),
      transport: m(86, "high", "Deutschlandticket 2026: 86 EUR/mo nationwide"),
      utilities: m(200, "high", "Numbeo Jun 2026: elec+gas+water ~190–210 EUR/mo"),
      healthcare: m(30, "high", "OOP: copay minimal; GKV covers most; prescription copay 5-10 EUR"),
      childcare: m(200, "high", "Berlin Kita: max ~200 EUR/mo (income-based subsidy)"),
      lifestyle: m(250, "moderate", "Numbeo Jun 2026: gym+bar+clothing ~250 EUR/mo"),
    },
    childcareMedianLocal: m(200, "high", "Berlin Kita income-based fee median"),
    schoolMedianLocal: {
      public: m(0, "high", "German state school: free; misc ~20 EUR/mo"),
      private: m(1500, "moderate", "Private/international school Berlin ~1,200–1,800 EUR/mo"),
    },
    relocation: {
      sunkCosts: {
        deposit: m(4500, "moderate", "3× rent Kaution; BGB max 3 months"),
        keyMoney: m(0, "high", "N/A: prohibited under German tenancy law"),
        moving: m(1500, "moderate", "International shipping estimate EUR"),
        visaAdmin: m(500, "moderate", "EU Blue Card or Niederlassungserlaubnis fees ~500 EUR"),
      },
      liquidityReserve: {
        cashCushion: m(6000, "moderate", "3× monthly essentials (~2,000 EUR/mo)"),
      },
    },
  },
  {
    id: "amsterdam",
    name: { en: "Amsterdam", id: "Amsterdam" },
    countryId: "nl",
    currency: "EUR",
    region: "europe",
    expenses: {
      housing: m(2000, "high", "Numbeo Jun 2026: 1BR city center ~1,800–2,200 EUR/mo"),
      food: m(450, "high", "Numbeo Jun 2026: restaurants + Albert Heijn ~450 EUR/mo"),
      transport: m(100, "high", "GVB/NS monthly equivalent ~100 EUR/mo"),
      utilities: m(220, "high", "Numbeo Jun 2026: elec+gas+water ~210–230 EUR/mo"),
      healthcare: m(100, "high", "OOP: ZVW deductible ~385 EUR/yr split ~32/mo + copays"),
      childcare: m(1000, "high", "After kinderopvangtoeslag subsidy ~900–1,100 EUR/mo"),
      lifestyle: m(300, "moderate", "Numbeo Jun 2026: gym+café+cycling ~300 EUR/mo"),
    },
    childcareMedianLocal: m(1000, "high", "Amsterdam kinderopvang net median"),
    schoolMedianLocal: {
      public: m(0, "high", "Dutch public school: free; misc ~25 EUR/mo"),
      private: m(1600, "moderate", "International school Amsterdam ~1,400–1,800 EUR/mo"),
    },
    relocation: {
      sunkCosts: {
        deposit: m(4000, "moderate", "2× rent deposit typical"),
        keyMoney: m(0, "high", "N/A"),
        moving: m(1500, "moderate", "International shipping estimate EUR"),
        visaAdmin: m(500, "moderate", "GVVA / highly skilled migrant permit ~500 EUR"),
      },
      liquidityReserve: {
        cashCushion: m(8500, "moderate", "3× monthly essentials (~2,850 EUR/mo)"),
      },
    },
  },
  {
    id: "lisbon",
    name: { en: "Lisbon", id: "Lisbon" },
    countryId: "pt",
    currency: "EUR",
    region: "europe",
    expenses: {
      housing: m(1400, "high", "Numbeo Jun 2026: 1BR city center ~1,200–1,600 EUR/mo"),
      food: m(350, "high", "Numbeo Jun 2026: tasca + Pingo Doce groceries ~350 EUR/mo"),
      transport: m(40, "high", "Carris/Metro Navegante pass 2026: 40 EUR/mo"),
      utilities: m(150, "high", "Numbeo Jun 2026: elec+water+internet ~150 EUR/mo"),
      healthcare: m(50, "high", "OOP: SNS copay (moderadora) ~5 EUR/visit + dental"),
      childcare: m(500, "moderate", "Public crèche ~150 EUR; private ~500 EUR/mo"),
      lifestyle: m(200, "moderate", "Numbeo Jun 2026: pastel+gym+clothing ~200 EUR/mo"),
    },
    childcareMedianLocal: m(500, "moderate", "Lisbon private crèche median"),
    schoolMedianLocal: {
      public: m(0, "high", "Portuguese public school: free; misc ~15 EUR/mo"),
      private: m(800, "moderate", "Private school Lisbon ~700–1,000 EUR/mo"),
    },
    relocation: {
      sunkCosts: {
        deposit: m(2800, "moderate", "2× rent deposit"),
        keyMoney: m(0, "high", "N/A"),
        moving: m(1200, "moderate", "International shipping estimate EUR"),
        visaAdmin: m(300, "moderate", "D2/D3/D8 visa + SEF/AIMA appointment fees ~300 EUR"),
      },
      liquidityReserve: {
        cashCushion: m(5500, "moderate", "3× monthly essentials (~1,850 EUR/mo)"),
      },
    },
  },
  {
    id: "zurich",
    name: { en: "Zurich", id: "Zürich" },
    countryId: "ch",
    currency: "CHF",
    region: "europe",
    expenses: {
      housing: m(2800, "high", "Numbeo Jun 2026: 1BR city center ~2,600–3,000 CHF/mo"),
      food: m(700, "high", "Numbeo Jun 2026: restaurant + Migros groceries ~700 CHF/mo"),
      transport: m(92, "high", "ZVV 2026: Zone 110 monthly Abo ~92 CHF/mo"),
      utilities: m(300, "high", "Numbeo Jun 2026: elec+heat+water ~290–310 CHF/mo"),
      healthcare: m(150, "high", "OOP copay only: Franchise split ~90/mo + 10% copay; LAMal premium in effectiveRate"),
      childcare: m(1800, "high", "Numbeo Jun 2026: Kita Zurich ~1,600–2,000 CHF/mo"),
      lifestyle: m(400, "moderate", "Numbeo Jun 2026: gym+café+recreation ~400 CHF/mo"),
    },
    childcareMedianLocal: m(1800, "high", "Zurich Kita median 2026"),
    schoolMedianLocal: {
      public: m(200, "high", "Swiss public school misc fees ~200 CHF/mo"),
      private: m(2500, "moderate", "International school Zurich ~2,200–2,800 CHF/mo"),
    },
    relocation: {
      sunkCosts: {
        deposit: m(8400, "high", "3× rent deposit; OR 3 months + guarantee; max per OR 720"),
        keyMoney: m(0, "high", "N/A: no key money in Switzerland"),
        moving: m(3000, "moderate", "International shipping + Swiss moving company estimate"),
        visaAdmin: m(1000, "moderate", "B permit (EU) or L permit processing + relocation admin"),
      },
      liquidityReserve: {
        cashCushion: m(12000, "moderate", "3× monthly essentials (~4,000 CHF/mo)"),
      },
    },
    subNational: {
      name: { en: "Canton of Zurich", id: "Kanton Zürich" },
      effectiveRate: {
        low: m(0.04, "high", "Kanton Zürich Steueramt 2025: cantonal+communal ~4% at low"),
        mid: m(0.08, "high", "Kanton Zürich: ~8% at mid band"),
        high: m(0.12, "high", "Kanton Zürich: ~12% at high band"),
      },
    },
  },
  {
    id: "warsaw",
    name: { en: "Warsaw", id: "Warsawa" },
    countryId: "pl",
    currency: "PLN",
    region: "europe",
    expenses: {
      housing: m(3500, "high", "Numbeo Jun 2026: 1BR city center ~3,200–3,800 PLN/mo"),
      food: m(1200, "high", "Numbeo Jun 2026: restaurant + Biedronka groceries ~1,200 PLN/mo"),
      transport: m(120, "high", "ZTM Warsaw monthly pass 2026: 120 PLN/mo"),
      utilities: m(600, "high", "Numbeo Jun 2026: elec+gas+water ~580–620 PLN/mo"),
      healthcare: m(200, "moderate", "OOP: private clinic queue-avoidance; NFZ covers basics"),
      childcare: m(1500, "moderate", "Private żłobek Warsaw ~1,200–1,800 PLN/mo"),
      lifestyle: m(600, "moderate", "Numbeo Jun 2026: gym+social ~600 PLN/mo"),
    },
    childcareMedianLocal: m(1500, "moderate", "Warsaw private nursery median"),
    schoolMedianLocal: {
      public: m(0, "high", "Polish public school: free; misc ~20 PLN/mo"),
      private: m(2000, "moderate", "International school Warsaw ~1,800–2,200 PLN/mo"),
    },
    relocation: {
      sunkCosts: {
        deposit: m(7000, "moderate", "2× rent deposit"),
        keyMoney: m(0, "high", "N/A"),
        moving: m(4000, "moderate", "International shipping estimate PLN"),
        visaAdmin: m(1500, "moderate", "Work permit + residence card fees ~1,500 PLN"),
      },
      liquidityReserve: {
        cashCushion: m(15000, "moderate", "3× monthly essentials (~5,000 PLN/mo)"),
      },
    },
  },
  {
    id: "prague",
    name: { en: "Prague", id: "Praha" },
    countryId: "cz",
    currency: "CZK",
    region: "europe",
    expenses: {
      housing: m(20000, "high", "Numbeo Jun 2026: 1BR city center ~18K–22K CZK/mo"),
      food: m(7000, "high", "Numbeo Jun 2026: restaurant + Albert groceries ~7,000 CZK/mo"),
      transport: m(550, "high", "DPP Prague monthly pass 2026: 550 CZK/mo"),
      utilities: m(4000, "high", "Numbeo Jun 2026: elec+gas+water ~3,800–4,200 CZK/mo"),
      healthcare: m(500, "moderate", "OOP: private dental + some specialists; VZP covers GP"),
      childcare: m(8000, "moderate", "Private školka Prague ~7,000–9,000 CZK/mo"),
      lifestyle: m(4000, "moderate", "Numbeo Jun 2026: gym+social ~4,000 CZK/mo"),
    },
    childcareMedianLocal: m(8000, "moderate", "Prague private nursery median"),
    schoolMedianLocal: {
      public: m(0, "high", "Czech public school: free; misc ~100 CZK/mo"),
      private: m(15000, "moderate", "International school Prague ~13K–17K CZK/mo"),
    },
    relocation: {
      sunkCosts: {
        deposit: m(40000, "moderate", "2× rent deposit"),
        keyMoney: m(0, "high", "N/A"),
        moving: m(18000, "moderate", "International shipping estimate CZK"),
        visaAdmin: m(5000, "moderate", "Employee card fees ~5,000 CZK"),
      },
      liquidityReserve: {
        cashCushion: m(90000, "moderate", "3× monthly essentials (~30K CZK/mo)"),
      },
    },
  },
  {
    id: "paris",
    name: { en: "Paris", id: "Paris" },
    countryId: "fr",
    currency: "EUR",
    region: "europe",
    expenses: {
      housing: m(1800, "high", "Numbeo Jun 2026: 1BR inside Périphérique ~1,600–2,000 EUR/mo"),
      food: m(500, "high", "Numbeo Jun 2026: brasserie + Carrefour City ~500 EUR/mo"),
      transport: m(86, "high", "Île-de-France Mobilités 2026: Navigo all-zones ~86 EUR/mo"),
      utilities: m(200, "high", "Numbeo Jun 2026: elec+gas+water ~190–210 EUR/mo"),
      healthcare: m(30, "high", "OOP: ticket modérateur residual after Sécu 70% + mutuelles"),
      childcare: m(600, "high", "Crèche municipal + complémentaire ~500–700 EUR/mo"),
      lifestyle: m(300, "moderate", "Numbeo Jun 2026: café+gym+clothing ~300 EUR/mo"),
    },
    childcareMedianLocal: m(600, "high", "Paris crèche net median 2026"),
    schoolMedianLocal: {
      public: m(0, "high", "Éducation nationale: free; misc ~20 EUR/mo"),
      private: m(1200, "moderate", "Private/international school Paris ~1,000–1,400 EUR/mo"),
    },
    relocation: {
      sunkCosts: {
        deposit: m(3600, "moderate", "2× rent deposit typical; max 2 months by law"),
        keyMoney: m(0, "high", "N/A: droit au bail only in commercial; residential illegal"),
        moving: m(1500, "moderate", "International shipping + déménageurs estimate"),
        visaAdmin: m(400, "moderate", "Passeport Talent / EU citizenship admin fees ~400 EUR"),
      },
      liquidityReserve: {
        cashCushion: m(8000, "moderate", "3× monthly essentials (~2,650 EUR/mo)"),
      },
    },
  },
  // ══════════════════════════════════════════
  // Nordics
  // ══════════════════════════════════════════
  {
    id: "stockholm",
    name: { en: "Stockholm", id: "Stockholm" },
    countryId: "se",
    currency: "SEK",
    region: "nordics",
    expenses: {
      housing: m(15000, "high", "Numbeo Jun 2026: 1BR Södermalm/Östermalm ~14K–16K SEK/mo"),
      food: m(4000, "high", "Numbeo Jun 2026: restaurant + ICA Maxi ~4,000 SEK/mo"),
      transport: m(950, "high", "SL 2026: monthly travel card zones A-B ~950 SEK/mo"),
      utilities: m(1500, "high", "Numbeo Jun 2026: elec+heat+water ~1,400–1,600 SEK/mo"),
      healthcare: m(250, "high", "OOP: max 1,200 SEK/yr (high-cost protection); ~100 SEK/mo avg"),
      childcare: m(1750, "high", "Maxtaxa 2026: max 1st child ~1,750 SEK/mo"),
      lifestyle: m(2500, "moderate", "Numbeo Jun 2026: gym+fika+social ~2,500 SEK/mo"),
    },
    childcareMedianLocal: m(1750, "high", "Stockholm maxtaxa preschool max fee"),
    schoolMedianLocal: {
      public: m(0, "high", "Swedish public school: free; misc ~50 SEK/mo"),
      private: m(4000, "moderate", "Friskola (private school) Stockholm ~3,500–4,500 SEK/mo"),
    },
    relocation: {
      sunkCosts: {
        deposit: m(30000, "moderate", "2× rent deposit"),
        keyMoney: m(0, "high", "N/A: black market queues exist but not licit costs"),
        moving: m(12000, "moderate", "International shipping estimate SEK"),
        visaAdmin: m(3500, "moderate", "Work permit (Migrationsverket) ~3,500 SEK"),
      },
      liquidityReserve: {
        cashCushion: m(65000, "moderate", "3× monthly essentials (~21,500 SEK/mo)"),
      },
    },
  },
  {
    id: "copenhagen",
    name: { en: "Copenhagen", id: "Kopenhagen" },
    countryId: "dk",
    currency: "DKK",
    region: "nordics",
    expenses: {
      housing: m(12000, "high", "Numbeo Jun 2026: 1BR Nørrebro/Frederiksberg ~11K–13K DKK/mo"),
      food: m(4500, "high", "Numbeo Jun 2026: smørrebrød + Netto groceries ~4,500 DKK/mo"),
      transport: m(500, "high", "Rejsekort monthly equivalent zones 1-2 ~500 DKK"),
      utilities: m(1500, "high", "Numbeo Jun 2026: elec+heat+water ~1,400–1,600 DKK/mo"),
      healthcare: m(200, "high", "OOP: dental + private physio; GP/hospital free under sygesikring"),
      childcare: m(2500, "high", "Dagtilbud 2026: max-rate estimate ~2,500 DKK/mo"),
      lifestyle: m(3000, "moderate", "Numbeo Jun 2026: hygge+gym+social ~3,000 DKK/mo"),
    },
    childcareMedianLocal: m(2500, "high", "Copenhagen dagpasning median rate"),
    schoolMedianLocal: {
      public: m(0, "high", "Danish folkeskole: free; misc ~30 DKK/mo"),
      private: m(5000, "moderate", "International school Copenhagen ~4,500–5,500 DKK/mo"),
    },
    relocation: {
      sunkCosts: {
        deposit: m(24000, "moderate", "2× rent deposit"),
        keyMoney: m(0, "high", "N/A"),
        moving: m(10000, "moderate", "International shipping estimate DKK"),
        visaAdmin: m(4000, "moderate", "Work permit (Styrelsen) ~4,000 DKK"),
      },
      liquidityReserve: {
        cashCushion: m(60000, "moderate", "3× monthly essentials (~20K DKK/mo)"),
      },
    },
  },
  {
    id: "oslo",
    name: { en: "Oslo", id: "Oslo" },
    countryId: "no",
    currency: "NOK",
    region: "nordics",
    expenses: {
      housing: m(16000, "high", "Numbeo Jun 2026: 1BR city center ~15K–17K NOK/mo"),
      food: m(6000, "high", "Numbeo Jun 2026: restaurants + Rema 1000 ~6,000 NOK/mo"),
      transport: m(850, "high", "Ruter 2026: 30-day Oslo pass ~850 NOK/mo"),
      utilities: m(2000, "high", "Numbeo Jun 2026: elec+heat+water ~1,900–2,100 NOK/mo"),
      healthcare: m(300, "high", "OOP: egenandel max ~3,000 NOK/yr ≈ 250/mo; GP free after"),
      childcare: m(3500, "high", "Makspris 2026: 3,315 NOK/mo per child (full day)"),
      lifestyle: m(4000, "moderate", "Numbeo Jun 2026: tur+aktivitet+social ~4,000 NOK/mo"),
    },
    childcareMedianLocal: m(3500, "high", "Oslo barnehage makspris 2026"),
    schoolMedianLocal: {
      public: m(0, "high", "Norwegian grunnskole: free; misc ~50 NOK/mo"),
      private: m(8000, "moderate", "International school Oslo ~7,000–9,000 NOK/mo"),
    },
    relocation: {
      sunkCosts: {
        deposit: m(32000, "moderate", "2× rent deposit"),
        keyMoney: m(0, "high", "N/A"),
        moving: m(14000, "moderate", "International shipping estimate NOK"),
        visaAdmin: m(4500, "moderate", "Skilled worker permit (UDI) ~4,500 NOK"),
      },
      liquidityReserve: {
        cashCushion: m(75000, "moderate", "3× monthly essentials (~25K NOK/mo)"),
      },
    },
  },
  {
    id: "helsinki",
    name: { en: "Helsinki", id: "Helsinki" },
    countryId: "fi",
    currency: "EUR",
    region: "nordics",
    expenses: {
      housing: m(1400, "high", "Numbeo Jun 2026: 1BR Kallio/Kamppi ~1,250–1,550 EUR/mo"),
      food: m(400, "high", "Numbeo Jun 2026: ravintola + K-Market ~400 EUR/mo"),
      transport: m(62, "high", "HSL 2026: Zone AB monthly ~62 EUR/mo"),
      utilities: m(180, "high", "Numbeo Jun 2026: elec+heat+water ~170–190 EUR/mo"),
      healthcare: m(50, "high", "OOP: terveyskeskusmaksu ~14 EUR/visit; Kela refunds some"),
      childcare: m(300, "high", "Päivähoito 2026: income-based max ~300 EUR/mo"),
      lifestyle: m(250, "moderate", "Numbeo Jun 2026: sauna+gym+social ~250 EUR/mo"),
    },
    childcareMedianLocal: m(300, "high", "Helsinki päivähoito income-based median"),
    schoolMedianLocal: {
      public: m(0, "high", "Finnish peruskoulu: free; misc ~20 EUR/mo"),
      private: m(1000, "moderate", "International school Helsinki ~800–1,200 EUR/mo"),
    },
    relocation: {
      sunkCosts: {
        deposit: m(2800, "moderate", "2× rent deposit"),
        keyMoney: m(0, "high", "N/A"),
        moving: m(1500, "moderate", "International shipping estimate EUR"),
        visaAdmin: m(500, "moderate", "Residence permit (Migri) ~500 EUR"),
      },
      liquidityReserve: {
        cashCushion: m(6000, "moderate", "3× monthly essentials (~2,000 EUR/mo)"),
      },
    },
  },
  // ══════════════════════════════════════════
  // Americas
  // ══════════════════════════════════════════
  {
    id: "san-francisco",
    name: { en: "San Francisco", id: "San Francisco" },
    countryId: "us",
    currency: "USD",
    region: "americas",
    expenses: {
      housing: m(3500, "high", "Numbeo Jun 2026: 1BR SoMa/SOMA/Mission ~3,200–3,800 USD/mo"),
      food: m(700, "high", "Numbeo Jun 2026: restaurant + Trader Joe's ~700 USD/mo"),
      transport: m(120, "high", "SFMTA/BART 2026: monthly pass ~120 USD/mo"),
      utilities: m(200, "high", "Numbeo Jun 2026: elec+gas+water+internet ~200 USD/mo"),
      healthcare: m(450, "moderate", "OOP: employee premium share ~200 + avg copay+deductible ~250 USD/mo"),
      childcare: m(2800, "high", "Numbeo Jun 2026: private daycare SF ~2,500–3,100 USD/mo"),
      lifestyle: m(500, "moderate", "Numbeo Jun 2026: gym+dining+clothing ~500 USD/mo"),
    },
    childcareMedianLocal: m(2800, "high", "SF private daycare median 2026"),
    schoolMedianLocal: {
      public: m(0, "high", "SFUSD public school: free; misc ~30 USD/mo"),
      private: m(3000, "moderate", "Private school SF ~2,500–3,500 USD/mo"),
    },
    relocation: {
      sunkCosts: {
        deposit: m(7000, "moderate", "2× rent deposit; SF max 2 months"),
        keyMoney: m(0, "high", "N/A: not customary in US rentals"),
        moving: m(4000, "moderate", "International shipping + domestic moving estimate"),
        visaAdmin: m(3000, "moderate", "H-1B filing fees + attorney estimate ~3,000 USD"),
      },
      liquidityReserve: {
        cashCushion: m(18000, "moderate", "3× monthly essentials (~6,000 USD/mo)"),
      },
    },
    subNational: {
      name: { en: "California", id: "California" },
      effectiveRate: {
        low: m(0.01, "high", "FTB 2025: CA state income tax effective ~1% at low band"),
        mid: m(0.04, "high", "FTB 2025: ~4% at mid band"),
        high: m(0.08, "high", "FTB 2025: ~8% at high band (up to 13.3% marginal)"),
      },
    },
  },
  {
    id: "new-york",
    name: { en: "New York", id: "New York" },
    countryId: "us",
    currency: "USD",
    region: "americas",
    expenses: {
      housing: m(4000, "high", "Numbeo Jun 2026: 1BR Manhattan/Brooklyn ~3,600–4,400 USD/mo"),
      food: m(700, "high", "Numbeo Jun 2026: restaurant + Whole Foods/Trader Joe's ~700 USD/mo"),
      transport: m(133, "high", "MTA 2026: monthly unlimited MetroCard ~133 USD/mo"),
      utilities: m(220, "high", "Numbeo Jun 2026: elec+gas+water+internet ~210–230 USD/mo"),
      healthcare: m(450, "moderate", "OOP: employee premium share + copays ~450 USD/mo"),
      childcare: m(3000, "high", "Numbeo Jun 2026: NYC private daycare ~2,800–3,200 USD/mo"),
      lifestyle: m(500, "moderate", "Numbeo Jun 2026: gym+dining+clothing ~500 USD/mo"),
    },
    childcareMedianLocal: m(3000, "high", "NYC private daycare median 2026"),
    schoolMedianLocal: {
      public: m(0, "high", "NYC public school: free; misc ~30 USD/mo"),
      private: m(3500, "moderate", "Independent school NYC ~3,000–4,000 USD/mo"),
    },
    relocation: {
      sunkCosts: {
        deposit: m(8000, "moderate", "2× rent deposit; NY max 1 month but common 2"),
        keyMoney: m(0, "high", "N/A"),
        moving: m(4000, "moderate", "International shipping + moving estimate"),
        visaAdmin: m(3000, "moderate", "H-1B filing + legal fees estimate ~3,000 USD"),
      },
      liquidityReserve: {
        cashCushion: m(20000, "moderate", "3× monthly essentials (~6,500 USD/mo)"),
      },
    },
    subNational: {
      name: { en: "New York State + City", id: "New York State + City" },
      effectiveRate: {
        low: m(0.04, "high", "NYSDTF 2025: NY state ~4% + NYC city ~3.8% effective at low"),
        mid: m(0.08, "high", "NYSDTF 2025: ~8% combined at mid band"),
        high: m(0.12, "high", "NYSDTF 2025: ~12% combined at high band"),
      },
    },
  },
  {
    id: "austin",
    name: { en: "Austin", id: "Austin" },
    countryId: "us",
    currency: "USD",
    region: "americas",
    expenses: {
      housing: m(2000, "high", "Numbeo Jun 2026: 1BR downtown Austin ~1,800–2,200 USD/mo"),
      food: m(500, "high", "Numbeo Jun 2026: Tex-Mex + HEB groceries ~500 USD/mo"),
      transport: m(50, "high", "CapMetro 2026: monthly pass ~50 USD/mo (car-free assumption)"),
      utilities: m(180, "high", "Numbeo Jun 2026: elec (ERCOT) + water + internet ~180 USD/mo"),
      healthcare: m(400, "moderate", "OOP: employee premium share + copays ~400 USD/mo"),
      childcare: m(1800, "high", "Numbeo Jun 2026: Austin daycare ~1,600–2,000 USD/mo"),
      lifestyle: m(350, "moderate", "Numbeo Jun 2026: gym+BBQ+outdoor ~350 USD/mo"),
    },
    childcareMedianLocal: m(1800, "high", "Austin private daycare median 2026"),
    schoolMedianLocal: {
      public: m(0, "high", "AISD public school: free; misc ~25 USD/mo"),
      private: m(1500, "moderate", "Private school Austin ~1,200–1,800 USD/mo"),
    },
    relocation: {
      sunkCosts: {
        deposit: m(4000, "moderate", "2× rent deposit typical"),
        keyMoney: m(0, "high", "N/A"),
        moving: m(3500, "moderate", "International shipping + domestic moving estimate"),
        visaAdmin: m(3000, "moderate", "H-1B or TN visa fees + legal ~3,000 USD"),
      },
      liquidityReserve: {
        cashCushion: m(12000, "moderate", "3× monthly essentials (~4,000 USD/mo)"),
      },
    },
    subNational: {
      name: { en: "Texas", id: "Texas" },
      effectiveRate: {
        low: m(0.0, "high", "Texas: no state income tax"),
        mid: m(0.0, "high", "Texas: no state income tax"),
        high: m(0.0, "high", "Texas: no state income tax"),
      },
    },
  },
  {
    id: "toronto",
    name: { en: "Toronto", id: "Toronto" },
    countryId: "ca",
    currency: "CAD",
    region: "americas",
    expenses: {
      housing: m(2500, "high", "Numbeo Jun 2026: 1BR downtown Toronto ~2,300–2,700 CAD/mo"),
      food: m(700, "high", "Numbeo Jun 2026: restaurant + Loblaws ~700 CAD/mo"),
      transport: m(156, "high", "TTC 2026: monthly pass ~156 CAD/mo"),
      utilities: m(200, "high", "Numbeo Jun 2026: Hydro One + water + internet ~200 CAD/mo"),
      healthcare: m(100, "high", "OOP: OHIP covers GP/hospital; dental+vision ~100 CAD/mo"),
      childcare: m(1000, "high", "$10/day federal program Ontario 2026 ~1,000 CAD/mo per child"),
      lifestyle: m(400, "moderate", "Numbeo Jun 2026: gym+patio+clothing ~400 CAD/mo"),
    },
    childcareMedianLocal: m(1000, "high", "Ontario $10/day CWELCC median 2026"),
    schoolMedianLocal: {
      public: m(0, "high", "TDSB public school: free; misc ~20 CAD/mo"),
      private: m(2500, "moderate", "Private school Toronto ~2,000–3,000 CAD/mo"),
    },
    relocation: {
      sunkCosts: {
        deposit: m(5000, "moderate", "2× rent deposit typical"),
        keyMoney: m(0, "high", "N/A"),
        moving: m(2500, "moderate", "International shipping + moving estimate CAD"),
        visaAdmin: m(2000, "moderate", "LMIA-exempt permit or PR application fees ~2,000 CAD"),
      },
      liquidityReserve: {
        cashCushion: m(12000, "moderate", "3× monthly essentials (~4,000 CAD/mo)"),
      },
    },
    subNational: {
      name: { en: "Ontario", id: "Ontario" },
      effectiveRate: {
        low: m(0.05, "high", "CRA/Ontario 2025: Ontario provincial income tax effective ~5% at low"),
        mid: m(0.09, "high", "CRA/Ontario 2025: ~9% at mid band"),
        high: m(0.13, "high", "CRA/Ontario 2025: ~13% at high band"),
      },
    },
  },
  {
    id: "sao-paulo",
    name: { en: "São Paulo", id: "São Paulo" },
    countryId: "br",
    currency: "BRL",
    region: "americas",
    expenses: {
      housing: m(3500, "high", "Numbeo Jun 2026: 1BR Paulista/Vila Madalena ~3,000–4,000 BRL/mo"),
      food: m(1500, "high", "Numbeo Jun 2026: restaurant + Pão de Açúcar ~1,500 BRL/mo"),
      transport: m(250, "high", "SPTrans/Metrô 2026: monthly pass ~250 BRL/mo"),
      utilities: m(600, "high", "Numbeo Jun 2026: elec+water+internet ~580–620 BRL/mo"),
      healthcare: m(800, "moderate", "OOP: plano de saúde copay + dental; SUS public but queued"),
      childcare: m(2000, "moderate", "Private escola infantil SP ~1,500–2,500 BRL/mo"),
      lifestyle: m(1000, "moderate", "Numbeo Jun 2026: gym+restaurante+social ~1,000 BRL/mo"),
    },
    childcareMedianLocal: m(2000, "moderate", "São Paulo private preschool median"),
    schoolMedianLocal: {
      public: m(0, "high", "Brazilian public school: free; misc ~30 BRL/mo"),
      private: m(3000, "moderate", "Private escola São Paulo ~2,500–3,500 BRL/mo"),
    },
    relocation: {
      sunkCosts: {
        deposit: m(7000, "moderate", "2× rent deposit"),
        keyMoney: m(0, "high", "N/A"),
        moving: m(8000, "moderate", "International shipping estimate BRL"),
        visaAdmin: m(3000, "moderate", "VITEM II work visa + registration fees ~3,000 BRL"),
      },
      liquidityReserve: {
        cashCushion: m(18000, "moderate", "3× monthly essentials (~6,000 BRL/mo)"),
      },
    },
  },
  {
    id: "mexico-city",
    name: { en: "Mexico City", id: "Kota Meksiko" },
    countryId: "mx",
    currency: "MXN",
    region: "americas",
    expenses: {
      housing: m(15000, "high", "Numbeo Jun 2026: 1BR Condesa/Roma/Polanco ~13K–17K MXN/mo"),
      food: m(5000, "high", "Numbeo Jun 2026: restaurant + Superama/Chedraui ~5,000 MXN/mo"),
      transport: m(500, "high", "Metro CDMX monthly estimate ~500 MXN/mo"),
      utilities: m(1500, "high", "Numbeo Jun 2026: elec+gas+water+internet ~1,500 MXN/mo"),
      healthcare: m(2000, "moderate", "OOP: private clinic + meds; IMSS covers formal employed"),
      childcare: m(8000, "moderate", "Private guardería CDMX ~7,000–9,000 MXN/mo"),
      lifestyle: m(3000, "moderate", "Numbeo Jun 2026: gym+tacos+social ~3,000 MXN/mo"),
    },
    childcareMedianLocal: m(8000, "moderate", "CDMX private preschool median"),
    schoolMedianLocal: {
      public: m(0, "high", "Mexican public school: free; misc ~100 MXN/mo"),
      private: m(12000, "moderate", "Private school CDMX ~10K–14K MXN/mo"),
    },
    relocation: {
      sunkCosts: {
        deposit: m(30000, "moderate", "2× rent deposit"),
        keyMoney: m(0, "high", "N/A"),
        moving: m(15000, "moderate", "International shipping estimate MXN"),
        visaAdmin: m(5000, "moderate", "FM3 work permit fees ~5,000 MXN"),
      },
      liquidityReserve: {
        cashCushion: m(72000, "moderate", "3× monthly essentials (~24K MXN/mo)"),
      },
    },
  },
  // ══════════════════════════════════════════
  // Others (MENA, South Asia, East Asia, Oceania, Africa)
  // ══════════════════════════════════════════
  {
    id: "dubai",
    name: { en: "Dubai", id: "Dubai" },
    countryId: "ae",
    currency: "AED",
    region: "mena",
    expenses: {
      housing: m(8000, "high", "Numbeo Jun 2026: 1BR Downtown/JBR/DIFC ~7,000–9,000 AED/mo"),
      food: m(1500, "high", "Numbeo Jun 2026: restaurant + Carrefour ~1,500 AED/mo"),
      transport: m(250, "high", "RTA 2026: monthly NOL card transit ~250 AED/mo"),
      utilities: m(800, "high", "Numbeo Jun 2026: DEWA elec+water+cooling+internet ~800 AED/mo"),
      healthcare: m(600, "moderate", "OOP: copay/deductible on mandatory employer health plan; DHA"),
      childcare: m(3000, "moderate", "Private nursery Dubai ~2,500–3,500 AED/mo"),
      lifestyle: m(1500, "moderate", "Numbeo Jun 2026: gym+brunch+social ~1,500 AED/mo"),
    },
    childcareMedianLocal: m(3000, "moderate", "Dubai private nursery median 2026"),
    schoolMedianLocal: {
      public: m(1000, "proxy", "KHDA public school: mainly Emirati nationals; expat proxy fee ~1,000 AED/mo"),
      private: m(3500, "high", "KHDA 2026: private school Dubai ~3,000–4,000 AED/mo"),
    },
    relocation: {
      sunkCosts: {
        deposit: m(16000, "moderate", "2× rent deposit; Dubai standard"),
        keyMoney: m(0, "high", "N/A: no key money custom in UAE"),
        moving: m(6000, "moderate", "International shipping estimate AED"),
        visaAdmin: m(2500, "moderate", "Employment visa + Emirates ID + medical ~2,500 AED"),
      },
      liquidityReserve: {
        cashCushion: m(36000, "moderate", "3× monthly essentials (~12K AED/mo)"),
      },
    },
  },
  {
    id: "bengaluru",
    name: { en: "Bengaluru", id: "Bangalore" },
    countryId: "in",
    currency: "INR",
    region: "asia",
    expenses: {
      housing: m(35000, "high", "Numbeo Jun 2026: 1BR Koramangala/Indiranagar ~30K–40K INR/mo"),
      food: m(12000, "high", "Numbeo Jun 2026: restaurant + Nature's Basket ~12K INR/mo"),
      transport: m(1500, "high", "Namma Metro monthly pass + auto estimate ~1,500 INR/mo"),
      utilities: m(3000, "high", "Numbeo Jun 2026: BESCOM elec+water+internet ~3,000 INR/mo"),
      healthcare: m(3000, "moderate", "OOP: private Apollo/Fortis clinic; ESI not applicable at tech salaries"),
      childcare: m(20000, "moderate", "Private play school Bengaluru ~15K–25K INR/mo"),
      lifestyle: m(7000, "moderate", "Numbeo Jun 2026: gym+dining+social ~7,000 INR/mo"),
    },
    childcareMedianLocal: m(20000, "moderate", "Bengaluru private preschool median"),
    schoolMedianLocal: {
      public: m(500, "moderate", "Govt school misc fees ~500 INR/mo"),
      private: m(15000, "moderate", "CBSE/ICSE private school Bengaluru ~12K–18K INR/mo"),
    },
    relocation: {
      sunkCosts: {
        deposit: m(105000, "moderate", "3× rent deposit typical in Bengaluru"),
        keyMoney: m(0, "high", "N/A"),
        moving: m(40000, "moderate", "International shipping estimate INR"),
        visaAdmin: m(15000, "moderate", "Employment visa fees + FRO registration ~15K INR"),
      },
      liquidityReserve: {
        cashCushion: m(165000, "moderate", "3× monthly essentials (~55K INR/mo)"),
      },
    },
  },
  {
    id: "seoul",
    name: { en: "Seoul", id: "Seoul" },
    countryId: "kr",
    currency: "KRW",
    region: "asia",
    expenses: {
      // Jeonse (lump-sum lease) complicates rent; using monthly rent (wolse) equivalent
      housing: m(1500000, "high", "Numbeo Jun 2026: 1BR Gangnam/Mapo wolse ~1.3M–1.7M KRW/mo"),
      food: m(500000, "high", "Numbeo Jun 2026: restaurant + E-Mart groceries ~500K KRW/mo"),
      transport: m(100000, "high", "T-money monthly Seoul metro/bus ~100K KRW/mo"),
      utilities: m(200000, "high", "Numbeo Jun 2026: KEPCO elec+gas+water ~200K KRW/mo"),
      healthcare: m(100000, "moderate", "OOP copay: 30% patient share; NHI premiums in effectiveRate"),
      childcare: m(500000, "moderate", "Private English preschool Seoul ~400K–600K KRW/mo"),
      lifestyle: m(400000, "moderate", "Numbeo Jun 2026: gym+pojangmacha+social ~400K KRW/mo"),
    },
    childcareMedianLocal: m(500000, "moderate", "Seoul private preschool (yeong-eo) median"),
    schoolMedianLocal: {
      public: m(50000, "moderate", "Korean public school misc fees ~50K KRW/mo"),
      private: m(1000000, "moderate", "Private hagwon + international school Seoul ~1M KRW/mo"),
    },
    relocation: {
      sunkCosts: {
        deposit: m(3000000, "moderate", "2× rent deposit; or partial jeonse lump sum substitute"),
        keyMoney: m(0, "high", "N/A: jeonse is lump-sum deposit returned, not key money"),
        moving: m(1500000, "moderate", "International shipping estimate KRW"),
        visaAdmin: m(500000, "moderate", "D-8 or E-7 visa fees + alien registration ~500K KRW"),
      },
      liquidityReserve: {
        cashCushion: m(8000000, "moderate", "3× monthly essentials (~2.6M KRW/mo)"),
      },
    },
  },
  {
    id: "sydney",
    name: { en: "Sydney", id: "Sydney" },
    countryId: "au",
    currency: "AUD",
    region: "oceania",
    expenses: {
      housing: m(3000, "high", "Numbeo Jun 2026: 1BR CBD/Inner West ~2,700–3,300 AUD/mo"),
      food: m(700, "high", "Numbeo Jun 2026: restaurant + Woolworths ~700 AUD/mo"),
      transport: m(180, "high", "Opal monthly cap equivalent zones 1-3 ~180 AUD/mo"),
      utilities: m(250, "high", "Numbeo Jun 2026: elec+gas+water+internet ~240–260 AUD/mo"),
      healthcare: m(100, "high", "OOP: dental + physio gap; Medicare covers GP bulk-billing"),
      childcare: m(1800, "high", "Numbeo Jun 2026: Sydney daycare ~1,600–2,000 AUD/mo (after CCS subsidy)"),
      lifestyle: m(400, "moderate", "Numbeo Jun 2026: gym+café+outdoor ~400 AUD/mo"),
    },
    childcareMedianLocal: m(1800, "high", "Sydney daycare median after CCS 2026"),
    schoolMedianLocal: {
      public: m(200, "high", "NSW public school voluntary contribution ~200 AUD/mo"),
      private: m(2000, "moderate", "Independent school Sydney ~1,700–2,300 AUD/mo"),
    },
    relocation: {
      sunkCosts: {
        deposit: m(6000, "moderate", "2× rent deposit; NSW bond max 4 weeks"),
        keyMoney: m(0, "high", "N/A"),
        moving: m(3000, "moderate", "International shipping estimate AUD"),
        visaAdmin: m(2000, "moderate", "TSS 482 or Skilled visa application fees ~2,000 AUD"),
      },
      liquidityReserve: {
        cashCushion: m(13000, "moderate", "3× monthly essentials (~4,300 AUD/mo)"),
      },
    },
  },
  {
    id: "nairobi",
    name: { en: "Nairobi", id: "Nairobi" },
    countryId: "ke",
    currency: "KES",
    region: "africa",
    expenses: {
      housing: m(80000, "high", "Numbeo Jun 2026: 1BR Westlands/Kilimani ~70K–90K KES/mo"),
      food: m(30000, "high", "Numbeo Jun 2026: restaurant + Naivas/Carrefour ~30K KES/mo"),
      transport: m(3000, "high", "Matatu/BRT monthly estimate ~3,000 KES/mo"),
      utilities: m(8000, "high", "Numbeo Jun 2026: KPLC elec+water+internet ~8,000 KES/mo"),
      healthcare: m(10000, "moderate", "OOP: private hospital; SHIF covers some; private plans common"),
      childcare: m(40000, "moderate", "Private preschool Nairobi ~35K–45K KES/mo"),
      lifestyle: m(15000, "moderate", "Numbeo Jun 2026: gym+social+nyama choma ~15K KES/mo"),
    },
    childcareMedianLocal: m(40000, "moderate", "Nairobi private preschool median"),
    schoolMedianLocal: {
      public: m(2000, "moderate", "Kenyan public school fees ~2,000 KES/mo"),
      private: m(60000, "moderate", "International school Nairobi ~50K–70K KES/mo"),
    },
    relocation: {
      sunkCosts: {
        deposit: m(160000, "moderate", "2× rent deposit"),
        keyMoney: m(0, "high", "N/A"),
        moving: m(60000, "moderate", "International shipping estimate KES"),
        visaAdmin: m(20000, "moderate", "Work permit + special pass fees ~20K KES"),
      },
      liquidityReserve: {
        cashCushion: m(360000, "moderate", "3× monthly essentials (~120K KES/mo)"),
      },
    },
  },
];

// ─── Dataset Export ──────────────────────────────────────────────────────────

export const dataset: Dataset = {
  snapshotDate: "2026-06-18",
  fx,
  countries,
  cities,
};
