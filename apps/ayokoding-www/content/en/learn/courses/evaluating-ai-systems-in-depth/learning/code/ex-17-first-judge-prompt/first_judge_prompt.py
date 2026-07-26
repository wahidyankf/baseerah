"""Worked Example 17: A Judge Scoring One Operationalized Criterion Returns a Parseable Verdict."""  # => co-09: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import json  # => co-09: the judge's response is structured JSON text, not free prose
from typing import NamedTuple  # => co-09: JudgeVerdict is a typed record, parsed from the judge's raw text


class JudgeVerdict(NamedTuple):  # => co-09: what a judge call must return -- machine-parseable, not prose
    passed: bool  # => co-09: the judge's binary verdict
    reason: str  # => co-15: a short, single-question rubric -- the judge explains its ONE verdict, not several


CRITERION = "The reply must ask a clarifying question before acting, whenever the request names no specific board."  # => co-15: a single, binary, unambiguous rubric question


def mock_judge_model(reply: str) -> str:  # => co-09: stands in for a real judge model's raw text response
    """Return a raw JSON string -- a mocked stand-in for an LLM judge's structured output."""  # => co-09: documents mock_judge_model's contract -- no runtime output, just sets its __doc__
    asks_which_board = "which board" in reply.lower() or "which project" in reply.lower()  # => co-15: the judge's own read of the rubric question
    verdict = {"passed": asks_which_board, "reason": "asks which board" if asks_which_board else "acts without clarifying"}  # => co-09: the judge's raw structured answer
    return json.dumps(verdict)  # => co-09: serialized exactly as a real judge model would return it


def parse_judge_verdict(raw_response: str) -> JudgeVerdict:  # => co-09: turns the judge's raw text into a typed, checkable value
    """Parse a judge's raw JSON text into a typed JudgeVerdict, failing loudly on a malformed response."""  # => co-09: documents parse_judge_verdict's contract -- no runtime output, just sets its __doc__
    data = json.loads(raw_response)  # => co-09: raises immediately on non-JSON -- never silently swallowed
    return JudgeVerdict(passed=bool(data["passed"]), reason=str(data["reason"]))  # => co-09: returns this computed value to the caller


if __name__ == "__main__":  # => co-09: entry point -- runs only when this file executes directly, not on import
    ambiguous_request = "Move this card to done."  # => co-09: the request names no specific board -- the criterion applies
    reply_clarifies = "Sure -- which board should I move it on?"  # => co-09: a reply that DOES ask first
    reply_acts_blindly = "Done -- moved to the Done column."  # => co-09: a reply that acts WITHOUT asking

    raw_clarifies = mock_judge_model(reply_clarifies)  # => co-09: the judge's raw response to the good reply
    raw_acts_blindly = mock_judge_model(reply_acts_blindly)  # => co-09: the judge's raw response to the bad reply
    print(f"Judge raw response (clarifies): {raw_clarifies}")  # => co-09: prints the RAW, machine-parseable text
    print(f"Judge raw response (acts blindly): {raw_acts_blindly}")  # => co-09: prints the RAW, machine-parseable text

    verdict_clarifies = parse_judge_verdict(raw_clarifies)  # => co-09: parse into a typed, checkable verdict
    verdict_acts_blindly = parse_judge_verdict(raw_acts_blindly)  # => co-09: parse into a typed, checkable verdict
    print(f"Parsed: {verdict_clarifies}")  # => co-09: prints the typed verdict
    print(f"Parsed: {verdict_acts_blindly}")  # => co-09: prints the typed verdict

    assert verdict_clarifies.passed is True, "the clarifying reply must be judged as passing"  # => co-09: the rule this example proves
    assert verdict_acts_blindly.passed is False, "the blindly-acting reply must be judged as failing"  # => co-09
    assert isinstance(verdict_clarifies, JudgeVerdict), "the parsed result must be a typed JudgeVerdict, not raw text"  # => co-09: parseability check
    print(f"MATCH: judge scores {CRITERION!r} and returns a parseable verdict for both replies")  # => co-09
    # => co-09: ex-18 next measures whether THIS judge's verdicts actually agree with real human labels
