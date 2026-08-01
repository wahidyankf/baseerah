import { AppFrame } from "./AppFrame";

export interface AppShellProps {
  greeting: string;
}

export function AppShell({ greeting }: AppShellProps) {
  return (
    <AppFrame>
      <p className="text-muted-foreground max-w-prose text-lg">
        BeaverNest is a personal operating layer — an AI assistant, a content builder, a posting helper, and a workflow
        engine in one.
      </p>
      <p className="text-accent-foreground flex items-center gap-1.5 text-sm font-medium">
        <span aria-hidden="true" className="bg-accent-foreground inline-block h-1.5 w-1.5 rounded-full" />
        {greeting}
      </p>
      <a
        href="https://github.com/wahidyankf/beaver-nest"
        target="_blank"
        rel="noreferrer"
        className="text-primary underline underline-offset-4"
      >
        View on GitHub<span className="sr-only"> (opens in new tab)</span>
      </a>
    </AppFrame>
  );
}
