# learning/code/ex-47-adapter-memory-and-routing/adapter_memory_and_routing.py
"""Worked Example 47: Adapter Memory and Routing."""  # => co-29: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from dataclasses import dataclass  # => co-21: a small, self-documenting record for the serving-side memory math


@dataclass(frozen=True)  # => co-21: frozen -- these are measured, fixed facts about a deployment shape, not a running total
class ServingProfile:  # => co-29: how much memory it costs to serve N variants of a task, two different ways
    strategy: str  # => co-29: "N full fine-tuned models" or "1 base + N adapters"
    base_models_loaded: int  # => co-29: how many full base-model copies must be resident in memory
    adapters_loaded: int  # => co-21: how many small adapters must be resident in memory
    base_model_size_mb: float  # => co-29: one base model's own memory footprint
    adapter_size_mb: float  # => co-21: one adapter's own memory footprint -- small, by construction


def total_memory_mb(profile: ServingProfile) -> float:  # => co-29: the serving-side memory cost of this strategy
    """Return the total memory in MB required by `profile`'s loaded base models and adapters."""  # => co-29: documents total_memory_mb's contract -- no runtime output, just sets its __doc__
    return (profile.base_models_loaded * profile.base_model_size_mb) + (profile.adapters_loaded * profile.adapter_size_mb)  # => co-29: returns this computed value to the caller


FIVE_FULL_MODELS = ServingProfile(  # => co-29: serving 5 task variants by loading 5 SEPARATE full fine-tuned models
    strategy="5 full fine-tuned models",  # => co-17: each one a complete, independently fine-tuned copy of the base
    base_models_loaded=5,  # => co-29: every variant needs its own full base-model copy in memory
    adapters_loaded=0,  # => co-29
    base_model_size_mb=980.0,  # => co-29: matches ex-28's own measured checkpoint size, roughly
    adapter_size_mb=0.0,  # => co-29
)  # => co-29: closes FIVE_FULL_MODELS

ONE_BASE_FIVE_ADAPTERS = ServingProfile(  # => co-21: serving the SAME 5 task variants with one shared base plus 5 small adapters
    strategy="1 base + 5 adapters",  # => co-21: exactly the operational shape ex-45/ex-46 demonstrated
    base_models_loaded=1,  # => co-29: the base is loaded ONCE, shared by every adapter
    adapters_loaded=5,  # => co-21: each variant is a small adapter attached to the SAME loaded base
    base_model_size_mb=980.0,  # => co-29: the same base-model size as above, loaded a single time
    adapter_size_mb=2.3,  # => co-21: matches ex-45's own TRIAGE_ADAPTER size
)  # => co-21: closes ONE_BASE_FIVE_ADAPTERS


if __name__ == "__main__":  # => co-29: entry point -- runs only when this file executes directly, not on import
    for profile in (FIVE_FULL_MODELS, ONE_BASE_FIVE_ADAPTERS):  # => co-29: compare both serving strategies for the SAME 5 task variants
        memory = total_memory_mb(profile)  # => co-29: this strategy's total memory cost
        print(f"  {profile.strategy}: {memory:,.1f} MB ({profile.base_models_loaded} base(s) + {profile.adapters_loaded} adapter(s))")  # => co-29
    full_models_memory = total_memory_mb(FIVE_FULL_MODELS)  # => co-29: the five-full-models total
    adapters_memory = total_memory_mb(ONE_BASE_FIVE_ADAPTERS)  # => co-21: the one-base-plus-adapters total
    memory_reduction = 1 - (adapters_memory / full_models_memory)  # => co-21: how much memory the adapter strategy saves
    print(f"Adapter strategy uses {memory_reduction:.0%} less memory to serve the same 5 task variants")  # => co-21,co-29
    assert full_models_memory == 4_900.0, "five full fine-tuned models must cost exactly 4,900 MB in this scenario"  # => co-29
    assert adapters_memory == 991.5, "one base plus five adapters must cost exactly 991.5 MB in this scenario"  # => co-21
    assert memory_reduction > 0.75, "the adapter strategy must use dramatically less memory to serve the same variant count"  # => co-21,co-29
    print("MATCH: serving 5 task variants costs 4,900 MB as separate full models but under 1,000 MB as one shared base plus 5 small adapters")  # => co-21,co-29
    # => co-21,co-29: this is the concrete number behind co-21's operational-advantage claim -- adapters make MANY variants affordable to keep resident at once
