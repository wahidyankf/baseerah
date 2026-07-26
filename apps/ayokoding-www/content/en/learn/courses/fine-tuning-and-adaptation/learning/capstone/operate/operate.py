# learning/capstone/operate/operate.py
"""Capstone Step 5: Operation (exercises co-21, co-29, co-30, co-32)."""  # => co-29: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

import json  # => co-29: the committed operate-result artefact this step writes, closing the capstone's own artefact chain
from dataclasses import dataclass, field  # => co-21: a served adapter is a small, composable artefact -- this class models exactly that shape, matching ex-45/ex-46
from pathlib import Path  # => co-29: locates the prior steps' committed artefacts and this step's own committed artefact
from typing import TypedDict, cast  # => co-29: types the committed artefacts this step reads and writes

EVALUATE_RESULT_PATH = Path(__file__).parent.parent / "evaluate" / "evaluate_result.json"  # => co-25: the prior step's own committed artefact
TRAIN_RESULT_PATH = Path(__file__).parent.parent / "train" / "train_result.json"  # => co-20: the training step's own committed artefact
RESULT_PATH = Path(__file__).parent / "operate_result.json"  # => co-29: this step's own committed artefact -- the capstone's final artefact


class EvaluateResult(TypedDict):  # => co-25: mirrors evaluate.py's own committed shape -- only the fields this step actually needs
    target_pass_rate_adapted: float  # => co-25: the newly evaluated adapter's own held-out pass rate
    significant: bool  # => co-25: must be True before this adapter is rolled out
    forgetting_detected: bool  # => co-22: must be False before this adapter is rolled out
    base_model_id: str  # => co-30: pinned, carried forward unchanged


class TrainResult(TypedDict):  # => co-20: mirrors train.py's own committed shape -- only the fields this step actually needs
    chosen_rank: int  # => co-20: reported in the final operational summary
    base_model_id: str  # => co-30: pinned, carried forward unchanged


@dataclass(frozen=True)  # => co-21: frozen -- a served adapter's identity is fixed once loaded, matching this course's own ex-45
class Adapter:  # => co-21: a small, versionable artefact -- NOT a whole new model
    name: str  # => co-21: which adapter this is
    base_model_id: str  # => co-30: which base this adapter is pinned to
    pass_rate: float  # => co-25: this adapter's own measured pass rate, for the operational rollout decision


@dataclass  # => co-29: a mock server -- ONE base model in memory, adapters attached and swapped on top of it, matching ex-45/ex-46
class MockServer:  # => co-29: stands in for the real serving stack this course's prerequisite topic covers in depth
    base_model_id: str  # => co-29: the single loaded base model this server holds in memory
    loaded_adapters: dict[str, Adapter] = field(default_factory=dict[str, Adapter])  # => co-21: adapters currently attached, keyed by name
    active_adapter_name: str | None = None  # => co-21: which adapter is currently routing requests

    def load_adapter(self, adapter: Adapter) -> None:  # => co-30: attach an adapter, rejecting a base mismatch at load time
        """Attach `adapter` to this server, raising if its `base_model_id` does not match the loaded base."""  # => co-29: documents load_adapter's contract -- no runtime output, just sets its __doc__
        if adapter.base_model_id != self.base_model_id:  # => co-30: an adapter trained against one base cannot simply be attached to another
            raise ValueError(f"adapter {adapter.name!r} was trained against {adapter.base_model_id!r}, not the loaded base {self.base_model_id!r}")  # => co-30
        self.loaded_adapters[adapter.name] = adapter  # => co-21: attach it -- cheap, because the adapter is small

    def switch_to(self, adapter_name: str) -> None:  # => co-21: the hot-swap itself -- no base reload
        """Set `active_adapter_name` to `adapter_name`, raising if it is not loaded."""  # => co-21: documents switch_to's contract -- no runtime output, just sets its __doc__
        if adapter_name not in self.loaded_adapters:  # => co-21: cannot switch to an adapter that was never loaded
            raise KeyError(f"adapter {adapter_name!r} is not loaded")  # => co-21
        self.active_adapter_name = adapter_name  # => co-21: the swap -- no base reload, no downtime


def trigger_is_concrete(trigger: str) -> bool:  # => co-30,co-32: a real trigger names a checkable CONDITION, not a vague aspiration -- matches ex-75's own check
    """Return whether `trigger` reads as a concrete, checkable condition rather than a vague statement."""  # => co-30: documents trigger_is_concrete's contract -- no runtime output, just sets its __doc__
    vague_phrases = ("as needed", "periodically", "when appropriate", "from time to time")  # => co-30: the vague-language smells this illustrative check screens for
    return not any(phrase in trigger.lower() for phrase in vague_phrases) and len(trigger) > 20  # => co-30: not vague AND substantive enough to be an actual condition


class OperateResult(TypedDict):  # => co-29: the capstone's own FINAL committed artefact, closing the five-step arc
    served: bool  # => co-29: the adapter successfully loaded and served a request
    hot_swap_verified: bool  # => co-21: the swap from the previous adapter to this one succeeded
    version_pinned: bool  # => co-30: the served adapter's own base pin matches its training-time base
    re_adaptation_trigger_concrete: bool  # => co-30: the re-adaptation condition is checkable, not vague
    retirement_trigger_concrete: bool  # => co-32: the retirement condition is checkable, not vague
    adaptation_justified_end_to_end: bool  # => co-06: the capstone's own final verdict


