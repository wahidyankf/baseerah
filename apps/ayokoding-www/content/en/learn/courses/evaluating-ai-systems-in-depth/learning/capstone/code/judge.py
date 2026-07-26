"""Capstone Step 3: A Judge on a Different Model Than the Generator, Validated Per Criterion With a CI."""  # => co-09/co-12: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import json  # => co-11: ground_truth.jsonl is read as real, adjudicated JSON lines
import math  # => co-10: the Wilson score interval needs only sqrt -- no external stats library required
import re  # => co-09: the criterion-3 judge heuristic extracts a digit sequence from free text
from pathlib import Path  # => co-11: locates ground_truth.jsonl relative to this file, not the caller's cwd
from typing import NamedTuple  # => co-11: every stage below uses a typed record, not a bare dict

GROUND_TRUTH_PATH = Path(__file__).parent / "ground_truth.jsonl"  # => co-11: resolved relative to THIS file
AGREEMENT_THRESHOLD = 0.85  # => co-12: labeling-guide.md's own justified threshold, reused here as the judge's validation bar

# The AGENT under test (the "generator") is a DIFFERENT model from this judge -- per co-16's rule,
# never use the same model to both generate and grade its own output. This judge is a smaller,
# cheaper, independently-configured model, illustrated here as a set of deterministic heuristics
# standing in for a real second model's API responses.
GENERATOR_MODEL_NAME = "tasklight-agent-v3"  # => co-16: the model under test
JUDGE_MODEL_NAME = "tasklight-judge-v1"  # => co-16: a DIFFERENT, separately-configured model doing the grading


class GroundTruthCase(NamedTuple):  # => co-11: one adjudicated, human-labeled case
    case_id: str  # => co-11
    criterion_id: int  # => co-11: which of criteria.md's four criteria this case tests
    request: str  # => co-11
    reply: str  # => co-11
    human_label: bool  # => co-11: the adjudicated ground truth
    reference_count: int | None  # => co-11: only present for criterion-3 (aggregate-count) cases


class AgreementInterval(NamedTuple):  # => co-10: a proportion, reported WITH its uncertainty, never alone -- same shape as ex-19
    criterion_id: int  # => co-12: scoped PER criterion, not one blended figure
    point_estimate: float  # => co-10: the raw agreement rate
    lower_bound: float  # => co-10: the interval's lower edge
    upper_bound: float  # => co-10: the interval's upper edge
    sample_size: int  # => co-10: how many cases this criterion's estimate rests on


def _as_int(value: object) -> int:  # => co-11: a strict-typing-safe narrowing helper -- int(object) is rejected by pyright strict, this is the explicit fix
    """Return `value` as an int, raising TypeError if it is not already an int (JSON numbers decode as int or float, never bool-like ints here)."""  # => co-11: documents _as_int's contract -- no runtime output, just sets its __doc__
    if isinstance(value, int) and not isinstance(value, bool):  # => co-11: excludes bool, since bool is technically an int subclass in Python
        return value  # => co-11: returns this computed value to the caller
    raise TypeError(f"expected an int, got {value!r}")  # => co-11: fails loudly on a malformed record rather than silently coercing


def _as_bool(value: object) -> bool:  # => co-11: a strict-typing-safe narrowing helper -- bool(object) is rejected by pyright strict, this is the explicit fix
    """Return `value` as a bool, raising TypeError if it is not already a bool."""  # => co-11: documents _as_bool's contract -- no runtime output, just sets its __doc__
    if isinstance(value, bool):  # => co-11: narrows the object type to bool explicitly
        return value  # => co-11: returns this computed value to the caller
    raise TypeError(f"expected a bool, got {value!r}")  # => co-11: fails loudly on a malformed record rather than silently coercing


def load_ground_truth(path: Path = GROUND_TRUTH_PATH) -> tuple[GroundTruthCase, ...]:  # => co-11: reads the real, adjudicated reference set from disk
    """Return every line of `path` as a `GroundTruthCase`, parsed from JSON."""  # => co-11: documents load_ground_truth's contract -- no runtime output, just sets its __doc__
    cases: list[GroundTruthCase] = []  # => co-11: accumulates one GroundTruthCase per JSONL line
    for line in path.read_text(encoding="utf-8").splitlines():  # => co-11: reads the file once, line by line
        data: dict[str, object] = json.loads(line)  # => co-11: parses this line's raw JSON
        cases.append(  # => co-11: builds a typed record from the parsed fields
            GroundTruthCase(  # => co-11
                case_id=str(data["case_id"]),  # => co-11
                criterion_id=_as_int(data["criterion_id"]),  # => co-11
                request=str(data["request"]),  # => co-11
                reply=str(data["reply"]),  # => co-11
                human_label=_as_bool(data["human_label"]),  # => co-11
                reference_count=_as_int(data["reference_count"]) if "reference_count" in data else None,  # => co-11: absent for criteria that do not need a numeric reference
            )  # => co-11
        )  # => co-11: closes cases.append(...)
    return tuple(cases)  # => co-11: returns this computed value to the caller


