import { AppHeader } from "@open-sharia-enterprise/web-ui";

export interface AppShellProps {
  greeting: string;
}

export function AppShell({ greeting }: AppShellProps) {
  return (
    <div className="flex min-h-screen flex-col">
      <AppHeader title="Baseerah" />
      <main className="flex flex-1 flex-col items-center justify-center gap-5 px-6 py-12 text-center">
        <div className="bg-accent text-accent-foreground flex flex-col items-center gap-1 rounded-lg px-5 py-2 sm:flex-row sm:gap-2">
          <span lang="ar" dir="rtl" className="text-2xl font-semibold">
            بصيرة
          </span>
          <span className="text-muted-foreground">insight &middot; wawasan</span>
        </div>
        <p className="text-4xl font-bold">{greeting}</p>
      </main>
      <footer className="border-border bg-secondary text-muted-foreground border-t px-8 py-4 text-center text-sm">
        baseerah-fe &middot; connected to :19320
      </footer>
    </div>
  );
}
