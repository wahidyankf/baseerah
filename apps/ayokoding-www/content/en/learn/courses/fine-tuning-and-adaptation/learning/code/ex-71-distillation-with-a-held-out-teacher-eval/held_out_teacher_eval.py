# learning/code/ex-71-distillation-with-a-held-out-teacher-eval/held_out_teacher_eval.py
"""Worked Example 71: Distillation with a Held-Out Teacher Eval."""  # => co-27: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import NamedTuple  # => co-27: one immutable row per held-out case, teacher's own result recorded honestly, not assumed perfect


class TeacherEvalCase(NamedTuple):  # => co-28: the teacher's OWN measured result on cases it never generated labels for
    case_id: str  # => co-27: unique id
    teacher_pass: bool  # => co-28: did the teacher itself get this held-out case right -- checked, not assumed


# => co-28: 10 held-out cases the teacher NEVER saw during its own training -- the teacher is not assumed to be a perfect oracle
TEACHER_HELD_OUT_RESULTS: list[TeacherEvalCase] = [  # => co-28: one row per held-out case
    TeacherEvalCase(case_id="held-01", teacher_pass=True),  # => co-28: teacher passes
    TeacherEvalCase(case_id="held-02", teacher_pass=True),  # => co-28: teacher passes
    TeacherEvalCase(case_id="held-03", teacher_pass=True),  # => co-28: teacher passes
    TeacherEvalCase(case_id="held-04", teacher_pass=True),  # => co-28: teacher passes
    TeacherEvalCase(case_id="held-05", teacher_pass=True),  # => co-28: teacher passes
    TeacherEvalCase(case_id="held-06", teacher_pass=True),  # => co-28: teacher passes
    TeacherEvalCase(case_id="held-07", teacher_pass=True),  # => co-28: teacher passes
    TeacherEvalCase(case_id="held-08", teacher_pass=True),  # => co-28: teacher passes
    TeacherEvalCase(case_id="held-09", teacher_pass=True),  # => co-28: teacher passes
    TeacherEvalCase(case_id="held-10", teacher_pass=False),  # => co-28: teacher itself FAILS this one -- the teacher is not perfect, and this proves it
]  # => co-28: closes TEACHER_HELD_OUT_RESULTS

ASSUMED_TEACHER_PASS_RATE = 0.97  # => co-28: the figure ex-42/ex-43 quoted for the teacher -- convenient, but was it actually MEASURED on held-out data?
STUDENT_PASS_RATE = 0.88  # => co-28: this distillation's own measured student result on THIS held-out set -- still below the teacher's honestly measured ceiling


if __name__ == "__main__":  # => co-28: entry point -- runs only when this file executes directly, not on import
    measured_teacher_pass_rate = sum(1 for c in TEACHER_HELD_OUT_RESULTS if c.teacher_pass) / len(TEACHER_HELD_OUT_RESULTS)  # => co-28: the teacher's ACTUAL held-out result, checked rather than assumed
    print(f"Assumed teacher pass rate (from earlier examples): {ASSUMED_TEACHER_PASS_RATE:.0%}")  # => co-27
    print(f"Measured teacher pass rate on THIS held-out set: {measured_teacher_pass_rate:.0%}")  # => co-28
    assert measured_teacher_pass_rate == 0.90, "the teacher's measured held-out pass rate must be exactly 90% in this scenario, not the assumed 97%"  # => co-28
    real_ceiling = measured_teacher_pass_rate  # => co-28: this is the student's REAL ceiling, not the number quoted from a different eval
    student_gap_to_real_ceiling = real_ceiling - STUDENT_PASS_RATE  # => co-28: how the student's own gap looks against the teacher's MEASURED ceiling
    student_gap_to_assumed_ceiling = ASSUMED_TEACHER_PASS_RATE - STUDENT_PASS_RATE  # => co-28: how the gap looked against the ASSUMED ceiling
    print(f"Student gap to measured ceiling: {student_gap_to_real_ceiling:.0%} | Student gap to assumed ceiling: {student_gap_to_assumed_ceiling:.0%}")  # => co-28
    assert student_gap_to_real_ceiling < student_gap_to_assumed_ceiling, "the gap against the MEASURED teacher ceiling must be smaller than the gap against the assumed one, since the true ceiling is lower"  # => co-28
    student_exceeds_measured_teacher = STUDENT_PASS_RATE > real_ceiling  # => co-28: does the student now look like it beats the teacher's REAL ceiling
    print(f"Student appears to exceed the teacher's measured ceiling: {student_exceeds_measured_teacher}")  # => co-28
    assert not student_exceeds_measured_teacher, "even against a lower, honestly measured ceiling, the student must still not exceed the teacher"  # => co-28
    print("MATCH: the teacher's OWN measured held-out pass rate is 90%, not the assumed 97% -- checking it changes the honest gap without ever flipping which model leads")  # => co-27,co-28
    # => co-27,co-28: a distillation ceiling quoted from a different eval, never re-measured on the student's own held-out set, is an assumption, not a fact
