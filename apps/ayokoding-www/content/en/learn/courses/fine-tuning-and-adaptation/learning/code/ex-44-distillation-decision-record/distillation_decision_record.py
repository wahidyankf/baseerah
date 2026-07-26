# learning/code/ex-44-distillation-decision-record/distillation_decision_record.py
"""Worked Example 44: Distillation Decision Record."""  # => co-28: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from dataclasses import dataclass  # => co-08: a written record, the same discipline ex-13's total-cost accounting and ex-57's decision record used


@dataclass(frozen=True)  # => co-08: frozen -- a decision record is a fact once written, not a mutable running total
class DistillationDecisionRecord:  # => co-28: a distillation decision framed explicitly as a COST trade, never a quality claim
    teacher_pass_rate: float  # => co-28: the teacher's own measured ceiling, quoted plainly, not exceeded
    student_pass_rate: float  # => co-28: the student's measured result -- BELOW the teacher, by design and by necessity
    monthly_inference_cost_teacher_usd: float  # => co-08: what serving the teacher at Vantage's traffic volume costs per month
    monthly_inference_cost_student_usd: float  # => co-08: what serving the student instead costs per month
    quality_cost_accepted: bool  # => co-28: an explicit, written acknowledgement that quality is being traded away, not hidden


RECORD = DistillationDecisionRecord(  # => co-28: Vantage's actual distillation decision, written down before shipping the student
    teacher_pass_rate=0.97,  # => co-28: matches ex-42/ex-43's teacher ceiling
    student_pass_rate=0.93,  # => co-28: matches ex-42's baseline student result
    monthly_inference_cost_teacher_usd=14_200.00,  # => co-08: the large teacher model's serving bill at current traffic
    monthly_inference_cost_student_usd=1_650.00,  # => co-08: the distilled student's serving bill at the SAME traffic
    quality_cost_accepted=True,  # => co-28: signed off explicitly -- the 4-point quality cost is accepted, not glossed over
)  # => co-28: closes RECORD


def is_honestly_framed(record: DistillationDecisionRecord) -> bool:  # => co-28: a distillation record must NEVER claim a quality win
    """Return whether `record` correctly frames its student as strictly cheaper and never claims it as strictly better on quality."""  # => co-28: documents is_honestly_framed's contract -- no runtime output, just sets its __doc__
    cheaper = record.monthly_inference_cost_student_usd < record.monthly_inference_cost_teacher_usd  # => co-08: the student must actually be cheaper
    not_better_on_quality = record.student_pass_rate <= record.teacher_pass_rate  # => co-28: the student must NEVER be claimed to exceed the teacher
    return cheaper and not_better_on_quality and record.quality_cost_accepted  # => co-28: all three must hold for a record to be honestly framed


if __name__ == "__main__":  # => co-28: entry point -- runs only when this file executes directly, not on import
    monthly_savings = RECORD.monthly_inference_cost_teacher_usd - RECORD.monthly_inference_cost_student_usd  # => co-08: the cost side of the trade
    quality_cost = RECORD.teacher_pass_rate - RECORD.student_pass_rate  # => co-28: the quality side of the trade
    print(f"Monthly savings: ${monthly_savings:,.2f} | Quality cost: {quality_cost:.0%} pass-rate points")  # => co-08,co-28
    assert monthly_savings > 10_000, "the cost savings must be large and concrete, the actual reason distillation was chosen"  # => co-08
    assert quality_cost > 0, "the record must show a real, non-zero quality cost -- pretending distillation is free is dishonest"  # => co-28
    honestly_framed = is_honestly_framed(RECORD)  # => co-28: run the framing check
    print(f"Record is honestly framed as a cost optimization: {honestly_framed}")  # => co-28
    assert honestly_framed, "this record must pass the honest-framing check -- cheaper and accepted-as-lower-quality, never claimed as better"  # => co-28
    print(f"MATCH: ${monthly_savings:,.0f}/month saved for {quality_cost:.0%} accepted quality cost -- written down as a trade, not sold as a free win")  # => co-08,co-28
    # => co-08,co-28: this is the written artefact the tension note demands -- distillation buys latency and cost, and this record says so explicitly
