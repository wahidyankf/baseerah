# learning/code/ex-49-retire-an-adapter/retire_an_adapter.py
"""Worked Example 49: Retire an Adapter."""  # => co-32: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from dataclasses import dataclass  # => co-32: a small, self-documenting record for the retirement decision itself


@dataclass(frozen=True)  # => co-32: frozen -- a retirement decision is a fact once measured and made, not a mutable running total
class RetirementCase:  # => co-32: an adapter, its replacement candidate, and the evidence deciding between them
    adapter_name: str  # => co-32: the adapter under consideration for retirement
    adapter_pass_rate: float  # => co-32: the adapter's own current measured pass rate on its target task
    replacement_strategy: str  # => co-04: what would replace the adapter -- here, retrieval, echoing ex-04's own lineage
    replacement_pass_rate: float  # => co-04: the replacement's measured pass rate on the SAME task
    replacement_maintenance_cost_usd_per_month: float  # => co-08: the replacement's own ongoing cost, for comparison
    adapter_maintenance_cost_usd_per_month: float  # => co-08,co-30: the adapter's own ongoing cost, including re-adaptation against base upgrades


CASE = RetirementCase(  # => co-32: Vantage's own retirement candidate -- a knowledge-lookup adapter, now beatable by retrieval
    adapter_name="pricing-lookup-adapter-v3",  # => co-32: an OLDER adapter, from back before ex-04's retrieval solution existed
    adapter_pass_rate=0.81,  # => co-32: its current measured pass rate -- decent, but no longer the best option
    replacement_strategy="retrieval over the live pricing document",  # => co-04: matches ex-04's own solved approach
    replacement_pass_rate=0.99,  # => co-04: retrieval's measured pass rate, matching ex-04's own result
    replacement_maintenance_cost_usd_per_month=40.0,  # => co-08: a document index refresh job, far cheaper than adapter upkeep
    adapter_maintenance_cost_usd_per_month=650.0,  # => co-08,co-30: periodic re-adaptation against base upgrades, per ex-48's own lineage
)  # => co-32: closes CASE


def should_retire(case: RetirementCase) -> bool:  # => co-32: retirement is warranted when the replacement wins on BOTH quality and cost
    """Return whether `case`'s replacement strategy both outperforms and costs less to maintain than the current adapter."""  # => co-32: documents should_retire's contract -- no runtime output, just sets its __doc__
    better_quality = case.replacement_pass_rate > case.adapter_pass_rate  # => co-04: does the replacement beat the adapter on the task
    cheaper_to_maintain = case.replacement_maintenance_cost_usd_per_month < case.adapter_maintenance_cost_usd_per_month  # => co-08: does the replacement cost less to keep running
    return better_quality and cheaper_to_maintain  # => co-32: both must hold for a clean, evidenced retirement


if __name__ == "__main__":  # => co-32: entry point -- runs only when this file executes directly, not on import
    print(f"Adapter: {CASE.adapter_name} | pass rate {CASE.adapter_pass_rate:.0%} | ${CASE.adapter_maintenance_cost_usd_per_month:.0f}/month")  # => co-32
    print(f"Replacement: {CASE.replacement_strategy} | pass rate {CASE.replacement_pass_rate:.0%} | ${CASE.replacement_maintenance_cost_usd_per_month:.0f}/month")  # => co-04
    retire = should_retire(CASE)  # => co-32: run the retirement decision
    print(f"Retirement decision: {'retire the adapter' if retire else 'keep the adapter'}")  # => co-32
    assert retire, "this scenario's replacement must clear both the quality and cost bar, making retirement the correct decision"  # => co-32
    pass_rate_gain = CASE.replacement_pass_rate - CASE.adapter_pass_rate  # => co-04: the quality improvement from retiring
    monthly_savings = CASE.adapter_maintenance_cost_usd_per_month - CASE.replacement_maintenance_cost_usd_per_month  # => co-08: the cost improvement from retiring
    print(f"Retiring gains {pass_rate_gain:.0%} pass rate and saves ${monthly_savings:.0f}/month")  # => co-04,co-08
    assert pass_rate_gain > 0 and monthly_savings > 0, "a correctly evidenced retirement must show BOTH a quality gain and a cost saving, not just one"  # => co-04,co-08
    print("MATCH: an old adapter beaten on both quality and cost by retrieval is retired, planned and evidenced, exactly the healthy outcome co-32 describes")  # => co-32,co-04
    # => co-32,co-04: this is what ex-49 exists to show -- adapters age against better alternatives too, and retiring one is a normal maintenance event, not a failure
