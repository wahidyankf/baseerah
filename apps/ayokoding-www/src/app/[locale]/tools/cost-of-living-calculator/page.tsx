import { Suspense } from "react";
import { CostOfLivingCalculatorContent } from "./calculator-content";

export default function CostOfLivingCalculatorPage() {
  return (
    <Suspense>
      <CostOfLivingCalculatorContent />
    </Suspense>
  );
}
