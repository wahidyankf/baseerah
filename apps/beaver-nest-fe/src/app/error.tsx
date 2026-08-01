"use client";

import { Button } from "@open-sharia-enterprise/web-ui";
import { AppFrame } from "@/components/AppFrame";

// Next.js App Router requires error.tsx to be a Client Component (it hydrates over the failed
// server-render tree). No metadata export is possible here for that same reason — this renders
// in place of the existing route, not as a new navigation.
export default function Error({ reset }: { error: Error & { digest?: string }; reset: () => void }) {
  return (
    <AppFrame>
      <p className="text-2xl font-semibold">Something went wrong.</p>
      <Button onClick={() => reset()}>Try again</Button>
    </AppFrame>
  );
}
