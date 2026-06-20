import { Suspense } from "react";
import type { Metadata } from "next";
import { CostOfLivingCalculatorContent } from "./calculator-content";

export async function generateMetadata(_props: { params: Promise<{ locale: string }> }): Promise<Metadata> {
  return { title: "Cost of Living Calculator | AyoKoding" };
}

export default function CostOfLivingCalculatorPage() {
  return (
    <Suspense>
      <CostOfLivingCalculatorContent />
    </Suspense>
  );
}
