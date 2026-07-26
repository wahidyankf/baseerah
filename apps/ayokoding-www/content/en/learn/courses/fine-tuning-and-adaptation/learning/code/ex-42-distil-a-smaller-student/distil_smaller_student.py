# learning/code/ex-42-distil-a-smaller-student/distil_smaller_student.py
"""Worked Example 42: Distil a Smaller Student."""  # => co-27: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from dataclasses import dataclass  # => co-27: a small, self-documenting record for teacher and student -- latency and cost, side by side


@dataclass(frozen=True)  # => co-27: frozen -- a model's measured profile is a fact once benchmarked, not a mutable running total
class ModelProfile:  # => co-27: a model's own cost/latency/quality shape, whether teacher or student
    name: str  # => co-27: which model this profile describes
    parameter_count: int  # => co-27: total parameters -- the thing distillation is trying to shrink
    pass_rate: float  # => co-27: measured pass rate on Vantage's triage task
    latency_ms: float  # => co-27: measured per-request latency, the thing distillation is trying to buy


TEACHER = ModelProfile(name="large-general-model", parameter_count=7_000_000_000, pass_rate=0.97, latency_ms=850.0)  # => co-27: a large, capable, SLOW model
STUDENT = ModelProfile(name="distilled-small-model", parameter_count=494_000_000, pass_rate=0.93, latency_ms=95.0)  # => co-27: trained to reproduce the teacher's triage outputs


if __name__ == "__main__":  # => co-27: entry point -- runs only when this file executes directly, not on import
    print(f"Teacher: {TEACHER.name} | {TEACHER.parameter_count:,} params | pass rate {TEACHER.pass_rate:.0%} | latency {TEACHER.latency_ms:.0f}ms")  # => co-27
    print(f"Student: {STUDENT.name} | {STUDENT.parameter_count:,} params | pass rate {STUDENT.pass_rate:.0%} | latency {STUDENT.latency_ms:.0f}ms")  # => co-27
    size_reduction = 1 - (STUDENT.parameter_count / TEACHER.parameter_count)  # => co-27: how much smaller the student is
    latency_reduction = 1 - (STUDENT.latency_ms / TEACHER.latency_ms)  # => co-27: how much faster the student is
    pass_rate_cost = TEACHER.pass_rate - STUDENT.pass_rate  # => co-28: the quality the student gave up to get there
    print(f"Size reduction: {size_reduction:.0%} | Latency reduction: {latency_reduction:.0%} | Pass-rate cost: {pass_rate_cost:.0%}")  # => co-27,co-28
    assert size_reduction > 0.85, "the student must be dramatically smaller than the teacher for distillation to be worth doing at all"  # => co-27
    assert latency_reduction > 0.80, "the student must be dramatically faster, since latency is the entire point of distilling here"  # => co-27
    assert pass_rate_cost > 0, "the student must give up SOME quality relative to the teacher -- distillation is never free"  # => co-28
    print(f"MATCH: a {size_reduction:.0%} parameter reduction and {latency_reduction:.0%} latency reduction cost {pass_rate_cost:.0%} pass-rate points -- exactly the trade distillation is meant to buy")  # => co-27,co-28
    # => co-27,co-28: this is a latency and cost optimization, not a quality technique -- the student is never expected to match the teacher, only approach it
