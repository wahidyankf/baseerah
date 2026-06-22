"use client";

import { useEffect, useRef, useState } from "react";

/**
 * Controlled text/number field with a local echo and a debounced commit.
 *
 * Motivation: the calculator treats the URL as the single source of truth, so every
 * committed value is written to the URL (via `router.replace`). Doing that on every
 * keystroke makes typing stutter — each write re-renders the whole calculator (and, in
 * production, asks the Next.js router to reconcile the route). This hook decouples the
 * two concerns:
 *
 *   - `value` is local state that updates synchronously on every keystroke, so the input
 *     stays responsive no matter how heavy the downstream re-render is.
 *   - `onCommit` is invoked only after the user pauses for `delay` ms (or immediately when
 *     `delay <= 0`), collapsing a burst of keystrokes into a single URL write.
 *
 * Correctness details:
 *   - The latest `onCommit` closure is always used (via a ref), so a commit that fires
 *     after an unrelated control changed the URL merges into the freshest state rather
 *     than overwriting it with a stale snapshot.
 *   - While a commit is pending, an incoming `external` change is ignored so it cannot
 *     clobber what the user is actively typing; once nothing is pending, `external`
 *     (e.g. a deep link, a currency reset, or our own committed value) is adopted.
 *   - `flush()` commits any pending value immediately — wire it to `onBlur` so leaving the
 *     field persists the typed value at once instead of waiting out the debounce window.
 *
 * Pass `delay = 0` for the uncontrolled/standalone path: the commit runs synchronously,
 * preserving the original immediate behaviour (and keeping prop-driven tests timer-free).
 */
export function useDebouncedField<T>(
  external: T,
  onCommit: (value: T) => void,
  delay: number,
): { value: T; onChange: (value: T) => void; flush: () => void } {
  const [local, setLocal] = useState<T>(external);
  const timer = useRef<ReturnType<typeof setTimeout> | null>(null);
  const pending = useRef<{ value: T } | null>(null);

  // Always call the freshest onCommit so a late commit merges into the latest state.
  const commitRef = useRef(onCommit);
  commitRef.current = onCommit;

  // Adopt external updates only when nothing is mid-flight, so active typing is never
  // clobbered and our own committed value round-tripping back is a no-op.
  useEffect(() => {
    if (timer.current === null) setLocal(external);
  }, [external]);

  // Clear any pending timer on unmount.
  useEffect(() => () => clearPending(), []);

  function clearPending() {
    if (timer.current !== null) {
      clearTimeout(timer.current);
      timer.current = null;
    }
    pending.current = null;
  }

  function commitNow(value: T) {
    clearPending();
    commitRef.current(value);
  }

  function onChange(value: T) {
    setLocal(value);
    if (delay <= 0) {
      commitNow(value);
      return;
    }
    pending.current = { value };
    if (timer.current !== null) clearTimeout(timer.current);
    timer.current = setTimeout(() => {
      timer.current = null;
      const p = pending.current;
      pending.current = null;
      if (p) commitRef.current(p.value);
    }, delay);
  }

  function flush() {
    if (pending.current !== null) commitNow(pending.current.value);
  }

  return { value: local, onChange, flush };
}

/** Debounce delay (ms) for continuous text inputs when the URL is the source of truth. */
export const URL_INPUT_DEBOUNCE_MS = 300;
