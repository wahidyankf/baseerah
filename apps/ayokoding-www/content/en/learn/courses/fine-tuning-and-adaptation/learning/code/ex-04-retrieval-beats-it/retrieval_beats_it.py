# learning/code/ex-04-retrieval-beats-it/retrieval_beats_it.py
"""Worked Example 4: Retrieval Beats It."""  # => co-04: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

# => co-04: the SAME storage-limit question from ex-03, solved a completely different way
PRICING_DOCUMENT: dict[str, object] = {  # => co-04: a live, editable source of truth -- NOT baked into any model's weights
    "source": "pricing-page-v7.md",  # => co-04: a real citation the answer can point back to
    "enterprise_storage_limit_gb": 500,  # => co-04: the CURRENT figure -- this document is updated the moment pricing changes
    "last_updated": "2026-07-01",  # => co-04: retrieval can state exactly how fresh its source is
}  # => co-04: closes PRICING_DOCUMENT


def retrieve_and_answer(question: str, document: dict[str, object]) -> tuple[str, str]:  # => co-04: (answer, citation)
    """Look up the current figure in `document` and answer with an explicit citation -- no memorization involved."""  # => co-04: documents retrieve_and_answer's contract -- no runtime output, just sets its __doc__
    del question  # => co-04: unused -- this mock always retrieves the SAME field, a real retriever would route by query
    limit = document["enterprise_storage_limit_gb"]  # => co-04: read straight from the live document, not from memory
    citation = f"{document['source']} (updated {document['last_updated']})"  # => co-04: exactly where this figure came from
    return f"The Enterprise plan's storage limit is {limit} GB.", citation  # => co-04: returns this computed value to the caller


if __name__ == "__main__":  # => co-04: entry point -- runs only when this file executes directly, not on import
    answer, citation = retrieve_and_answer(  # => co-04: ask the SAME question ex-03's fine-tuned model got wrong
        "What is the Enterprise plan's storage limit?",
        PRICING_DOCUMENT,  # => co-04: the current question, against the current document
    )  # => co-04: closes the call
    print(f"Retrieval-based answer: {answer!r}")  # => co-04: prints the current, correct answer
    print(f"Citation: {citation}")  # => co-04: prints exactly where it came from -- something ex-03's fine-tune could never provide
    stated_limit = int(answer.split()[6])  # => co-04: pull the number the answer actually stated, same parsing as ex-03
    assert stated_limit == 500, "retrieval must reflect the CURRENT figure, not a training-time snapshot"  # => co-04

    PRICING_DOCUMENT["enterprise_storage_limit_gb"] = 750  # => co-04: simulate a pricing change -- edit the document, nothing else
    updated_answer, _ = retrieve_and_answer("What is the Enterprise plan's storage limit?", PRICING_DOCUMENT)  # => co-04: ask again
    print(f"After a pricing update, same question: {updated_answer!r}")  # => co-04: the answer updates immediately
    assert "750" in updated_answer, "editing the document must update the answer with ZERO retraining"  # => co-04: the whole point
    print("MATCH: retrieval stayed current, cited its source, and updated with a document edit -- no training run required")  # => co-04
    # => co-02,co-04: this is co-04's rule made concrete -- the SAME knowledge gap ex-03 mishandled is solved here, cheaper and correctly
