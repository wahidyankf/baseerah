# learning/code/ex-36-regression-suite/regression_suite.py
"""Worked Example 36: Regression Suite."""  # => co-26: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import NamedTuple  # => co-26: one immutable row per regression case, each tagged with the capability it exercises


class RegressionCase(NamedTuple):  # => co-26: a capability check the fine-tune was NOT meant to change
    case_id: str  # => co-26: unique id
    capability: str  # => co-26: which untouched capability this case probes -- deliberately NOT the triage target task
    prompt: str  # => co-26: what is asked
    expected_answer: str  # => co-26: the correct answer, independent of the fine-tuning target


TARGET_TASK_CAPABILITY = "triage"  # => co-26: the ONE capability the fine-tune is meant to change -- everything else here must differ from it

# => co-26: 12 cases spanning four capabilities the triage fine-tune should never have touched
REGRESSION_SUITE: list[RegressionCase] = [  # => co-26: one row per regression case
    RegressionCase(case_id="reg-01", capability="arithmetic", prompt="What is 47 + 58?", expected_answer="105"),  # => co-26: arithmetic 1
    RegressionCase(case_id="reg-02", capability="arithmetic", prompt="What is 12 * 9?", expected_answer="108"),  # => co-26: arithmetic 2
    RegressionCase(case_id="reg-03", capability="arithmetic", prompt="What is 144 / 12?", expected_answer="12"),  # => co-26: arithmetic 3
    RegressionCase(case_id="reg-04", capability="general-qa", prompt="What is the capital of Japan?", expected_answer="Tokyo"),  # => co-26: general-qa 1
    RegressionCase(case_id="reg-05", capability="general-qa", prompt="How many days in a leap year?", expected_answer="366"),  # => co-26: general-qa 2
    RegressionCase(case_id="reg-06", capability="general-qa", prompt="What gas do plants absorb?", expected_answer="carbon dioxide"),  # => co-26: general-qa 3
    RegressionCase(case_id="reg-07", capability="formatting", prompt="List three fruits as a comma-separated line.", expected_answer="apple, pear, plum"),  # => co-26: formatting 1
    RegressionCase(case_id="reg-08", capability="formatting", prompt="Format 'hello world' in title case.", expected_answer="Hello World"),  # => co-26: formatting 2
    RegressionCase(case_id="reg-09", capability="formatting", prompt="Wrap 'note' in square brackets.", expected_answer="[note]"),  # => co-26: formatting 3
    RegressionCase(case_id="reg-10", capability="reasoning", prompt="If all cats are mammals and Tom is a cat, is Tom a mammal?", expected_answer="yes"),  # => co-26: reasoning 1
    RegressionCase(case_id="reg-11", capability="reasoning", prompt="Which is heavier: one kilogram of feathers or 500 grams of lead?", expected_answer="the feathers"),  # => co-26: reasoning 2
    RegressionCase(case_id="reg-12", capability="reasoning", prompt="A train leaves at 2pm and arrives at 5pm; how long was the trip?", expected_answer="3 hours"),  # => co-26: reasoning 3
]  # => co-26: closes REGRESSION_SUITE -- 12 cases, 4 capabilities, 3 each


if __name__ == "__main__":  # => co-26: entry point -- runs only when this file executes directly, not on import
    capabilities_covered = {case.capability for case in REGRESSION_SUITE}  # => co-26: the DISTINCT capabilities this suite actually probes
    print(f"Regression suite covers {len(REGRESSION_SUITE)} cases across capabilities: {sorted(capabilities_covered)}")  # => co-26
    assert TARGET_TASK_CAPABILITY not in capabilities_covered, "the regression suite must NOT re-test the fine-tune's own target task"  # => co-26
    assert len(capabilities_covered) == 4, "the suite must span multiple distinct untouched capabilities, not just one"  # => co-26
    for capability in sorted(capabilities_covered):  # => co-26: verify EACH capability has multiple cases, not just a token single check
        cases_in_capability = [c for c in REGRESSION_SUITE if c.capability == capability]  # => co-26: this capability's own cases
        assert len(cases_in_capability) >= 3, f"capability {capability!r} must have at least 3 regression cases to be a real check"  # => co-26
    print("MATCH: this suite is built entirely from capabilities OUTSIDE the fine-tuning target -- exactly what co-22's forgetting check needs to run against")  # => co-26
    # => co-26: without this suite, ex-37's forgetting measurement has nothing to run against -- the suite must exist BEFORE the adapted model does
