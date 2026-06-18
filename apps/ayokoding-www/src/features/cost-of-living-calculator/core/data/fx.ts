// AUTHORITATIVE FX SNAPSHOT — the single source for every USD conversion in the app.
// Sources: ECB Euro reference rates 2026-06-17, Xe.com mid-market 2026-06-17,
//          x-rates.com cross-check 2026-06-18.
// Each entry is the USD value of 1 unit of that currency.
// A city's USD rate is DERIVED from this table via city.currency — no city stores its own rate.
// fxSnapshotDate may differ from cities/roles snapshotDate (each dataset tracks its own date).

export type FxTable = {
  fxSnapshotDate: string; // ISO date of this FX snapshot
  ratesUsdPerUnit: Record<string, number>; // ISO-4217 -> USD value per 1 unit
};

export const fx: FxTable = {
  fxSnapshotDate: "2026-06-17",
  ratesUsdPerUnit: {
    // Always
    USD: 1.0,

    // ASEAN
    IDR: 0.000056282, // Xe.com 2026-06-17; ~17,768 IDR/USD [high]
    MYR: 0.24598, // Xe.com 2026-06-17 [high]
    SGD: 0.77946, // Xe.com 2026-06-17 [high]
    THB: 0.030727, // Xe.com 2026-06-17 [high]
    VND: 0.000037989, // Xe.com 2026-06-17; ~26,323 VND/USD [high]
    PHP: 0.016556, // Xe.com 2026-06-17 [high]
    KHR: 0.0002489, // Xe.com 2026-06-17; ~4,018 KHR/USD [moderate — thin market]
    LAK: 0.000045412, // Xe.com 2026-06-17; ~22,025 LAK/USD [moderate — thin market]
    MMK: 0.0004762, // Xe.com 2026-06-17; ~2,099 MMK/USD [moderate — parallel market exists]
    BND: 0.77946, // Xe.com 2026-06-17; pegged 1:1 to SGD [high]

    // Japan
    JPY: 0.0062412, // Xe.com 2026-06-17; ECB cross-check 0.006238 [high]

    // Europe (non-Nordic)
    GBP: 1.3396, // Xe.com 2026-06-17; ECB cross-check 1.3404 [high]
    EUR: 1.1591, // ECB reference rate 2026-06-17 [high]
    CHF: 1.2611, // Xe.com 2026-06-17; ECB cross-check 1.2608 [high]
    PLN: 0.27329, // Xe.com 2026-06-17; ECB cross-check 0.27330 [high]
    CZK: 0.048029, // Xe.com 2026-06-17; ECB cross-check 0.048002 [high]

    // Nordics
    SEK: 0.1064, // Xe.com 2026-06-17; ECB cross-check 0.10642 [high]
    DKK: 0.1551, // Xe.com 2026-06-17; ECB cross-check 0.15507 [high]
    NOK: 0.10519, // Xe.com 2026-06-17; ECB cross-check 0.10530 [high]
    ISK: 0.0080281, // Xe.com 2026-06-17; ECB cross-check 0.008027 [high]

    // Americas
    CAD: 0.71272, // Xe.com 2026-06-17 [high]
    MXN: 0.058178, // Xe.com 2026-06-17 [high]
    BRL: 0.19758, // Xe.com 2026-06-17 [high]
    ARS: 0.00069613, // Xe.com 2026-06-17; official BCRA rate [moderate — parallel market exists]
    CLP: 0.0011328, // Xe.com 2026-06-17 [high]

    // Middle East
    AED: 0.27229, // Xe.com 2026-06-17; near-fixed peg ~3.6725 AED/USD [high]

    // South & East Asia
    INR: 0.010598, // Xe.com 2026-06-17; ~94.35 INR/USD [high]
    KRW: 0.00066037, // Xe.com 2026-06-17; ~1,514 KRW/USD [high]
    TWD: 0.031649, // Xe.com 2026-06-17; ~31.6 TWD/USD [high]
    CNY: 0.14797, // Xe.com 2026-06-17; ECB cross-check 0.14795 [high]

    // Oceania
    AUD: 0.70709, // Xe.com 2026-06-17; ECB cross-check 0.70600 [high]

    // Africa
    KES: 0.0077248, // Xe.com 2026-06-17; CBK ~129.5 KES/USD [high]
    NGN: 0.00073597, // Xe.com 2026-06-17; CBN official ~1,359 NGN/USD [high — official rate]
    EGP: 0.020031, // Xe.com 2026-06-17; CBE ~49.92 EGP/USD [high]
  },
};

// ONLY conversion primitive — every *Usd function in calc.ts routes through this.
// Throws if the currency is not in the fx table, rather than returning NaN.
export function fxToUsd(fxTable: FxTable, currency: string): number {
  const rate = fxTable.ratesUsdPerUnit[currency];
  if (rate === undefined) {
    throw new Error(`Currency "${currency}" not found in fx table (fxSnapshotDate: ${fxTable.fxSnapshotDate})`);
  }
  return rate;
}

// Convenience: reads the rate for a city's own currency from the fx table.
export function cityFxToUsd(fxTable: FxTable, city: { currency: string }): number {
  return fxToUsd(fxTable, city.currency);
}

// Convert a USD amount to a chosen display currency.
// result = usd / fxToUsd(fx, displayCurrency)
export function usdToDisplay(fxTable: FxTable, usd: number, displayCurrency: string): number {
  const displayRate = fxToUsd(fxTable, displayCurrency);
  return usd / displayRate;
}