if __name__ == "__main__":  # => co-29: entry point -- runs only when this file executes directly, not on import
    evaluate_raw = cast(EvaluateResult, json.loads(EVALUATE_RESULT_PATH.read_text(encoding="utf-8")))  # => co-25: read the evaluation step's own committed artefact
    train_raw = cast(TrainResult, json.loads(TRAIN_RESULT_PATH.read_text(encoding="utf-8")))  # => co-20: read the training step's own committed artefact
    assert evaluate_raw["significant"] and not evaluate_raw["forgetting_detected"], "operation must only proceed once the evaluation step's own bar is cleared"  # => co-22,co-25

    server = MockServer(base_model_id=evaluate_raw["base_model_id"])  # => co-29: ONE base, loaded once, matching the pinned base from every prior step
    previous_adapter = Adapter(name="triage-v0", base_model_id=evaluate_raw["base_model_id"], pass_rate=0.75)  # => co-32: the previously-deployed adapter this rollout replaces
    new_adapter = Adapter(name="triage-v1", base_model_id=evaluate_raw["base_model_id"], pass_rate=evaluate_raw["target_pass_rate_adapted"])  # => co-25: THIS capstone's own newly trained and evaluated adapter
    server.load_adapter(previous_adapter)  # => co-29: the previously-deployed adapter, still loaded
    server.load_adapter(new_adapter)  # => co-30: the new adapter, rejected at load time if its base pin did not match -- it matches here
    server.switch_to("triage-v0")  # => co-21: start from the currently-deployed adapter, as a real rollout would
    server.switch_to("triage-v1")  # => co-21: the hot-swap -- no base reload between requests
    print(f"Active adapter after rollout: {server.active_adapter_name} (previous: {previous_adapter.pass_rate:.0%}, new: {new_adapter.pass_rate:.0%})")  # => co-21,co-25
    hot_swap_verified = server.active_adapter_name == "triage-v1" and new_adapter.pass_rate > previous_adapter.pass_rate  # => co-21: swap succeeded AND the new adapter genuinely beats the old one
    assert hot_swap_verified, "the hot-swap must land on the new adapter, and the new adapter must genuinely beat the one it replaced"  # => co-21,co-25
    served = server.active_adapter_name == "triage-v1"  # => co-29: the adapter is now actively serving
    version_pinned = new_adapter.base_model_id == train_raw["base_model_id"] == evaluate_raw["base_model_id"]  # => co-30: the SAME base pin, traced through every prior step
    print(f"Served: {served} | Hot-swap verified: {hot_swap_verified} | Version-pinned to {new_adapter.base_model_id!r}: {version_pinned}")  # => co-29,co-21,co-30
    assert version_pinned, "the served adapter's base pin must match the base pin from every prior step, unchanged"  # => co-30

    re_adaptation_trigger = "the pinned base version is superseded by a new release the platform team deploys"  # => co-30: matches this course's own ex-48/ex-75 scenario
    retirement_trigger = "a retrieval-based or prompting-based alternative measurably beats this adapter on both pass rate and monthly cost"  # => co-32: matches this course's own ex-49/ex-75 scenario
    re_adaptation_concrete = trigger_is_concrete(re_adaptation_trigger)  # => co-30: verify the re-adaptation condition is actually checkable
    retirement_concrete = trigger_is_concrete(retirement_trigger)  # => co-32: verify the retirement condition is actually checkable
    print(f"Re-adaptation trigger concrete: {re_adaptation_concrete} | Retirement trigger concrete: {retirement_concrete}")  # => co-30,co-32
    assert re_adaptation_concrete and retirement_concrete, "both the maintenance and retirement triggers must be concrete, checkable conditions, not vague aspirations"  # => co-30,co-32

    adaptation_justified_end_to_end = all(  # => co-06: the capstone's own final verdict, citing every step's own check
        (served, hot_swap_verified, version_pinned, re_adaptation_concrete, retirement_concrete, evaluate_raw["significant"], not evaluate_raw["forgetting_detected"])  # => co-06
    )  # => co-06: closes the all(...) check
    print(f"Adaptation is justified end to end: {adaptation_justified_end_to_end}")  # => co-06
    assert adaptation_justified_end_to_end, "every one of the five steps' own checks must hold for this capstone to close as justified"  # => co-06

    result: OperateResult = {  # => co-29: the capstone's own FINAL committed artefact
        "served": served,  # => co-29
        "hot_swap_verified": hot_swap_verified,  # => co-21
        "version_pinned": version_pinned,  # => co-30
        "re_adaptation_trigger_concrete": re_adaptation_concrete,  # => co-30
        "retirement_trigger_concrete": retirement_concrete,  # => co-32
        "adaptation_justified_end_to_end": adaptation_justified_end_to_end,  # => co-06
    }  # => co-29: closes result
    RESULT_PATH.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n", encoding="utf-8")  # => co-29: commits the capstone's own final artefact
    print(f"MATCH: operation result committed to {RESULT_PATH.name} -- served, hot-swapped, version-pinned, and both maintenance triggers concrete")  # => co-21,co-29,co-30,co-32
    # => co-06,co-21,co-25,co-29,co-30,co-32: five committed artefacts, one per step, chain into a single justified, evidenced adaptation -- exactly the arc ex-50 cites by example number, this time run as one real small project