def mock_judge_verdict(case: GroundTruthCase) -> bool:  # => co-09: the judge model's own verdict, scoped per criterion -- one heuristic branch per criterion, standing in for one judge prompt per criterion
    """Return the judge's True/False verdict for `case`, using per-criterion logic."""  # => co-09: documents mock_judge_verdict's contract -- no runtime output, just sets its __doc__
    if case.criterion_id == 1:  # => co-09: criterion 1 -- asks before acting on an ambiguous target
        request_names_explicit_id = bool(re.search(r"#\d+", case.request))  # => co-09: if the REQUEST already names an explicit ticket ID, there was no real ambiguity to clarify
        return "which" in case.reply.lower() or request_names_explicit_id  # => co-09: the judge's own read of criterion 1 -- passes if it asked, OR if nothing needed asking
    if case.criterion_id == 2:  # => co-09: criterion 2 -- acts on the exact target named
        requested_id = re.search(r"#(\d+)", case.request)  # => co-09: extracts the requested ticket ID
        acted_id = re.search(r"#(\d+)", case.reply)  # => co-09: extracts the ID the reply actually acted on
        return bool(requested_id and acted_id and requested_id.group(1) == acted_id.group(1))  # => co-09: the judge's own read of criterion 2
    if case.criterion_id == 3:  # => co-09: criterion 3 -- reports the true aggregate count
        found_digits = re.search(r"\d+", case.reply)  # => co-09: a REAL judge limitation -- this heuristic can only extract a plain digit sequence
        if found_digits is None:  # => co-09: the reply used a vague phrase ("roughly a dozen") with no extractable digit
            return True  # => co-09: a genuine, documented judge FAILURE MODE -- defaults to True when it cannot parse a number, rather than correctly flagging vagueness as wrong
        return int(found_digits.group(0)) == case.reference_count  # => co-09: the judge's own read of criterion 3 when a digit IS present
    if "all tickets" in case.reply.lower():  # => co-09: criterion 4 -- applies every stated filter condition; "all tickets" signals the filter was ignored
        return False  # => co-09: the judge's own read of criterion 4
    return True  # => co-09: no "all tickets" phrase -- the judge reads this as the filter having been applied


def wilson_interval(successes: int, total: int, *, criterion_id: int, z: float = 1.96) -> AgreementInterval:  # => co-10: the SAME Wilson interval formula as ex-19, scoped here per criterion
    """Compute a Wilson score confidence interval for `successes`/`total`, tagged with `criterion_id`."""  # => co-10: documents wilson_interval's contract -- no runtime output, just sets its __doc__
    p_hat = successes / total  # => co-10: the raw point estimate for THIS criterion
    denominator = 1 + z**2 / total  # => co-10: Wilson's correction term
    center = p_hat + z**2 / (2 * total)  # => co-10: the interval's recentered midpoint
    spread = z * math.sqrt(p_hat * (1 - p_hat) / total + z**2 / (4 * total**2))  # => co-10: the interval's half-width
    lower = max(0.0, (center - spread) / denominator)  # => co-10: clamps to a valid probability floor
    upper = min(1.0, (center + spread) / denominator)  # => co-10: clamps to a valid probability ceiling
    return AgreementInterval(criterion_id=criterion_id, point_estimate=p_hat, lower_bound=lower, upper_bound=upper, sample_size=total)  # => co-10: returns this computed value to the caller


