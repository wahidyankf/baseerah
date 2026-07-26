"""Worked Example 40: Attribute a Failure to a Subagent vs. the Orchestrating Agent."""  # => co-20: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import NamedTuple  # => co-20: OrchestratedStep tags each step with WHICH agent performed it


class OrchestratedStep(NamedTuple):  # => co-20: one step, tagged with the agent that actually ran it
    performed_by: str  # => co-20: "orchestrator" or the name of the subagent that ran this step
    tool_name: str  # => co-20: which tool was invoked
    step_is_correct: bool  # => co-20: ground truth -- was this step itself correct?


# The orchestrator delegates ticket triage to a "triage-subagent". The failure below happened
# INSIDE that subagent's own delegated work, not in the orchestrator's own routing decision.
DELEGATED_TRAJECTORY = (  # => co-20: co-18's trajectory concept, extended with a "performed_by" agent tag per step
    OrchestratedStep("orchestrator", "delegate_to_triage_subagent", step_is_correct=True),  # => co-20: orchestrator's OWN decision to delegate -- correct
    OrchestratedStep("triage-subagent", "search_ticket", step_is_correct=True),  # => co-20: subagent's step -- correct
    OrchestratedStep("triage-subagent", "close_ticket", step_is_correct=False),  # => co-20: subagent's step -- WRONG: closed an open, unresolved ticket
    OrchestratedStep("orchestrator", "report_result_to_user", step_is_correct=True),  # => co-20: orchestrator's OWN step -- correctly reported what the subagent did, even though that action was wrong
)  # => co-20: closes DELEGATED_TRAJECTORY


def attribute_failure_by_agent(trajectory: tuple[OrchestratedStep, ...]) -> str | None:  # => co-20: attributes failure to WHICH agent (orchestrator or a named subagent), not just which step
    """Return the `performed_by` value of the FIRST incorrect step, or None if every step was correct."""  # => co-20: documents attribute_failure_by_agent's contract -- no runtime output, just sets its __doc__
    for step in trajectory:  # => co-20: scans steps in order -- the FIRST incorrect step names the responsible agent
        if not step.step_is_correct:  # => co-20: found the causing step
            return step.performed_by  # => co-20: returns this computed value to the caller -- WHO is responsible, not just where
    return None  # => co-20: no failing step found


if __name__ == "__main__":  # => co-20: entry point -- runs only when this file executes directly, not on import
    responsible_agent = attribute_failure_by_agent(DELEGATED_TRAJECTORY)  # => co-20: attribute the failure to a specific agent, not the system as a whole
    orchestrator_steps_all_correct = all(s.step_is_correct for s in DELEGATED_TRAJECTORY if s.performed_by == "orchestrator")  # => co-20: check the orchestrator's OWN steps in isolation
    print(f"Responsible agent: {responsible_agent}")  # => co-20: prints who is responsible
    print(f"Orchestrator's own steps were all correct: {orchestrator_steps_all_correct}")  # => co-20: prints the orchestrator's own record

    assert responsible_agent == "triage-subagent", "the failure must be attributed to the triage-subagent, whose own step was wrong -- not the orchestrator"  # => co-20: the rule this example proves
    assert orchestrator_steps_all_correct is True, "the orchestrator's OWN routing and reporting steps were both correct -- delegating was not itself a mistake"  # => co-20
    print(f"MATCH: the failure is attributed to '{responsible_agent}', while the orchestrator's own delegation and reporting steps are cleared -- blame lands on the agent that actually erred, not the boundary between them")  # => co-20
    # => co-20: this per-agent attribution is what lets ex-49 later route a fix back to the RIGHT owner during error analysis, not to the whole pipeline
