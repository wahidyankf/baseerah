export function fmtNum(n: number): string {
  return n.toLocaleString("en-US", { maximumFractionDigits: 0 });
}

export function fmtCurrency(n: number, currency?: string | null): string {
  return `${currency ?? ""} ${fmtNum(n)}`.trim();
}

export function fmtCurrencyTrailing(n: number, code: string): string {
  return `${fmtNum(n)} ${code}`;
}
