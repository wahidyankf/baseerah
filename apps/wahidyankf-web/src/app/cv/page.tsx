import { Suspense } from "react";
import { CvContent } from "@/features/cv/CvContent";

export default function CV() {
  return (
    <Suspense fallback={<div>Loading...</div>}>
      <CvContent />
    </Suspense>
  );
}
