"""Worked Example 68: Design a Red-Team Probe Deliberately FROM a Known Taxonomy Mode."""  # => co-21: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import NamedTuple  # => co-21: RedTeamCase is a typed record explicitly tracing back to its taxonomy origin


class RedTeamCase(NamedTuple):  # => co-21: a red-team probe, built with an explicit link back to co-03's taxonomy
    probe_text: str  # => co-21: the adversarial input itself
    targets_taxonomy_mode: str  # => co-03: WHICH known failure mode this probe was deliberately designed to exercise HARDER
    exaggeration_strategy: str  # => co-21: how this probe pushes the known mode further than ordinary production traffic does


KNOWN_MODE = "skips-clarifying-question"  # => co-03: the SAME taxonomy mode ex-01/ex-04/ex-08 already established from real error analysis

# Rather than waiting for production traffic to exhibit an EXTREME version of this mode, this
# case is deliberately engineered to stress it -- multiple stacked ambiguities in one request.
RED_TEAM_CASE = RedTeamCase(  # => co-21: a taxonomy-derived probe, not a generic "try to break it" prompt
    probe_text="Move that one to the other board and also close the other ticket, you know which ones.",  # => co-21: THREE separate ambiguous references stacked in one request
    targets_taxonomy_mode=KNOWN_MODE,  # => co-03: explicitly traces back to the known mode
    exaggeration_strategy="stacks three simultaneous ambiguous references, instead of the single ambiguity ordinary production cases exhibit",  # => co-21
)  # => co-21: closes RED_TEAM_CASE


def mock_agent_reply(probe_text: str) -> str:  # => co-21: a mocked agent under test against the stress-tested probe
    """Return a mocked agent reply to `probe_text`, correctly asking for clarification given the stacked ambiguity."""  # => co-21: documents mock_agent_reply's contract -- no runtime output, just sets its __doc__
    del probe_text  # => co-21: unused in this mock -- illustrates the correct, resistant reply
    return "That request has a few unclear parts -- which board, and which ticket, exactly?"  # => co-21: correctly asks for clarification on ALL ambiguous parts, not just one


def reply_asks_for_clarification(reply: str) -> bool:  # => co-21: checks the SAME derived criterion (co-08) the ordinary-severity case would use
    """Pass iff `reply` asks a clarifying question rather than guessing."""  # => co-21: documents reply_asks_for_clarification's contract -- no runtime output, just sets its __doc__
    return "which" in reply.lower() or "unclear" in reply.lower()  # => co-21: returns this computed value to the caller


if __name__ == "__main__":  # => co-21: entry point -- runs only when this file executes directly, not on import
    reply = mock_agent_reply(RED_TEAM_CASE.probe_text)  # => co-21: run the agent against the stress-tested probe
    passed = reply_asks_for_clarification(reply)  # => co-21: score it with the SAME derived criterion as an ordinary case
    print(f"Red-team probe (targets {RED_TEAM_CASE.targets_taxonomy_mode!r}): {RED_TEAM_CASE.probe_text!r}")  # => co-21: prints the probe and its taxonomy origin
    print(f"Exaggeration strategy: {RED_TEAM_CASE.exaggeration_strategy}")  # => co-21: prints how it was deliberately stressed
    print(f"Agent reply: {reply!r}")  # => co-21: prints the agent's reply
    print(f"Passed: {passed}")  # => co-21: prints the verdict

    assert RED_TEAM_CASE.targets_taxonomy_mode == KNOWN_MODE, "the red-team case must explicitly trace back to a known, established taxonomy mode"  # => co-03: the rule this example proves
    assert passed is True, "the agent must survive the exaggerated, taxonomy-derived stress case, not just the ordinary-severity version"  # => co-21: the rule this example proves
    print(f"MATCH: a red-team probe deliberately engineered FROM the '{RED_TEAM_CASE.targets_taxonomy_mode}' taxonomy mode stresses it harder than ordinary traffic, and the agent still passes")  # => co-21
    # => co-21: ex-69 next asks how this exact dataset -- red-team cases included -- stays REPRODUCIBLE as it evolves over time
