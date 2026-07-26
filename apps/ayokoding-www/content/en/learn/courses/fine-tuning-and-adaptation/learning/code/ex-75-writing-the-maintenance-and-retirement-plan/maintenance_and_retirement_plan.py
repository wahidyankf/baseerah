# learning/code/ex-75-writing-the-maintenance-and-retirement-plan/maintenance_and_retirement_plan.py
"""Worked Example 75: Writing the Maintenance and Retirement Plan."""  # => co-30: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from dataclasses import dataclass  # => co-32: the closing operational document -- concrete conditions, not vague intentions


@dataclass(frozen=True)  # => co-30: frozen -- a maintenance plan is a written commitment once filed, not a mutable running total
class MaintenancePlan:  # => co-30,co-32: the standing document co-30 and co-32 both require an adapter to ship with
    adapter_name: str  # => co-30: which adapter this plan covers
    pinned_base_version: str  # => co-30: the exact base version this adapter is currently valid against, matching ex-48's own pin
    re_adaptation_trigger: str  # => co-30: the CONCRETE condition that requires re-training, stated plainly enough to act on
    retirement_trigger: str  # => co-32: the CONCRETE condition under which this adapter is retired outright, not re-trained
    owning_team: str  # => co-30: who is accountable for acting when either trigger fires


PLAN = MaintenancePlan(  # => co-30,co-32: Vantage's own filed plan for the triage adapter
    adapter_name="triage-v1",  # => co-30: matches ex-45/ex-48's own adapter
    pinned_base_version="qwen2.5-0.5b-instruct-r1",  # => co-30: matches ex-48's own pin
    re_adaptation_trigger="the pinned base version is superseded by a new release the platform team deploys",  # => co-30: concrete, matches ex-48's own scenario exactly
    retirement_trigger="a retrieval-based or prompting-based alternative measurably beats this adapter on both pass rate and monthly cost",  # => co-32: concrete, matches ex-49's own scenario exactly
    owning_team="platform-ml-team",  # => co-30: matches ex-72's own registry record for this adapter
)  # => co-30: closes PLAN


def trigger_is_concrete(trigger: str) -> bool:  # => co-30,co-32: a real trigger names a checkable CONDITION, not a vague aspiration
    """Return whether `trigger` reads as a concrete, checkable condition rather than a vague statement."""  # => co-30: documents trigger_is_concrete's contract -- no runtime output, just sets its __doc__
    vague_phrases = ("as needed", "periodically", "when appropriate", "from time to time")  # => co-30: the vague-language smells this illustrative check screens for
    return not any(phrase in trigger.lower() for phrase in vague_phrases) and len(trigger) > 20  # => co-30: not vague AND substantive enough to be an actual condition


if __name__ == "__main__":  # => co-30: entry point -- runs only when this file executes directly, not on import
    print(f"Adapter: {PLAN.adapter_name} | pinned to {PLAN.pinned_base_version} | owned by {PLAN.owning_team}")  # => co-30
    print(f"Re-adaptation trigger: {PLAN.re_adaptation_trigger}")  # => co-30
    print(f"Retirement trigger: {PLAN.retirement_trigger}")  # => co-32
    re_adaptation_concrete = trigger_is_concrete(PLAN.re_adaptation_trigger)  # => co-30: verify the re-adaptation condition is actually checkable
    retirement_concrete = trigger_is_concrete(PLAN.retirement_trigger)  # => co-32: verify the retirement condition is actually checkable
    print(f"Re-adaptation trigger is concrete: {re_adaptation_concrete} | Retirement trigger is concrete: {retirement_concrete}")  # => co-30,co-32
    assert re_adaptation_concrete, "the re-adaptation trigger must be a concrete, checkable condition, not a vague aspiration"  # => co-30
    assert retirement_concrete, "the retirement trigger must be a concrete, checkable condition, not a vague aspiration"  # => co-32
    vague_plan_trigger = "review the adapter periodically and retire it as needed"  # => co-30,co-32: a deliberately vague counter-example, the kind this check must catch
    vague_is_concrete = trigger_is_concrete(vague_plan_trigger)  # => co-30: run the SAME check against the vague counter-example
    print(f"Vague counter-example trigger is concrete: {vague_is_concrete}")  # => co-30
    assert not vague_is_concrete, "a vague trigger like 'review periodically... as needed' must fail the concreteness check"  # => co-30,co-32
    print("MATCH: this plan's own triggers cite exact, checkable conditions from ex-48 and ex-49 -- a vague 'review periodically' trigger is caught and rejected")  # => co-30,co-32
    # => co-30,co-32: this is the final artefact the tension note and co-32 both call for -- a plan an on-call engineer can actually act on, not a good intention
