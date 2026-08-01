import * as React from "react";

export interface AppHeaderProps {
  title: string;
  titleHref?: string;
  subtitle?: string;
  onBack?: () => void;
  trailing?: React.ReactNode;
}

export function AppHeader({ title, titleHref, subtitle, onBack, trailing }: AppHeaderProps) {
  const titleElement = <h1 className="truncate text-xl leading-none font-extrabold tracking-tight">{title}</h1>;

  return (
    <header className="flex items-center gap-3 px-4 py-3">
      {onBack && (
        <button
          type="button"
          aria-label="Go back"
          onClick={onBack}
          className="bg-secondary flex h-10 w-10 flex-shrink-0 items-center justify-center rounded-xl"
        >
          ←
        </button>
      )}
      <div className="min-w-0 flex-1">
        {titleHref ? (
          <a href={titleHref} className="hover:opacity-80">
            {titleElement}
          </a>
        ) : (
          titleElement
        )}
        {subtitle && <p className="text-muted-foreground mt-0.5 text-xs">{subtitle}</p>}
      </div>
      {trailing && <div className="flex-shrink-0">{trailing}</div>}
    </header>
  );
}
