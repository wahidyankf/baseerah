# learning/code/ex-24-teacher-errors-propagate/teacher_errors_propagate.py
"""Worked Example 24: Teacher Errors Propagate."""  # => co-14: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import TypedDict  # => co-14: the same SFT example shape reused across this band


class SFTExample(TypedDict):  # => co-09: mirrors ex-17/ex-19's schema for this file's self-containment
    instruction: str  # => co-09: what the model is asked to do
    response: str  # => co-09: the target the model is trained to produce for this instruction


# => co-14: a LARGER teacher model, prompted to generate synthetic triage examples -- it has one systematic, confidently-wrong habit
TEACHER_GENERATED_DATASET: list[SFTExample] = [  # => co-14: five synthetic examples, one deliberately planted teacher error
    {"instruction": "Triage: dashboard fails to load for one user.", "response": "Priority: P2. Category: bug."},  # => co-14: correct
    {"instruction": "Triage: customer requests a data export.", "response": "Priority: P3. Category: feature-request."},  # => co-14: correct
    {"instruction": "Triage: login page returns a 404 for everyone.", "response": "Priority: P3. Category: bug."},  # => co-14: WRONG -- a company-wide outage mislabeled as low priority
    {"instruction": "Triage: customer wants a CSV export template.", "response": "Priority: P3. Category: feature-request."},  # => co-14: correct
    {"instruction": "Triage: customer asks about API rate limits.", "response": "Priority: P3. Category: feature-request."},  # => co-14: correct
]  # => co-14: closes TEACHER_GENERATED_DATASET -- the teacher SYSTEMATICALLY under-prioritizes "everyone" outages, once, silently


def mock_student_trained_on(dataset: list[SFTExample], instruction: str) -> str:  # => co-14: the student memorizes whatever the teacher generated, verbatim
    """Return the response the student learned for `instruction` (mocked as an exact memorized lookup)."""  # => co-14: documents mock_student_trained_on's contract -- no runtime output, just sets its __doc__
    for example in dataset:  # => co-14: a real model would generalize -- this mock finds the literal memorized match
        if example["instruction"] == instruction:  # => co-14: exact match -- what the student actually memorized from the teacher's output
            return example["response"]  # => co-14: recites exactly what the teacher generated, error included
    return "UNSEEN"  # => co-14: no training example matched this exact phrasing


if __name__ == "__main__":  # => co-14: entry point -- runs only when this file executes directly, not on import
    outage_instruction = "Triage: login page returns a 404 for everyone."  # => co-14: the case the teacher got wrong
    teacher_answer = next(ex["response"] for ex in TEACHER_GENERATED_DATASET if ex["instruction"] == outage_instruction)  # => co-14: what the teacher actually generated
    student_answer = mock_student_trained_on(TEACHER_GENERATED_DATASET, outage_instruction)  # => co-14: what the student, trained on the teacher's output, now says
    print(f"Teacher generated: {teacher_answer!r}")  # => co-14: the teacher's own (wrong) label
    print(f"Student learned: {student_answer!r}")  # => co-14: the student's answer, after training on the teacher's data
    assert teacher_answer == student_answer, "the student must reproduce the teacher's error EXACTLY -- that is the propagation"  # => co-14,co-28
    assert "P3" in student_answer, "this specific planted error must be the mislabeled-as-low-priority outage"  # => co-14
    print("MATCH: a company-wide outage, mislabeled P3 by the teacher, was learned by the student as if it were correct")  # => co-14
    # => co-14,co-28: nothing in the student's own training loss flags this -- the error looks exactly like any other correctly-labelled example
