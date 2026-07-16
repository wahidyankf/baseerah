"use client";

import * as React from "react";

import { cn } from "../../utils/cn";
import { CopyButton } from "./copy-button";

export interface CodeBlockProps extends React.ComponentProps<"div"> {
  /**
   * Raw, verbatim source text handed to the clipboard (forwarded to `CopyButton`'s `value`). Passed
   * through untouched — no trimming/normalizing — so every annotation and newline survives the copy.
   */
  code: string;
  /** Forwarded to `CopyButton`'s `copyLabel` (localized by consuming apps). */
  copyLabel?: string;
  /** Forwarded to `CopyButton`'s `copiedLabel` (localized by consuming apps). */
  copiedLabel?: string;
  /** Forwarded to `CopyButton`'s `resetMs` — how long the success state persists. Defaults to 2000ms. */
  resetMs?: number;
}

/**
 * Layout composer for the code-block use case. Wraps the app's already-highlighted `<pre>`/figure
 * subtree in a `group relative` container that establishes its **own** positioning context (never
 * relying on app CSS), then overlays the `CopyButton` at the top-right — outside the scrolling
 * `<pre>` so the control is never clipped by `overflow-x: auto`. The button is hover-revealed on
 * fine pointers, always shown on `focus-visible`/`focus-within`, and always visible where the
 * pointer can't hover (`@media (hover: none)`), so it's never stranded from touch, keyboard, or AT.
 */
function CodeBlock({ code, copyLabel, copiedLabel, resetMs, className, children, ...props }: CodeBlockProps) {
  return (
    <div data-slot="code-block" className={cn("group relative", className)} {...props}>
      {children}
      <CopyButton
        value={code}
        copyLabel={copyLabel}
        copiedLabel={copiedLabel}
        resetMs={resetMs}
        className="absolute top-2 right-2 opacity-0 transition-opacity group-focus-within:opacity-100 group-hover:opacity-100 focus-visible:opacity-100 [@media(hover:none)]:opacity-100"
      />
    </div>
  );
}

export { CodeBlock };
