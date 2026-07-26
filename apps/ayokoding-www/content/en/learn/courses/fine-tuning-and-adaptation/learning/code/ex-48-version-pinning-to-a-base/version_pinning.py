# learning/code/ex-48-version-pinning-to-a-base/version_pinning.py
"""Worked Example 48: Version-Pinning to a Base."""  # => co-30: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from dataclasses import dataclass  # => co-30: a small, self-documenting record for an adapter's explicit base-version pin


@dataclass(frozen=True)  # => co-21: frozen -- a pin is a fixed fact about how an adapter was trained, not a mutable running total
class PinnedAdapter:  # => co-21: an adapter, EXPLICITLY tied to the exact base version it was trained against
    name: str  # => co-21: which adapter this is
    trained_against_base_version: str  # => co-30: the exact base-model version string this adapter's weights assume


def is_compatible_with_base(adapter: PinnedAdapter, currently_deployed_base_version: str) -> bool:  # => co-30: does the pin still hold
    """Return whether `adapter`'s pinned base version matches `currently_deployed_base_version`."""  # => co-30: documents is_compatible_with_base's contract -- no runtime output, just sets its __doc__
    return adapter.trained_against_base_version == currently_deployed_base_version  # => co-30: returns this computed value to the caller


TRIAGE_ADAPTER = PinnedAdapter(name="triage-v1", trained_against_base_version="qwen2.5-0.5b-instruct-r1")  # => co-30: pinned to the base version it was actually trained against


if __name__ == "__main__":  # => co-30: entry point -- runs only when this file executes directly, not on import
    deployed_base_version_before_upgrade = "qwen2.5-0.5b-instruct-r1"  # => co-30: matches the adapter's own pin -- the world at training time
    compatible_before_upgrade = is_compatible_with_base(TRIAGE_ADAPTER, deployed_base_version_before_upgrade)  # => co-30: check compatibility BEFORE any base upgrade
    print(f"Before upgrade: base {deployed_base_version_before_upgrade!r}, adapter pinned to {TRIAGE_ADAPTER.trained_against_base_version!r} -> compatible: {compatible_before_upgrade}")  # => co-30
    assert compatible_before_upgrade, "the adapter must be compatible with the base version it was actually trained against"  # => co-30
    deployed_base_version_after_upgrade = "qwen2.5-0.5b-instruct-r2"  # => co-30: the platform team ships a newer base-model release
    compatible_after_upgrade = is_compatible_with_base(TRIAGE_ADAPTER, deployed_base_version_after_upgrade)  # => co-30: check the SAME adapter against the NEW base
    print(f"After upgrade: base {deployed_base_version_after_upgrade!r}, adapter still pinned to {TRIAGE_ADAPTER.trained_against_base_version!r} -> compatible: {compatible_after_upgrade}")  # => co-30
    assert not compatible_after_upgrade, "the SAME adapter must be flagged incompatible once the deployed base version moves past its pin"  # => co-30
    re_adaptation_required = not compatible_after_upgrade  # => co-30: the standing maintenance obligation this incompatibility creates
    print(f"Re-adaptation required before serving triage-v1 against the new base: {re_adaptation_required}")  # => co-08,co-30
    assert re_adaptation_required, "an incompatible pin must translate into an explicit re-adaptation requirement, never a silent mismatch"  # => co-08,co-30
    print("MATCH: the pin correctly flags the SAME adapter as incompatible the moment its base is upgraded -- a recurring cost, not a one-time one")  # => co-08,co-30
    # => co-08,co-30: this is the maintenance obligation ex-14 introduced, made concrete for the serving layer -- every base upgrade re-opens this check
