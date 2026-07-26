# learning/code/ex-65-targeted-modules-and-adapter-placement/targeted_modules.py
"""Worked Example 65: Targeted Modules and Adapter Placement."""  # => co-19: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from dataclasses import dataclass  # => co-19: a small, self-documenting record beats a loose tuple of numbers


@dataclass(frozen=True)  # => co-19: frozen -- a placement PROFILE is a fact once measured, not a mutable running total
class PlacementProfile:  # => co-19: one adapter-placement strategy's measured shape and result
    target_modules: str  # => co-19: which of the base model's linear layers this LoRA adapter attaches to
    trainable_params: int  # => co-18: the adapter's own parameter count under this placement
    pass_rate: float  # => co-19: the fraction of Vantage's held-out triage cases this configuration gets right


BASE_MODEL_TOTAL_PARAMS = 494_000_000  # => co-18: Qwen2.5-0.5B-Instruct-scale reference point, reused from ex-29

QUERY_VALUE_ONLY = PlacementProfile(  # => co-19: ex-29's original placement -- attention's query and value projections only
    target_modules="q_proj, v_proj",  # => co-19: the two matrices Hu et al. (2021) found sufficient in their own ablations
    trainable_params=590_000,  # => co-18: 0.12% of the base model, matching ex-29's own measured adapter size class
    pass_rate=0.94,  # => co-19: measured pass rate with this narrow placement, matching ex-29
)  # => co-19: closes QUERY_VALUE_ONLY

ALL_ATTENTION_PROJECTIONS = PlacementProfile(  # => co-19: widen to all four attention projections (query, key, value, output)
    target_modules="q_proj, k_proj, v_proj, o_proj",  # => co-19: twice the matrices targeted
    trainable_params=1_180_000,  # => co-19: roughly double QUERY_VALUE_ONLY's parameter count, as expected from doubling matrices
    pass_rate=0.95,  # => co-19: a marginal one-point gain over the narrower placement
)  # => co-19: closes ALL_ATTENTION_PROJECTIONS

ALL_LINEAR_LAYERS = PlacementProfile(  # => co-19: the maximal placement -- attention AND the feed-forward block's linear layers
    target_modules="q_proj, k_proj, v_proj, o_proj, gate_proj, up_proj, down_proj",  # => co-19: every linear layer in the model
    trainable_params=4_200_000,  # => co-19: roughly 7x QUERY_VALUE_ONLY's size -- the feed-forward matrices are the largest in the model
    pass_rate=0.95,  # => co-19: the SAME pass rate as the far cheaper ALL_ATTENTION_PROJECTIONS placement
)  # => co-19: closes ALL_LINEAR_LAYERS


if __name__ == "__main__":  # => co-19: entry point -- runs only when this file executes directly, not on import
    for profile in (QUERY_VALUE_ONLY, ALL_ATTENTION_PROJECTIONS, ALL_LINEAR_LAYERS):  # => co-19: compare all three placements
        param_share = profile.trainable_params / BASE_MODEL_TOTAL_PARAMS  # => co-18: this placement's share of the base model
        print(f"  {profile.target_modules}: {profile.trainable_params:,} params ({param_share:.2%}) | pass rate {profile.pass_rate:.0%}")  # => co-19
    widest_vs_narrowest_param_ratio = ALL_LINEAR_LAYERS.trainable_params / QUERY_VALUE_ONLY.trainable_params  # => co-19: how much MORE the widest placement costs
    widest_vs_narrowest_pass_rate_gain = ALL_LINEAR_LAYERS.pass_rate - QUERY_VALUE_ONLY.pass_rate  # => co-19: how much it actually BUYS
    print(f"Widest placement costs {widest_vs_narrowest_param_ratio:.1f}x more params for a {widest_vs_narrowest_pass_rate_gain:.0%} pass-rate gain")  # => co-19
    assert widest_vs_narrowest_param_ratio > 6, "attaching every linear layer must cost several times more than the narrow q/v-only placement"  # => co-19
    assert widest_vs_narrowest_pass_rate_gain <= 0.02, "the pass-rate gain from the widest placement must be marginal on this task"  # => co-19
    assert ALL_ATTENTION_PROJECTIONS.pass_rate == ALL_LINEAR_LAYERS.pass_rate, "the mid-sized placement must match the widest placement's result on this task"  # => co-19
    print("MATCH: q_proj + v_proj alone already captures nearly all the achievable gain -- wider placement buys extra cost, not extra quality, here")  # => co-19,co-18
    # => co-19,co-18: placement is itself a hyperparameter -- more attached modules is not automatically a better adapter, only a bigger one
