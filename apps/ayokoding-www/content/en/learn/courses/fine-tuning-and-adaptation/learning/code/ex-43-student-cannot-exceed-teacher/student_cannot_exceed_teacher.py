# learning/code/ex-43-student-cannot-exceed-teacher/student_cannot_exceed_teacher.py
"""Worked Example 43: Student Cannot Exceed Teacher."""  # => co-28: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import NamedTuple  # => co-28: one immutable row per training configuration tried on the student, all on the SAME held-out eval


class StudentTrainingRun(NamedTuple):  # => co-27: a distillation attempt -- more data or more epochs, still bounded by the same teacher
    configuration: str  # => co-27: what was changed about this student training run
    training_examples: int  # => co-27: how many teacher-labeled examples the student trained on
    pass_rate: float  # => co-28: measured pass rate on the SAME held-out eval the teacher was measured on


TEACHER_PASS_RATE = 0.97  # => co-28: the teacher's own measured ceiling on this eval, from ex-42's lineage -- fixed, not something the student can move

# => co-28: three attempts to push the student closer to the teacher -- more data, more epochs, and both together
STUDENT_ATTEMPTS: list[StudentTrainingRun] = [  # => co-28: one row per attempt, in the order they were tried
    StudentTrainingRun(configuration="baseline (2,000 examples, 2 epochs)", training_examples=2_000, pass_rate=0.93),  # => co-28: the original ex-42 run
    StudentTrainingRun(configuration="more data (8,000 examples, 2 epochs)", training_examples=8_000, pass_rate=0.95),  # => co-28: 4x the teacher-labeled data
    StudentTrainingRun(configuration="more epochs (8,000 examples, 6 epochs)", training_examples=8_000, pass_rate=0.96),  # => co-28: 4x the data AND 3x the epochs
]  # => co-28: closes STUDENT_ATTEMPTS -- the best attempt still falls short of the teacher


if __name__ == "__main__":  # => co-28: entry point -- runs only when this file executes directly, not on import
    print(f"Teacher pass rate: {TEACHER_PASS_RATE:.0%}")  # => co-28
    for attempt in STUDENT_ATTEMPTS:  # => co-28: show every attempt's result, in order, against the FIXED teacher ceiling
        gap_to_teacher = TEACHER_PASS_RATE - attempt.pass_rate  # => co-28: how far this attempt still is from the teacher
        print(f"  {attempt.configuration}: pass rate {attempt.pass_rate:.0%} (gap to teacher: {gap_to_teacher:.0%})")  # => co-28
    for attempt in STUDENT_ATTEMPTS:  # => co-28: verify EVERY attempt, not just the last, stayed strictly below the teacher's ceiling
        assert attempt.pass_rate < TEACHER_PASS_RATE, f"{attempt.configuration} must not exceed the teacher's own pass rate"  # => co-28
    best_attempt = max(STUDENT_ATTEMPTS, key=lambda a: a.pass_rate)  # => co-28: the best of the three attempts
    print(f"Best attempt: {best_attempt.configuration} at {best_attempt.pass_rate:.0%}, still {TEACHER_PASS_RATE - best_attempt.pass_rate:.0%} below the teacher")  # => co-28
    improving_but_bounded = STUDENT_ATTEMPTS[0].pass_rate < STUDENT_ATTEMPTS[1].pass_rate < STUDENT_ATTEMPTS[2].pass_rate  # => co-28: more effort DOES help, but never crosses the line
    assert improving_but_bounded, "each additional round of student training effort must genuinely improve the result, just never past the teacher"  # => co-28
    print("MATCH: 4x the data and 3x the epochs closed the gap from 4 points to 1 point -- and never crossed it, because the student cannot exceed its teacher")  # => co-28
    # => co-28: more compute spent on the student narrows the gap, it never erases it -- the teacher's own ceiling is the student's hard bound
