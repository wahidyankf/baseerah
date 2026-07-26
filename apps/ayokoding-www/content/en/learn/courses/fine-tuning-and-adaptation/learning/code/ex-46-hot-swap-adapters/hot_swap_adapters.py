# learning/code/ex-46-hot-swap-adapters/hot_swap_adapters.py
"""Worked Example 46: Hot-Swap Adapters."""  # => co-21: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from dataclasses import dataclass, field  # => co-21: two adapters, one base, swapped on demand -- the operational win over full fine-tuning


@dataclass(frozen=True)  # => co-21: frozen -- an adapter's identity is fixed once trained
class Adapter:  # => co-21: a small, composable artefact, attachable to and detachable from a shared base
    name: str  # => co-21: which adapter this is
    style: str  # => co-21: the behaviour this adapter shapes, for this file's illustration


@dataclass  # => co-29: a mock server holding ONE base and MULTIPLE adapters, switching between them per request
class MockServer:  # => co-29: stands in for the real serving stack this course's prerequisite topic covers in depth
    base_model_id: str  # => co-29: the single loaded base model, shared by every attached adapter
    loaded_adapters: dict[str, Adapter] = field(default_factory=dict[str, Adapter])  # => co-21: every adapter currently attached, keyed by name
    active_adapter_name: str | None = None  # => co-21: which adapter is currently routing requests -- switched WITHOUT reloading the base

    def load_adapter(self, adapter: Adapter) -> None:  # => co-21: attach an adapter, cheap because it is small
        """Attach `adapter` to this server's loaded set."""  # => co-21: documents load_adapter's contract -- no runtime output, just sets its __doc__
        self.loaded_adapters[adapter.name] = adapter  # => co-21: attach it to the shared base

    def switch_to(self, adapter_name: str) -> None:  # => co-21: the hot-swap itself -- change which adapter is active, base untouched
        """Set `active_adapter_name` to `adapter_name`, raising if it is not loaded."""  # => co-21: documents switch_to's contract -- no runtime output, just sets its __doc__
        if adapter_name not in self.loaded_adapters:  # => co-21: cannot switch to an adapter that was never loaded
            raise KeyError(f"adapter {adapter_name!r} is not loaded")  # => co-21
        self.active_adapter_name = adapter_name  # => co-21: the swap -- no base reload, no downtime

    def serve(self, prompt: str) -> str:  # => co-29: route a request through the CURRENTLY active adapter
        """Return a mocked response to `prompt`, shaped by whichever adapter is currently active."""  # => co-29: documents serve's contract -- no runtime output, just sets its __doc__
        if self.active_adapter_name is None:  # => co-29: no adapter active means nothing can be served
            raise RuntimeError("no adapter is active")  # => co-29
        active = self.loaded_adapters[self.active_adapter_name]  # => co-21: the currently active adapter's own record
        return f"[{active.name}, style={active.style}] {prompt}"  # => co-29: a mocked response shaped by the active adapter's style


BILLING_TONE_ADAPTER = Adapter(name="billing-tone-v1", style="formal, apologetic")  # => co-21: shapes responses for sensitive billing conversations
ESCALATION_TONE_ADAPTER = Adapter(name="escalation-tone-v1", style="urgent, directive")  # => co-21: shapes responses for P1 escalations


if __name__ == "__main__":  # => co-21: entry point -- runs only when this file executes directly, not on import
    server = MockServer(base_model_id="qwen2.5-0.5b-instruct")  # => co-29: ONE base, loaded once
    server.load_adapter(BILLING_TONE_ADAPTER)  # => co-21: attach the first adapter
    server.load_adapter(ESCALATION_TONE_ADAPTER)  # => co-21: attach the second adapter -- same base, no second base load
    server.switch_to("billing-tone-v1")  # => co-21: activate the billing-tone behaviour
    billing_response = server.serve("Customer was double-charged this month.")  # => co-29: served with billing tone
    print(f"Billing response: {billing_response}")  # => co-29
    server.switch_to("escalation-tone-v1")  # => co-21: hot-swap to the OTHER adapter -- no base reload between requests
    escalation_response = server.serve("Production outage affecting all customers.")  # => co-29: served with escalation tone
    print(f"Escalation response: {escalation_response}")  # => co-29
    assert "billing-tone-v1" in billing_response, "the first request must be served through the billing-tone adapter"  # => co-21
    assert "escalation-tone-v1" in escalation_response, "the second request must be served through the escalation-tone adapter, after the swap"  # => co-21
    assert server.base_model_id == "qwen2.5-0.5b-instruct", "the base model identity must be UNCHANGED across the swap -- only the active adapter moved"  # => co-29
    print("MATCH: two distinct behaviours served from one base model, switched between requests with no base reload -- the swap the tension note calls the operational advantage")  # => co-21,co-29
    # => co-21,co-29: a full fine-tune would need TWO whole loaded models for this -- adapters make it one base plus two small, swappable artefacts
