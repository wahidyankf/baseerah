# learning/code/ex-45-serve-an-adapter/serve_an_adapter.py
"""Worked Example 45: Serve an Adapter."""  # => co-29: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from dataclasses import dataclass, field  # => co-21: an adapter is a small, composable artefact -- this class models exactly that shape


@dataclass(frozen=True)  # => co-21: frozen -- a served adapter's identity is fixed once loaded, matching the artefact it was built from
class Adapter:  # => co-21: a small, versionable artefact -- NOT a whole new model
    name: str  # => co-21: which adapter this is
    base_model_id: str  # => co-30: which base model this adapter was trained against -- see ex-48 for what happens when this drifts
    weights_mb: float  # => co-21: the adapter's own on-disk size -- small, unlike a full fine-tuned checkpoint


@dataclass  # => co-29: a mock serving stack -- ONE base model in memory, adapters attached and swapped on top of it
class MockServer:  # => co-29: stands in for the real serving stack this course's prerequisite topic covers in depth
    base_model_id: str  # => co-29: the single loaded base model this server holds in memory
    loaded_adapters: dict[str, Adapter] = field(default_factory=dict[str, Adapter])  # => co-21: adapters currently attached, keyed by name -- small enough to hold several at once

    def load_adapter(self, adapter: Adapter) -> None:  # => co-29: attach an adapter to the currently loaded base
        """Attach `adapter` to this server, raising if its `base_model_id` does not match the loaded base."""  # => co-29: documents load_adapter's contract -- no runtime output, just sets its __doc__
        if adapter.base_model_id != self.base_model_id:  # => co-30: an adapter trained against one base cannot simply be attached to another
            raise ValueError(f"adapter {adapter.name!r} was trained against {adapter.base_model_id!r}, not the loaded base {self.base_model_id!r}")  # => co-30
        self.loaded_adapters[adapter.name] = adapter  # => co-21: attach it -- cheap, because the adapter is small

    def serve(self, adapter_name: str, prompt: str) -> str:  # => co-29: route a request through the base PLUS the named adapter
        """Return a mocked response to `prompt`, shaped by the adapter named `adapter_name`."""  # => co-29: documents serve's contract -- no runtime output, just sets its __doc__
        if adapter_name not in self.loaded_adapters:  # => co-29: the adapter must be loaded before it can serve traffic
            raise KeyError(f"adapter {adapter_name!r} is not loaded on this server")  # => co-29
        return f"[{adapter_name}] {prompt} -> Priority: P2. Category: access."  # => co-29: a mocked, adapter-shaped response, standing in for a real generation call


TRIAGE_ADAPTER = Adapter(name="triage-v1", base_model_id="qwen2.5-0.5b-instruct", weights_mb=2.3)  # => co-21: matches ex-29's own trained adapter, roughly


if __name__ == "__main__":  # => co-29: entry point -- runs only when this file executes directly, not on import
    server = MockServer(base_model_id="qwen2.5-0.5b-instruct")  # => co-29: ONE base model loaded, matching TRIAGE_ADAPTER's own base
    server.load_adapter(TRIAGE_ADAPTER)  # => co-29: attach the adapter -- cheap, since it is only 2.3MB
    print(f"Loaded adapters: {list(server.loaded_adapters)}")  # => co-29
    response = server.serve("triage-v1", "Customer cannot log in after a password reset.")  # => co-29: serve one request through the adapted behaviour
    print(f"Response: {response}")  # => co-29
    assert "triage-v1" in server.loaded_adapters, "the adapter must be present in the server's loaded set after load_adapter"  # => co-29
    assert response.startswith("[triage-v1]"), "the served response must be routed through the named adapter, not the bare base"  # => co-29
    mismatched_adapter = Adapter(name="wrong-base-adapter", base_model_id="a-different-base", weights_mb=2.1)  # => co-30: an adapter trained against a DIFFERENT base
    try:  # => co-30: attempting to attach a mismatched adapter must be rejected, not silently allowed
        server.load_adapter(mismatched_adapter)  # => co-30
        raised = False  # => co-30
    except ValueError:  # => co-30: this is the expected, correct outcome
        raised = True  # => co-30
    assert raised, "loading an adapter trained against a different base must raise, never silently attach"  # => co-30
    print("MATCH: the adapter loads onto its matching base and serves adapted behaviour; a mismatched-base adapter is rejected at load time, not at inference time")  # => co-29,co-30
    # => co-29,co-30: this is the operational shape co-29 describes -- the base stays loaded once, and adapters attach to and detach from it cheaply
