# learning/code/ex-22-expert-authored-examples/expert_authored.py
"""Worked Example 22: Expert-Authored Examples."""  # => co-13: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from dataclasses import dataclass  # => co-13: two sourcing strategies, each with its own cost/quality shape


@dataclass(frozen=True)  # => co-13: frozen -- a sourcing strategy's measured profile should not mutate after the fact
class SourcingProfile:  # => co-13: the trade-off co-13 names, made an actual comparable record
    example_count: int  # => co-13: how many examples this strategy produced
    cost_per_example_usd: float  # => co-13: labour cost per example -- expert time is expensive
    label_error_rate: float  # => co-13: what fraction of examples an audit (ex-20's technique) would flag as wrong or inconsistent
    covers_rare_categories: bool  # => co-13: can this strategy deliberately cover categories production traffic under-samples (ex-21)?


TRAFFIC_SOURCED = SourcingProfile(example_count=2_000, cost_per_example_usd=0.02, label_error_rate=0.08, covers_rare_categories=False)  # => co-13: ex-21's approach -- fast, free, but skewed

EXPERT_AUTHORED = SourcingProfile(
    example_count=300, cost_per_example_usd=4.50, label_error_rate=0.01, covers_rare_categories=True
)  # => co-13,co-11: a lead deliberately writing examples for EVERY category, including rare ones


if __name__ == "__main__":  # => co-13: entry point -- runs only when this file executes directly, not on import
    traffic_total_cost = TRAFFIC_SOURCED.example_count * TRAFFIC_SOURCED.cost_per_example_usd  # => co-13: total labour cost, traffic-sourced
    expert_total_cost = EXPERT_AUTHORED.example_count * EXPERT_AUTHORED.cost_per_example_usd  # => co-13: total labour cost, expert-authored
    print(f"Traffic-sourced: {TRAFFIC_SOURCED.example_count} examples, ${traffic_total_cost:,.2f} total, {TRAFFIC_SOURCED.label_error_rate:.0%} error rate")  # => co-13
    print(f"Expert-authored: {EXPERT_AUTHORED.example_count} examples, ${expert_total_cost:,.2f} total, {EXPERT_AUTHORED.label_error_rate:.0%} error rate")  # => co-13
    print(f"Expert-authored covers rare categories: {EXPERT_AUTHORED.covers_rare_categories} | Traffic-sourced: {TRAFFIC_SOURCED.covers_rare_categories}")  # => co-13
    per_example_cost_ratio = EXPERT_AUTHORED.cost_per_example_usd / TRAFFIC_SOURCED.cost_per_example_usd  # => co-13: expert authoring costs MORE per example, honestly
    print(f"Expert authoring costs {per_example_cost_ratio:.0f}x more PER example than traffic-sourcing")  # => co-13: the real cost trade-off, stated plainly
    assert per_example_cost_ratio > 100, "expert authoring must be substantially more expensive per example -- that is the real cost of this trade"  # => co-13
    assert EXPERT_AUTHORED.label_error_rate < TRAFFIC_SOURCED.label_error_rate, "expert authoring must produce a lower label-error rate"  # => co-11,co-13
    assert EXPERT_AUTHORED.covers_rare_categories and not TRAFFIC_SOURCED.covers_rare_categories, "only expert authoring can deliberately cover what ex-21 showed traffic-sourcing structurally misses"  # => co-13
    print("MATCH: expert authoring costs far more per example, but is the ONLY strategy that fixes ex-21's rare-category blind spot on purpose")  # => co-11,co-13
    # => co-11,co-13: this is co-13's second sourcing profile -- the right choice depends on what the gap actually needs, not a universal "always prefer X"
