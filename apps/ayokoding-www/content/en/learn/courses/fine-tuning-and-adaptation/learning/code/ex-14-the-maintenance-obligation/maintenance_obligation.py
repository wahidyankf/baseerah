# learning/code/ex-14-the-maintenance-obligation/maintenance_obligation.py
"""Worked Example 14: The Maintenance Obligation."""  # => co-30: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from dataclasses import dataclass  # => co-30: an adapter's pin is a fact worth a typed record, not a loose comment


@dataclass(frozen=True)  # => co-30: frozen -- a historical pin record should not mutate after the fact
class AdapterVersionPin:  # => co-30: what a served adapter is ACTUALLY pinned to
    adapter_name: str  # => co-30: the artefact's own name
    base_model_version: str  # => co-30: the exact base checkpoint this adapter was trained against
    trained_on_dataset_snapshot: str  # => co-30: which dataset commit produced this adapter (co-03's discipline, reused)


ADAPTER_PIN = AdapterVersionPin(  # => co-30: the vocabulary adapter from ex-08, as actually shipped
    adapter_name="ticket-vocab-adapter-v1",  # => co-30
    base_model_version="qwen2.5-0.5b-instruct@2026-03",  # => co-30: the base checkpoint this adapter was trained against
    trained_on_dataset_snapshot="ticket-vocab-dataset@commit-a3f9c1",  # => co-30
)  # => co-30: closes ADAPTER_PIN

BASE_MODEL_RELEASE_HISTORY = [  # => co-30: how often the base model this adapter depends on actually gets superseded
    "qwen2.5-0.5b-instruct@2026-03",  # => co-30: the version ADAPTER_PIN was trained against
    "qwen2.5-0.5b-instruct@2026-09",  # => co-30: a later release, six months on -- the adapter does NOT automatically transfer
    "qwen2.5-0.5b-instruct@2027-04",  # => co-30: a further release seven months after that
]  # => co-30: closes BASE_MODEL_RELEASE_HISTORY -- roughly every six to seven months, historically


def is_current(pin: AdapterVersionPin, release_history: list[str]) -> bool:  # => co-30: is this adapter still pinned to the LATEST base?
    """Pass iff `pin.base_model_version` is the most recent entry in `release_history`."""  # => co-30: documents is_current's contract -- no runtime output, just sets its __doc__
    return pin.base_model_version == release_history[-1]  # => co-30: the latest release is always the last entry, by construction here


if __name__ == "__main__":  # => co-30: entry point -- runs only when this file executes directly, not on import
    print(f"Adapter {ADAPTER_PIN.adapter_name!r} pinned to base {ADAPTER_PIN.base_model_version!r}")  # => co-30: what shipped
    still_current = is_current(ADAPTER_PIN, BASE_MODEL_RELEASE_HISTORY)  # => co-30: is the pin still the latest base, TODAY?
    print(f"Still pinned to the latest base release: {still_current}")  # => co-30
    releases_since_training = BASE_MODEL_RELEASE_HISTORY.index(BASE_MODEL_RELEASE_HISTORY[-1]) - BASE_MODEL_RELEASE_HISTORY.index(ADAPTER_PIN.base_model_version)  # => co-30
    print(f"Base-model releases since this adapter was trained: {releases_since_training}")  # => co-30: exactly how far behind it now is
    assert not still_current, "this scenario simulates the base model having moved on -- the pin must now be stale"  # => co-30
    assert releases_since_training == 2, "two base releases must have shipped since this adapter's training snapshot"  # => co-30
    print("MATCH: the adapter is pinned two base releases behind -- re-adaptation is a recurring cost, not a one-time project")  # => co-30
    # => co-08,co-30: every base-model upgrade cycle is another bill on ex-13's maintenance line, owned by whoever shipped the adapter
