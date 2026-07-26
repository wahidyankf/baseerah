"""Worked Example 67: Detect Eval Contamination by Checking for Verbatim Overlap With a Training Corpus."""  # => co-22: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

# A DIFFERENT contamination signal than ex-43's cache-timing check: here, the eval QUESTION TEXT
# itself has leaked verbatim into a corpus the model was fine-tuned on -- a training-data leak, not a runtime cache hit.
TRAINING_CORPUS_SAMPLE = (  # => co-22: a small, illustrative sample of what the fine-tuning corpus actually contained
    "Support macro: when a user says 'move this to done', ask which board before acting.",  # => co-22: this sentence closely mirrors one eval case's exact wording
    "Billing FAQ: refunds process within 5-7 business days.",  # => co-22: unrelated corpus entry
    "Onboarding doc: new users start on the Free plan by default.",  # => co-22: unrelated corpus entry
)  # => co-22: closes TRAINING_CORPUS_SAMPLE

EVAL_CASES = (  # => co-22: the eval suite's own case texts, to be checked for corpus overlap
    "move this to done",  # => co-22: SUSPICIOUS -- near-identical phrasing appears in the training corpus sample above
    "what is the refund timeline for a cancelled order",  # => co-22: clean -- no matching corpus phrase
)  # => co-22: closes EVAL_CASES


def normalize(text: str) -> str:  # => co-22: lowercases and strips punctuation-adjacent quoting, so near-verbatim matches are not missed by casing/quoting alone
    """Return `text` lowercased, with single/double quote characters stripped."""  # => co-22: documents normalize's contract -- no runtime output, just sets its __doc__
    return text.lower().replace("'", "").replace('"', "")  # => co-22: returns this computed value to the caller


def find_contaminated_cases(eval_cases: tuple[str, ...], corpus: tuple[str, ...]) -> tuple[str, ...]:  # => co-22: flags eval cases whose text appears (near-)verbatim inside the training corpus
    """Return the `eval_cases` entries whose normalized text is a substring of any normalized `corpus` entry."""  # => co-22: documents find_contaminated_cases's contract -- no runtime output, just sets its __doc__
    normalized_corpus = tuple(normalize(c) for c in corpus)  # => co-22: normalize once, reused for every eval case
    return tuple(case for case in eval_cases if any(normalize(case) in corpus_entry for corpus_entry in normalized_corpus))  # => co-22: returns this computed value to the caller


if __name__ == "__main__":  # => co-22: entry point -- runs only when this file executes directly, not on import
    contaminated = find_contaminated_cases(EVAL_CASES, TRAINING_CORPUS_SAMPLE)  # => co-22: check every eval case against the corpus sample
    print(f"Eval cases: {EVAL_CASES}")  # => co-22: prints the raw eval case texts
    print(f"Flagged as contaminated (verbatim corpus overlap): {contaminated}")  # => co-22: prints the flagged cases

    assert contaminated == ("move this to done",), "only the eval case whose exact phrasing appears in the training corpus sample must be flagged"  # => co-22: the rule this example proves
    assert "what is the refund timeline for a cancelled order" not in contaminated, "an eval case with no corpus overlap must NOT be flagged"  # => co-22
    print(f"MATCH: {len(contaminated)} of {len(EVAL_CASES)} eval cases show verbatim overlap with the training corpus sample -- a contamination risk distinct from ex-43's runtime cache-timing signal")  # => co-22
    # => co-22: ex-68 next builds a red-team case DELIBERATELY sourced from the taxonomy, rather than screening for accidental corpus overlap
