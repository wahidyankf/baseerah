"use client";

import * as React from "react";
import { Check, Copy } from "lucide-react";

import { cn } from "../../utils/cn";
import { Button } from "../button/button";
import { useCopyToClipboard } from "./use-copy-to-clipboard";

export interface CopyButtonProps extends React.ComponentProps<"button"> {
  /** Exact text written to the clipboard. */
  value: string;
  /**
   * `aria-label` in the resting state. Web-ui primitives are locale-agnostic, so consuming apps
   * pass a localized string; defaults to the English "Copy" (mirrors the resizable-panel
   * `handleAriaLabel` locale-agnostic-with-English-default precedent).
   */
  copyLabel?: string;
  /** Announced via the live region on success and used as the success-state `aria-label`. */
  copiedLabel?: string;
  /** How long the success state persists before reverting. Defaults to 2000ms. */
  resetMs?: number;
}

/**
 * A standalone, reusable copy affordance (copy any string). Composes the `Button` primitive
 * (`variant="ghost" size="icon-sm"`, which auto-sizes the lucide svg and supplies the
 * `focus-visible` ring) and adds the `Copy`→`Check` icon swap plus a visually-hidden
 * `role="status"` live region so assistive tech hears the success. Icons are `aria-hidden` because
 * the accessible name comes from `aria-label`; keyboard operability (Enter/Space) is native to the
 * underlying `<button>`.
 */
function CopyButton({
  value,
  copyLabel = "Copy",
  copiedLabel = "Copied",
  resetMs = 2000,
  className,
  onClick,
  ...props
}: CopyButtonProps) {
  const { copied, copy } = useCopyToClipboard({ resetMs });

  const handleClick = (event: React.MouseEvent<HTMLButtonElement>) => {
    onClick?.(event);
    void copy(value);
  };

  return (
    <>
      <Button
        type="button"
        variant="ghost"
        size="icon-sm"
        data-slot="code-block-copy"
        aria-label={copied ? copiedLabel : copyLabel}
        onClick={handleClick}
        className={cn(
          // Resting icon meets WCAG AA non-text contrast against both Shiki grounds; hover/focus
          // deepens it. Success switches to a theme-token green (Tailwind palette, not raw hex).
          "text-muted-foreground transition-colors hover:text-foreground",
          copied && "text-green-600 hover:text-green-600 dark:text-green-500 dark:hover:text-green-500",
          className,
        )}
        {...props}
      >
        {copied ? <Check aria-hidden="true" /> : <Copy aria-hidden="true" />}
      </Button>
      {/*
        Always-present polite live region: a native `<output>` (implicit `role="status"` +
        `aria-live="polite"`, with the redundant `aria-live` kept per MDN's recommendation) that
        starts empty and only carries text while `copied`, so AT announces the success without
        stealing focus. This aria-live status pattern is new to web-ui. Using `<output>` (rather than
        a `<span role="status">`) keeps the markup jsx-a11y-clean while preserving the same role.
      */}
      <output aria-live="polite" className="sr-only">
        {copied ? copiedLabel : ""}
      </output>
    </>
  );
}

export { CopyButton };