def measure_agreement_per_criterion(cases: tuple[GroundTruthCase, ...]) -> tuple[AgreementInterval, ...]:  # => co-12: agreement is measured PER criterion -- never one blended figure across unrelated criteria
    """Return one `AgreementInterval` per distinct `criterion_id` present in `cases`."""  # => co-12: documents measure_agreement_per_criterion's contract -- no runtime output, just sets its __doc__
    by_criterion: dict[int, list[bool]] = {}  # => co-12: groups per-case match results by criterion
    for case in cases:  # => co-09: run the judge on every case
        judge_verdict = mock_judge_verdict(case)  # => co-09: this case's judge verdict
        matches = judge_verdict == case.human_label  # => co-17: measured agreement, not assumed
        by_criterion.setdefault(case.criterion_id, []).append(matches)  # => co-12: records this case's match under its own criterion
    intervals = [wilson_interval(sum(matches), len(matches), criterion_id=cid) for cid, matches in sorted(by_criterion.items())]  # => co-10: one CI per criterion, criteria in ascending order
    return tuple(intervals)  # => co-12: returns this computed value to the caller


def position_bias_probe() -> bool:  # => co-13: the SAME position-bias probe pattern as ex-79, run here as part of the capstone's own validation
    """Return True iff a position-bias probe detects a flip purely from slot order."""  # => co-13: documents position_bias_probe's contract -- no runtime output, just sets its __doc__
    verdict_first = "compliant"  # => co-13: mock judge verdict when the compliant reply is shown first
    verdict_second = "compliant"  # => co-13: the SAME mock judge verdict when the compliant reply is shown second
    return verdict_first != verdict_second  # => co-13: returns this computed value to the caller


def verbosity_bias_probe() -> bool:  # => co-13: the SAME verbosity-bias probe pattern as ex-79
    """Return True iff a padded, longer reply scores HIGHER than an equally-correct short one."""  # => co-13: documents verbosity_bias_probe's contract -- no runtime output, just sets its __doc__
    short_score, long_score = 8, 6  # => co-13: mock judge scores -- the longer, padded reply correctly scores LOWER
    return long_score > short_score  # => co-13: returns this computed value to the caller


if __name__ == "__main__":  # => co-09: entry point -- runs only when this file executes directly, not on import
    print(f"Generator model: {GENERATOR_MODEL_NAME}, judge model: {JUDGE_MODEL_NAME} (different models, per co-16)")  # => co-16: states the model separation explicitly
    ground_truth = load_ground_truth()  # => co-11: load the real, adjudicated reference set
    intervals = measure_agreement_per_criterion(ground_truth)  # => co-12: measure agreement, scoped per criterion, with a CI

    retired_criteria: list[int] = []  # => co-22: criteria whose judge falls below threshold get retired, not shipped
    for interval in intervals:  # => co-12: reports and validates every criterion independently
        status = "OK" if interval.point_estimate >= AGREEMENT_THRESHOLD else "RETIRE -- use the deterministic reference-based scorer instead"  # => co-22
        print(f"Criterion {interval.criterion_id}: agreement {interval.point_estimate:.0%} [{interval.lower_bound:.0%}, {interval.upper_bound:.0%}] on n={interval.sample_size} -- {status}")  # => co-12
        if interval.point_estimate < AGREEMENT_THRESHOLD:  # => co-22: below the justified threshold
            retired_criteria.append(interval.criterion_id)  # => co-22: tracks which criteria this judge must NOT be trusted on

    position_biased = position_bias_probe()  # => co-13: run the position-bias probe
    verbosity_biased = verbosity_bias_probe()  # => co-13: run the verbosity-bias probe
    print(f"Position-bias probe: bias_detected={position_biased}")  # => co-13
    print(f"Verbosity-bias probe: bias_detected={verbosity_biased}")  # => co-13
    print(f"Criteria retired (judge below {AGREEMENT_THRESHOLD:.0%} threshold): {retired_criteria}")  # => co-22

    assert len(ground_truth) == 13, "the capstone's own ground-truth set must contain exactly the thirteen adjudicated cases on disk"  # => co-11: the rule this example proves
    assert len(intervals) == 4, "agreement must be measured for all four of criteria.md's criteria, none skipped"  # => co-12: the rule this example proves
    assert retired_criteria == [3], "criterion 3's judge -- which cannot parse vague count phrases like 'roughly a dozen' -- must fall below threshold and be retired"  # => co-22: the rule this example proves
    assert position_biased is False and verbosity_biased is False, "the judge must pass both bias probes before being trusted on the criteria it was NOT retired from"  # => co-13: the rule this example proves
    print("MATCH: agreement measured per criterion with a Wilson CI, criterion 3 correctly retired (a real judge limitation, not a made-up one), and both bias probes pass clean")  # => co-12
    # => co-16: Step 4 next scores the agent's TRAJECTORY, not just its final answer, using the criteria that survived validation
