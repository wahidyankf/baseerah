export function fmtNum(n: number): string {
  return n.toLocaleString("en-US", { maximumFractionDigits: 0 });
}

export function fmtCurrency(n: number, currency?: string | null): string {
  return `${currency ?? ""} ${fmtNum(n)}`.trim();
}

export function fmtCurrencyTrailing(n: number, code: string): string {
  return `${fmtNum(n)} ${code}`;
}

/**
 * Maps a country's healthcare funding model to a web-ui badge hue — a color-blind-friendly
 * traffic-light progression matching the approved mockups: tax-funded → sage (green, fully
 * covered), mandatory payroll insurance (mixed) → honey (amber, partly covered), out-of-pocket
 * → terracotta (red, you pay it yourself).
 */
export function fmtDualCurrency(localAmount: number, localCurrency: string, usdAmount: number): string {
  return `${fmtCurrency(localAmount, localCurrency)} / $${fmtNum(usdAmount)}`;
}

export function healthcareBadgeHue(type: "oop" | "tax-funded" | "mixed"): "sage" | "honey" | "terracotta" {
  if (type === "tax-funded") return "sage";
  if (type === "mixed") return "honey";
  return "terracotta";
}
