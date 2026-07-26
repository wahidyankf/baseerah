# learning/code/ex-72-adapter-registry-and-discovery/adapter_registry.py
"""Worked Example 72: Adapter Registry and Discovery."""  # => co-21: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from dataclasses import dataclass, field  # => co-21: a registry entry per adapter -- discoverable metadata, not just a loaded weights blob


@dataclass(frozen=True)  # => co-21: frozen -- a registry entry's own facts are fixed once the adapter is registered
class AdapterRecord:  # => co-21: everything a caller needs to find and trust an adapter, without loading it first
    name: str  # => co-21: unique registry key
    task: str  # => co-21: which task this adapter is FOR -- what discovery actually searches on
    base_model_id: str  # => co-30: which base this adapter is pinned to
    owner: str  # => co-21: who is accountable for this adapter's maintenance obligation


@dataclass  # => co-21: a mock registry -- a small, in-memory stand-in for a real adapter catalog service
class AdapterRegistry:  # => co-21: the thing a serving layer queries BEFORE attempting to load anything
    entries: dict[str, AdapterRecord] = field(default_factory=dict[str, AdapterRecord])  # => co-21: registered adapters, keyed by name

    def register(self, record: AdapterRecord) -> None:  # => co-21: add an adapter to the registry
        """Add `record` to this registry's entries, keyed by its `name`."""  # => co-21: documents register's contract -- no runtime output, just sets its __doc__
        self.entries[record.name] = record  # => co-21: register it

    def find_for_task(self, task: str) -> AdapterRecord | None:  # => co-21: discovery -- find the right adapter WITHOUT the caller knowing its exact name
        """Return the registered `AdapterRecord` whose `task` matches `task`, or None if none is registered."""  # => co-21: documents find_for_task's contract -- no runtime output, just sets its __doc__
        for record in self.entries.values():  # => co-21: search by task, the caller's actual need, not by adapter name
            if record.task == task:  # => co-21: the first match wins -- a real registry would need a tie-break policy for multiple candidates
                return record  # => co-21: returns this computed value to the caller
        return None  # => co-21: no adapter is registered for this task


TRIAGE_RECORD = AdapterRecord(name="triage-v1", task="ticket-triage", base_model_id="qwen2.5-0.5b-instruct-r1", owner="platform-ml-team")  # => co-21: matches ex-45's own served adapter
BILLING_TONE_RECORD = AdapterRecord(name="billing-tone-v1", task="billing-response-tone", base_model_id="qwen2.5-0.5b-instruct-r1", owner="support-ops-team")  # => co-21: matches ex-46's own second adapter


if __name__ == "__main__":  # => co-21: entry point -- runs only when this file executes directly, not on import
    registry = AdapterRegistry()  # => co-21: an empty registry to start
    registry.register(TRIAGE_RECORD)  # => co-21: register the first adapter
    registry.register(BILLING_TONE_RECORD)  # => co-21: register the second adapter
    found = registry.find_for_task("ticket-triage")  # => co-21: discover BY TASK, not by knowing the adapter's exact name in advance
    print(f"Discovered adapter for 'ticket-triage': {found.name if found else None} (owner: {found.owner if found else None})")  # => co-21
    assert found is not None and found.name == "triage-v1", "discovery by task must return the correctly matching adapter record"  # => co-21
    missing = registry.find_for_task("refund-approval")  # => co-21: a task with NO registered adapter
    print(f"Discovered adapter for 'refund-approval': {missing}")  # => co-21
    assert missing is None, "discovery must return None for a task with no registered adapter, not raise or guess"  # => co-21
    assert found.base_model_id == BILLING_TONE_RECORD.base_model_id, "both registered adapters must share the same base pin in this scenario, matching ex-46's shared-base serving shape"  # => co-30
    print("MATCH: the registry resolves a task name to the right adapter's metadata WITHOUT the caller needing to know its exact adapter name in advance")  # => co-21
    # => co-21: this is the discoverability half of co-21's 'composable artefact' claim -- an adapter is only truly composable if it can be FOUND, not just loaded
