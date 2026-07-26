"""Worked Example 74: Retry a Timed-Out Judge Call Within a Bounded Budget, Then Fail Loudly."""  # => co-25: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import NamedTuple  # => co-25: RetryOutcome is a typed record describing exactly what happened across retries


class RetryOutcome(NamedTuple):  # => co-25: what a bounded-retry judge call actually did, for the CI log
    succeeded: bool  # => co-25: whether the call eventually got a real response
    attempts_used: int  # => co-25: how many attempts it took
    gave_up: bool  # => co-25: whether it exhausted the retry budget without success


def mock_judge_call(attempt_number: int, *, fails_until_attempt: int) -> bool:  # => co-25: a mocked judge call -- fails (times out) on early attempts, succeeds once `attempt_number` reaches `fails_until_attempt`
    """Return True (call succeeded) iff `attempt_number >= fails_until_attempt`."""  # => co-25: documents mock_judge_call's contract -- no runtime output, just sets its __doc__
    return attempt_number >= fails_until_attempt  # => co-25: returns this computed value to the caller


def call_judge_with_bounded_retries(*, fails_until_attempt: int, max_attempts: int = 3) -> RetryOutcome:  # => co-25: BOUNDED retry -- never retries forever, always terminates with a clear outcome
    """Retry `mock_judge_call` up to `max_attempts` times, returning a `RetryOutcome` describing what happened."""  # => co-25: documents call_judge_with_bounded_retries's contract -- no runtime output, just sets its __doc__
    for attempt in range(1, max_attempts + 1):  # => co-25: bounded loop -- never an unbounded retry storm
        if mock_judge_call(attempt, fails_until_attempt=fails_until_attempt):  # => co-25: this attempt succeeded
            return RetryOutcome(succeeded=True, attempts_used=attempt, gave_up=False)  # => co-25: returns this computed value to the caller
    return RetryOutcome(succeeded=False, attempts_used=max_attempts, gave_up=True)  # => co-25: exhausted the budget -- fails LOUDLY, not silently


if __name__ == "__main__":  # => co-25: entry point -- runs only when this file executes directly, not on import
    recovers_on_retry = call_judge_with_bounded_retries(fails_until_attempt=2)  # => co-25: a transient timeout that recovers within the retry budget
    never_recovers = call_judge_with_bounded_retries(fails_until_attempt=10)  # => co-25: a persistent failure that exhausts the retry budget
    print(f"Transient timeout: succeeded={recovers_on_retry.succeeded}, attempts={recovers_on_retry.attempts_used}, gave_up={recovers_on_retry.gave_up}")  # => co-25
    print(f"Persistent failure: succeeded={never_recovers.succeeded}, attempts={never_recovers.attempts_used}, gave_up={never_recovers.gave_up}")  # => co-25

    assert recovers_on_retry.succeeded is True and recovers_on_retry.attempts_used == 2, "a transient failure that clears within the retry budget must succeed, using exactly the attempts it needed"  # => co-25: the rule this example proves
    assert never_recovers.gave_up is True and never_recovers.attempts_used == 3, "a persistent failure must exhaust the BOUNDED retry budget (3 attempts) and give up loudly, not retry forever"  # => co-25: the rule this example proves
    print(  # => co-25: opens the final MATCH print, reached only if both asserts above passed
        f"MATCH: a transient judge-call timeout recovers within {recovers_on_retry.attempts_used} attempts, while a persistent failure exhausts the bounded {never_recovers.attempts_used}-attempt budget and reports gave_up=True instead of retrying forever"
    )  # => co-25
    # => co-25: ex-75 next distinguishes a genuinely FLAKY eval case from a REAL regression -- a related, but distinct, CI-reliability concern
