"use client";

import { useCallback, useEffect, useRef, useState } from "react";

/** How long the `copied` flag stays true before auto-reverting, in milliseconds. */
const DEFAULT_RESET_MS = 2000;

export interface UseCopyToClipboardOptions {
  /**
   * How long the `copied` flag stays true before auto-reverting. Defaults to
   * `DEFAULT_RESET_MS` (2000ms) — the same duration the button's success icon/label persists.
   */
  resetMs?: number;
}

export interface UseCopyToClipboardResult {
  /**
   * `true` only after a **resolved** `navigator.clipboard.writeText`. Deliberately never set on a
   * rejection (non-secure context / permission denied) so the UI can't show a false success.
   */
  copied: boolean;
  /**
   * Writes `value` to the clipboard via `navigator.clipboard.writeText`. On resolve it flips
   * `copied` true and schedules the auto-revert; on reject it swallows the error and leaves
   * `copied` false. No `document.execCommand` fallback — the async Clipboard API is universally
   * available in the secure contexts (HTTPS / localhost) both consuming sites run in.
   */
  copy: (value: string) => Promise<void>;
}

/**
 * Owns the copy side effect and the transient `copied` flag so `CopyButton` and its tests share one
 * implementation. Mirrors `use-resizable-width.ts`'s `"use client"` + timeout-cleanup-on-unmount
 * shape: the pending reset timeout is cleared both on unmount and before scheduling a fresh one, so
 * no `setCopied` ever fires after unmount and rapid re-copies don't stack overlapping reverts.
 */
export function useCopyToClipboard({
  resetMs = DEFAULT_RESET_MS,
}: UseCopyToClipboardOptions = {}): UseCopyToClipboardResult {
  const [copied, setCopied] = useState(false);
  // A ref (not state) because the timeout id is bookkeeping the render output never reads; storing
  // it in state would trigger needless re-renders on every schedule/clear.
  const timeoutRef = useRef<ReturnType<typeof setTimeout> | null>(null);

  const clearPending = useCallback(() => {
    if (timeoutRef.current !== null) {
      clearTimeout(timeoutRef.current);
      timeoutRef.current = null;
    }
  }, []);

  // Clear any in-flight revert when the component unmounts so the timer can't call `setCopied`
  // on an unmounted tree.
  useEffect(() => clearPending, [clearPending]);

  const copy = useCallback(
    async (value: string) => {
      try {
        await navigator.clipboard.writeText(value);
      } catch {
        // Rejected write (non-secure context / denied): leave `copied` false — no false success.
        return;
      }
      // Cancel a still-pending revert from a prior copy before starting a fresh success window.
      clearPending();
      setCopied(true);
      timeoutRef.current = setTimeout(() => {
        setCopied(false);
        timeoutRef.current = null;
      }, resetMs);
    },
    [clearPending, resetMs],
  );

  return { copied, copy };
}
