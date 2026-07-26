# learning/code/ex-52-a-knowledge-heavy-request-rejected/knowledge_heavy_rejected.py
"""Worked Example 52: A Knowledge-Heavy Request, Rejected."""  # => co-01: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from dataclasses import dataclass  # => co-06: the decision record this stakeholder request is walked through


@dataclass(frozen=True)  # => co-06: frozen -- a stakeholder request should not mutate once recorded
class StakeholderRequest:  # => co-01: what a real request for a fine-tune looks like BEFORE any triage happens
    requester: str  # => co-01: who asked
    raw_ask: str  # => co-01: their own words, unedited
    contains_facts_that_change: bool  # => co-01: does satisfying this ask require memorizing something that will go stale?


REQUEST = StakeholderRequest(  # => co-01: an actual quarterly request from Vantage's Product team
    requester="Product team lead",  # => co-01
    raw_ask="Fine-tune the assistant on our full feature list and current plan limits so it can answer any product question.",  # => co-01
    contains_facts_that_change=True,  # => co-01: plan limits and the feature list both change every release
)  # => co-01: closes REQUEST


def triage(request: StakeholderRequest) -> tuple[str, str]:  # => co-01: co-01's classification, applied to a raw, unedited ask
    """Classify `request` as behaviour-shaped or knowledge-shaped, with the reasoning."""  # => co-01: documents triage's contract -- no runtime output, just sets its __doc__
    if request.contains_facts_that_change:  # => co-01: the single deciding question co-01 asks
        return "KNOWLEDGE_SHAPED", "the ask requires memorizing facts (feature list, plan limits) that change every release"  # => co-01
    return "BEHAVIOUR_SHAPED", "the ask is about how the assistant acts, not what it knows"  # => co-01


def decide(kind: str) -> tuple[bool, str]:  # => co-06: the same "behaviour-shaped, not knowledge-shaped" check from ex-08/ex-09
    """Return (True, reason) only for a behaviour-shaped classification."""  # => co-06: documents decide's contract -- no runtime output, just sets its __doc__
    if kind == "KNOWLEDGE_SHAPED":  # => co-06: check 3 from ex-08's gate, applied here in isolation
        return False, "NO-GO: knowledge-shaped -- redirect to a retrieval pipeline over the current feature list and plan limits"  # => co-06
    return True, "GO: behaviour-shaped -- proceed to the rest of the decision gate"  # => co-06


if __name__ == "__main__":  # => co-06: entry point -- runs only when this file executes directly, not on import
    kind, kind_reason = triage(REQUEST)  # => co-01: classify the raw request
    print(f"Request from {REQUEST.requester}: {REQUEST.raw_ask!r}")  # => co-01: prints the raw ask, verbatim
    print(f"Classification: {kind} -- {kind_reason}")  # => co-01: prints the classification and its justification
    decision, decision_reason = decide(kind)  # => co-06: run the gate's knowledge-shaped check
    print(f"Decision: {'GO' if decision else 'NO-GO'} -- {decision_reason}")  # => co-06: prints the documented outcome
    assert kind == "KNOWLEDGE_SHAPED", "this request must classify as knowledge-shaped"  # => co-01
    assert decision is False, "a knowledge-shaped request must be rejected before any dataset work begins"  # => co-06
    assert "retrieval" in decision_reason, "the rejection must name the actual redirect, not just say no"  # => co-06
    print("MATCH: an entire well-intentioned stakeholder ask is rejected at triage -- before a single training example is written")  # => co-01,co-06
    # => co-01,co-02,co-06: this is co-02's mistake caught EARLY -- rejecting at the request stage is far cheaper than rejecting after a dataset exists
