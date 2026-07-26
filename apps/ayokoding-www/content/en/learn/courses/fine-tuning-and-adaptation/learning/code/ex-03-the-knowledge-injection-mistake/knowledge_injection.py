# learning/code/ex-03-the-knowledge-injection-mistake/knowledge_injection.py
"""Worked Example 3: The Knowledge-Injection Mistake."""  # => co-02: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic


def mock_fine_tuned_on_facts(question: str, *, training_cutoff_price: int) -> str:  # => co-02: a model FINE-TUNED to memorize a price table
    """Return a canned, memorized answer baked in at fine-tuning time -- it cannot know anything after `training_cutoff_price`."""  # => co-02: documents mock_fine_tuned_on_facts's contract -- no runtime output, just sets its __doc__
    del question  # => co-02: unused -- this mock always recites the SAME memorized figure, regardless of phrasing
    return f"The Enterprise plan's storage limit is {training_cutoff_price} GB."  # => co-02: confident, fluent, and frozen at training time


CURRENT_STORAGE_LIMIT_GB = 500  # => co-01: the REAL, current limit -- changed after this mock model's fine-tuning run
TRAINING_TIME_STORAGE_LIMIT_GB = 250  # => co-02: what the fine-tune memorized -- correct THEN, stale NOW

if __name__ == "__main__":  # => co-02: entry point -- runs only when this file executes directly, not on import
    answer = mock_fine_tuned_on_facts(  # => co-02: ask the fine-tuned model the question a customer actually asked
        "What is the Enterprise plan's storage limit?",  # => co-02: today's real question
        training_cutoff_price=TRAINING_TIME_STORAGE_LIMIT_GB,  # => co-02: it can only answer with what it memorized
    )  # => co-02: closes the call
    print(f"Fine-tuned model answers: {answer!r}")  # => co-02: prints the confident, stale answer
    stated_limit = int(answer.split()[6])  # => co-02: pull the number the model actually stated, word-index 6 in this fixed sentence
    is_stale = stated_limit != CURRENT_STORAGE_LIMIT_GB  # => co-02: does the memorized figure match reality TODAY?
    print(f"Stated: {stated_limit} GB | Actually current: {CURRENT_STORAGE_LIMIT_GB} GB | Stale: {is_stale}")  # => co-02
    assert is_stale, "a fact fine-tuned into weights must go stale the moment the real fact changes"  # => co-02: the failure mode
    can_cite_a_source = False  # => co-01: a fine-tuned fact has no citation -- it is baked into weights, not retrieved from a document
    print(f"Can the model point to a source document for this figure: {can_cite_a_source}")  # => co-01
    assert not can_cite_a_source, "a memorized fact cannot be traced back to the document that stated it"  # => co-01
    print("MATCH: the fine-tune produced a confident, fluent, and WRONG answer -- and no way to trace or refresh it")  # => co-02
    # => co-01,co-02: this is the mistake this course names first -- fine-tuning is the WRONG tool for facts that change
