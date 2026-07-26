# learning/code/ex-11-legitimate-case-register/legit_case_register.py
"""Worked Example 11: Legitimate Case -- Register."""  # => co-07: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

# => co-07: Vantage's brand-voice register is a LEARNED style, not a short rule -- attempts to compress it into a prompt, and their result
REGISTER_PROMPT_ATTEMPTS: dict[str, float] = {  # => co-07: prompt token budget -> measured register-compliance rate, on a held-out eval
    "one-line style rule": 0.32,  # => co-07: "write in Vantage's confident, plain-spoken voice" -- far too underspecified
    "one paragraph of guidance": 0.51,  # => co-07: more detail helps, but still well short of usable
    "five few-shot examples (~600 tokens)": 0.68,  # => co-07: meaningfully better, still not reliable
    "twenty few-shot examples (~2,400 tokens)": 0.79,  # => co-07: diminishing returns are visible now
    "full style guide + fifty examples (~6,000 tokens)": 0.86,  # => co-07: near the practical context-budget ceiling for this prompt slot
}  # => co-07: closes REGISTER_PROMPT_ATTEMPTS -- more budget keeps helping, but the curve is flattening well short of target

TARGET_COMPLIANCE = 0.95  # => co-07: the bar Vantage's brand team set for auto-sent replies

MAX_PRACTICAL_PROMPT_TOKENS = 6000  # => co-07: the largest register-description budget this pipeline can afford per request, on cost/latency grounds


if __name__ == "__main__":  # => co-07: entry point -- runs only when this file executes directly, not on import
    for attempt, rate in REGISTER_PROMPT_ATTEMPTS.items():  # => co-07: prints the whole diminishing-returns curve
        print(f"  {attempt}: {rate:.0%}")  # => co-07: shows how far each budget level gets
    best_rate = max(REGISTER_PROMPT_ATTEMPTS.values())  # => co-07: the best ANY practical prompt budget achieved
    best_attempt = max(REGISTER_PROMPT_ATTEMPTS, key=lambda k: REGISTER_PROMPT_ATTEMPTS[k])  # => co-07: which attempt achieved it
    print(f"Best achievable within {MAX_PRACTICAL_PROMPT_TOKENS} tokens: {best_rate:.0%} ({best_attempt})")  # => co-07
    print(f"Target compliance: {TARGET_COMPLIANCE:.0%}")  # => co-07: the actual bar
    gap_remains = best_rate < TARGET_COMPLIANCE  # => co-07: co-06's "alternatives exhausted" check for THIS case
    assert best_rate > 0.5, "the register attempts must show real, meaningful improvement with more budget"  # => co-07
    assert gap_remains, "even the largest practical prompt budget must fall short of the target compliance"  # => co-07
    print(f"Gap remains after exhausting practical prompt budget: {gap_remains}")  # => co-07
    print("MATCH: a domain register has no compact textual description -- more prompt budget helps, but a ceiling remains -- a legitimate case")  # => co-07
    # => co-07: unlike ex-05's one-sentence disclaimer, a whole VOICE cannot be compressed into any prompt budget this pipeline can afford
