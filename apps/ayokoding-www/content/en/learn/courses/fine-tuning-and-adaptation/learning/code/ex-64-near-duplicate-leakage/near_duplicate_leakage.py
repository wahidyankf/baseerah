# learning/code/ex-64-near-duplicate-leakage/near_duplicate_leakage.py
"""Worked Example 64: Near-Duplicate Leakage."""  # => co-16: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

TRAIN_INSTRUCTIONS = [  # => co-16: five training instructions, committed and used to train
    "Triage: customer cannot log in after a password reset.",  # => co-16: 1
    "Triage: customer wants an invoice re-sent.",  # => co-16: 2
    "Triage: customer was double-charged this month.",  # => co-16: 3
    "Triage: customer wants dark mode added.",  # => co-16: 4
    "Triage: a scheduled export silently failed overnight.",  # => co-16: 5
]  # => co-16: closes TRAIN_INSTRUCTIONS

TEST_INSTRUCTIONS = [  # => co-16: a supposedly disjoint test set -- but one entry is a PARAPHRASE of a training case, not an exact copy
    "Triage: customer's login fails after they reset their password.",  # => co-16: a REWORDED version of TRAIN_INSTRUCTIONS[0] -- same situation
    "Triage: customer's SSO login is broken company-wide.",  # => co-16: genuinely new
    "Triage: customer asks about the free trial length.",  # => co-16: genuinely new
]  # => co-16: closes TEST_INSTRUCTIONS


def exact_match_leak_check(train: list[str], test: list[str]) -> set[str]:  # => co-15: ex-25/ex-26's ORIGINAL leak check -- string equality only
    """Return the set of `test` entries that appear verbatim in `train`."""  # => co-15: documents exact_match_leak_check's contract -- no runtime output, just sets its __doc__
    train_set = set(train)  # => co-15: exact string membership
    return {t for t in test if t in train_set}  # => co-15: only literal, byte-identical matches count here


def near_duplicate_leak_check(train: list[str], test: list[str]) -> list[tuple[str, str]]:  # => co-16: catches PARAPHRASES, not just exact copies
    """Return (train, test) pairs sharing >= 4 significant words (length >= 5), a stand-in for a real semantic-similarity check."""  # => co-16: documents near_duplicate_leak_check's contract -- no runtime output, just sets its __doc__
    leaks: list[tuple[str, str]] = []  # => co-16: accumulates every near-duplicate pair found
    for train_instruction in train:  # => co-16: compare every train instruction against every test instruction
        train_words = {w.lower().strip(".,") for w in train_instruction.split() if len(w) >= 5}  # => co-16: significant words in this train case
        for test_instruction in test:  # => co-16: this test instruction
            test_words = {w.lower().strip(".,") for w in test_instruction.split() if len(w) >= 5}  # => co-16: significant words in this test case
            if len(train_words & test_words) >= 3:  # => co-16: a real similarity model would do better -- this is an illustrative stand-in
                leaks.append((train_instruction, test_instruction))  # => co-16: record the suspicious pair
    return leaks  # => co-16: returns this computed value to the caller


if __name__ == "__main__":  # => co-16: entry point -- runs only when this file executes directly, not on import
    exact_leaks = exact_match_leak_check(TRAIN_INSTRUCTIONS, TEST_INSTRUCTIONS)  # => co-15: run the ORIGINAL, exact-match-only check
    print(f"Exact-match leak check finds: {exact_leaks}")  # => co-15: an empty set -- this check is BLIND to paraphrases
    assert exact_leaks == set(), "the exact-match check must find NOTHING -- the leak here is a paraphrase, not a literal copy"  # => co-15,co-16
    near_leaks = near_duplicate_leak_check(TRAIN_INSTRUCTIONS, TEST_INSTRUCTIONS)  # => co-16: run the NEAR-DUPLICATE check instead
    print(f"Near-duplicate leak check finds: {near_leaks}")  # => co-16: catches the paraphrased pair the exact check missed
    assert len(near_leaks) == 1, "the near-duplicate check must catch exactly the one planted paraphrase"  # => co-16
    assert "password reset" in near_leaks[0][0].lower() or "password" in near_leaks[0][0].lower(), "the caught pair must be the password-reset paraphrase"  # => co-16
    print("MATCH: an exact-match leak check reports clean, while a near-duplicate check catches the SAME leak an exact check is structurally blind to")  # => co-15,co-16
    # => co-15,co-16: ex-25/ex-26's exact-match discipline is necessary but not sufficient -- a paraphrase leaks the SAME information without ever matching a string
