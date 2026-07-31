import { AppFrame } from "./AppFrame";

export interface AppShellProps {
  greeting: string;
}

export function AppShell({ greeting }: AppShellProps) {
  return (
    <AppFrame>
      <p className="text-muted-foreground max-w-prose text-lg">
        Baseerah is a personal operating layer — an AI assistant, a content builder, a posting helper, and a workflow
        engine in one.
      </p>
      {/* Bespoke two-line chip, not the shared `Badge` primitive: `Badge` is a single-line,
          11-13px uppercase label (libs/web-ui/src/components/badge/badge.tsx) that isn't suited
          to this chip's two-line, 24px/16px bilingual content, so `rounded-lg` is a deliberate,
          recorded choice rather than an ad hoc one — see Rule-15 finding DWT-004. */}
      <div
        className="bg-accent text-accent-foreground flex flex-col items-center gap-1 rounded-lg px-5 py-2 sm:flex-row sm:gap-2"
        title="insight (English) · wawasan (Indonesian) · بصيرة (Arabic)"
      >
        <span lang="ar" dir="rtl" className="text-2xl font-semibold">
          بصيرة
        </span>
        <span className="text-muted-foreground">insight &middot; wawasan</span>
      </div>
      <p className="text-muted-foreground text-sm">{greeting}</p>
      <a
        href="https://github.com/wahidyankf/baseerah"
        target="_blank"
        rel="noreferrer"
        className="text-primary underline underline-offset-4"
      >
        View on GitHub
      </a>
    </AppFrame>
  );
}
