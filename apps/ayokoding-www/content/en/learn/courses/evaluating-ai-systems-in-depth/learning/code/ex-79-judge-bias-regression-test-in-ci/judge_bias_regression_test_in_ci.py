"""Worked Example 79: Run a Judge-Bias Probe AS a CI Check, So Bias Creep Blocks a Merge Too."""  # => co-13: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import NamedTuple  # => co-13: BiasProbeResult is a typed record -- the CI-gate-shaped verdict for a bias probe


class BiasProbeResult(NamedTuple):  # => co-13: a bias probe's own CI-shaped verdict, not just a diagnostic print
    probe_name: str  # => co-13: which bias mode this probe checks
    bias_detected: bool  # => co-13: whether the judge showed the bias on this probe
    blocks_merge: bool  # => co-23: whether THIS probe result should block a merge, same shape as the regression gate


def verbosity_bias_probe() -> BiasProbeResult:  # => co-13: the SAME verbosity-bias probe pattern as ex-26/ex-51, wired as a CI check
    """Return a `BiasProbeResult` for a verbosity-bias check -- a longer, padded reply scored against a short, correct one."""  # => co-13: documents verbosity_bias_probe's contract -- no runtime output, just sets its __doc__
    short_correct_reply = "Yes, offline sync is available on the Pro plan."  # => co-13: short, fully correct
    long_padded_reply = "Great question! Let me walk you through this in detail. Offline sync is indeed available, and it is one of the many features we offer, specifically on the Pro plan tier, which you may find useful."  # => co-13: much longer, no more correct
    short_score = 8  # => co-13: a validated judge's mock score for the short reply (out of 10)
    long_score = 6  # => co-13: a validated judge's mock score for the long reply -- LOWER, correctly penalizing padding, not rewarding length
    del short_correct_reply, long_padded_reply  # => co-13: text kept for documentation clarity; only the mock scores drive the check
    bias_detected = long_score > short_score  # => co-13: bias would mean the judge WRONGLY preferred the longer reply
    return BiasProbeResult(probe_name="verbosity-bias", bias_detected=bias_detected, blocks_merge=bias_detected)  # => co-23: returns this computed value to the caller


def position_bias_probe() -> BiasProbeResult:  # => co-13: a position-bias probe, wired as a CI check
    """Return a `BiasProbeResult` for a position-bias check -- the SAME pair judged in both slot orders."""  # => co-13: documents position_bias_probe's contract -- no runtime output, just sets its __doc__
    verdict_when_a_is_first = "A"  # => co-13: a mock judge's verdict when reply A is shown first
    verdict_when_a_is_second = "A"  # => co-13: the SAME mock judge's verdict when reply A is shown second -- consistent, no positional flip
    bias_detected = verdict_when_a_is_first != verdict_when_a_is_second  # => co-13: bias would mean the verdict flips purely from slot order
    return BiasProbeResult(probe_name="position-bias", bias_detected=bias_detected, blocks_merge=bias_detected)  # => co-23: returns this computed value to the caller


if __name__ == "__main__":  # => co-23: entry point -- runs only when this file executes directly, not on import
    probes = (verbosity_bias_probe(), position_bias_probe())  # => co-13: run every bias probe, exactly like running the fast tier's ordinary cases
    for p in probes:  # => co-13: prints each probe's own verdict
        print(f"{p.probe_name}: bias_detected={p.bias_detected}, blocks_merge={p.blocks_merge}")  # => co-13
    any_blocks = any(p.blocks_merge for p in probes)  # => co-23: the overall CI-gate decision -- ANY detected bias blocks the merge
    print(f"Overall: merge blocked by bias probes = {any_blocks}")  # => co-23

    assert probes[0].bias_detected is False, "the validated judge must NOT show verbosity bias -- it must not have crept back in"  # => co-13: the rule this example proves
    assert probes[1].bias_detected is False, "the validated judge must NOT show position bias either"  # => co-13: the rule this example proves
    assert any_blocks is False, "with no bias detected on either probe, the merge must NOT be blocked"  # => co-23: the rule this example proves
    print(f"MATCH: both bias probes run as ordinary CI checks (blocks_merge={any_blocks}) -- if either had detected bias creep, the merge itself would have been blocked, not just logged as a diagnostic")  # => co-23
    # => co-23: ex-80 next assembles a MINIATURE end-to-end dry run of the entire pipeline, one last integrated check before the real capstone
