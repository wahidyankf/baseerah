"""Worked Example 34: Annotate a Recalibration Cadence Tied to Model, Prompt, and Data Changes."""  # => co-16: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import NamedTuple  # => co-16: RecalibrationTrigger is a typed record, not a bare string list


class RecalibrationTrigger(NamedTuple):  # => co-16: one CONCRETE, checkable event that forces a re-measurement
    trigger_name: str  # => co-16: a short name for this trigger
    concrete_condition: str  # => co-16: exactly what must be observed -- never vague ("periodically") on its own
    is_triggered_by: str  # => co-16: WHICH of model/prompt/data this trigger reacts to


RECALIBRATION_SCHEDULE = [  # => co-16: every trigger this course names as a concrete recalibration cadence
    RecalibrationTrigger("generator-model-swap", "the model producing the answers under evaluation changes at all", "model"),  # => co-16
    RecalibrationTrigger("judge-model-swap", "the judge model itself is upgraded, replaced, or repointed at a new version", "model"),  # => co-16
    RecalibrationTrigger("prompt-template-edit", "the judge's own prompt template is edited in any way", "prompt"),  # => co-16
    RecalibrationTrigger("scheduled-quarterly-check", "a fixed calendar interval elapses, regardless of any known change", "data"),  # => co-16
    RecalibrationTrigger("distribution-shift-detected", "the live input distribution measurably differs from the validation sample", "data"),  # => co-16
]  # => co-16: closes RECALIBRATION_SCHEDULE -- five concrete, checkable triggers


def event_requires_recalibration(event_description: str, schedule: list[RecalibrationTrigger]) -> RecalibrationTrigger | None:  # => co-16: matches a REAL event against the schedule
    """Return the FIRST trigger whose condition text overlaps `event_description`, or None if nothing matches."""  # => co-16: documents event_requires_recalibration's contract -- no runtime output, just sets its __doc__
    for trigger in schedule:  # => co-16: check every trigger in order
        condition_words = set(trigger.concrete_condition.lower().split())  # => co-16: crude but concrete keyword overlap, not a vague vibe check
        event_words = set(event_description.lower().split())  # => co-16: the real event's own words
        if len(condition_words & event_words) >= 3:  # => co-16: a real, checkable overlap threshold
            return trigger  # => co-16: this trigger fires
    return None  # => co-16: no trigger fires -- no recalibration required by this schedule


if __name__ == "__main__":  # => co-16: entry point -- runs only when this file executes directly, not on import
    real_event = "the model producing the answers under evaluation changes to a new version this week"  # => co-16: an actual event the team observed
    unrelated_event = "the team redesigned the internal ticket-tagging color scheme"  # => co-16: an event that should NOT trigger recalibration

    fired_trigger = event_requires_recalibration(real_event, RECALIBRATION_SCHEDULE)  # => co-16: which trigger, if any, this real event matches
    no_trigger = event_requires_recalibration(unrelated_event, RECALIBRATION_SCHEDULE)  # => co-16: confirms an unrelated event fires nothing
    print(f"Real event matched trigger: {fired_trigger.trigger_name if fired_trigger else None}")  # => co-16: prints which trigger fired
    print(f"Unrelated event matched trigger: {no_trigger}")  # => co-16: prints None -- no false trigger

    assert fired_trigger is not None and fired_trigger.trigger_name == "generator-model-swap", "a real model swap must fire the generator-model-swap trigger"  # => co-16: the rule this example proves
    assert no_trigger is None, "an unrelated event must fire no trigger at all"  # => co-16
    all_named_concretely = all(t.concrete_condition and t.is_triggered_by in {"model", "prompt", "data"} for t in RECALIBRATION_SCHEDULE)  # => co-16
    assert all_named_concretely, "every trigger in the schedule must name a concrete condition tied to model, prompt, or data"  # => co-16
    print("MATCH: a real model-swap event correctly fires the matching trigger, and every trigger in the schedule is concrete, not vague")  # => co-16
    # => co-16: "recalibrate periodically" is not a schedule -- these five concrete triggers are what turns co-16 into something a team can actually operate
