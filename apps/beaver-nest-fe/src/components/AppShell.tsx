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
