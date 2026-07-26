# learning/code/ex-13-total-cost-of-a-fine-tune/total_cost.py
"""Worked Example 13: Total Cost of a Fine-Tune."""  # => co-08: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from dataclasses import dataclass, fields  # => co-08: a typed budget beats a spreadsheet nobody re-checks


@dataclass(frozen=True)  # => co-08: frozen -- a cost estimate should not mutate after it is computed
class FineTuneBudget:  # => co-08: every cost bucket co-08 names, made an actual, addable record
    compute_usd: float  # => co-08: the ONLY line item a naive estimate usually includes
    data_labour_usd: float  # => co-08: labelling, reviewing, and curating the dataset (Band B's whole job)
    evaluation_usd: float  # => co-08: building and running the eval suite the result must be judged against
    first_year_maintenance_usd: float  # => co-08: re-adaptation runs this fine-tune's owner will pay for within a year

    def total(self) -> float:  # => co-08: the honest total -- every bucket, summed
        """Sum every named cost field on this budget."""  # => co-08: documents total's contract -- no runtime output, just sets its __doc__
        return sum(getattr(self, f.name) for f in fields(self))  # => co-08: iterates every dataclass field generically


NAIVE_COMPUTE_ONLY_ESTIMATE_USD = 1_800.0  # => co-08: what the initial project proposal budgeted -- compute only

REAL_BUDGET = FineTuneBudget(  # => co-08: the honest, full accounting for the same project
    compute_usd=1_800.0,  # => co-08: matches the naive estimate exactly -- compute was never the undercounted part
    data_labour_usd=6_400.0,  # => co-08: two engineers, several days, curating and auditing the SFT dataset (Band B)
    evaluation_usd=2_200.0,  # => co-08: building the paired eval + regression suite (Band C) and running it
    first_year_maintenance_usd=3_500.0,  # => co-08: one expected base-model upgrade cycle this year, forcing re-adaptation
)  # => co-08: closes REAL_BUDGET

if __name__ == "__main__":  # => co-08: entry point -- runs only when this file executes directly, not on import
    real_total = REAL_BUDGET.total()  # => co-08: the honestly-summed total
    print(f"Naive compute-only estimate: ${NAIVE_COMPUTE_ONLY_ESTIMATE_USD:,.2f}")  # => co-08: what the proposal said
    print(f"Real total (compute + data labour + eval + Y1 maintenance): ${real_total:,.2f}")  # => co-08: what it actually costs
    undercounted_by = real_total - NAIVE_COMPUTE_ONLY_ESTIMATE_USD  # => co-08: exactly how much the naive estimate missed
    undercount_ratio = real_total / NAIVE_COMPUTE_ONLY_ESTIMATE_USD  # => co-08: expressed as a multiple, easier to communicate upward
    print(f"Undercounted by: ${undercounted_by:,.2f} ({undercount_ratio:.1f}x the naive estimate)")  # => co-08
    assert real_total > NAIVE_COMPUTE_ONLY_ESTIMATE_USD * 5, "the honest total must dwarf the compute-only estimate for this demo to land"  # => co-08
    assert REAL_BUDGET.data_labour_usd > REAL_BUDGET.compute_usd, "data labour must exceed raw compute -- co-10's dataset-is-the-work claim, restated as cost"  # => co-08
    print("MATCH: the honest total is several times the compute-only figure -- data labour and maintenance are the buckets nobody budgets")  # => co-08
    # => co-08: this is co-08 made concrete -- a project proposal that only prices compute will blow its budget the moment data work starts
