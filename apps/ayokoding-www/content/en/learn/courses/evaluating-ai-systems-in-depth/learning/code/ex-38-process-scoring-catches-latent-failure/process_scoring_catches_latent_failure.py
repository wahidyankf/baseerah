"""Worked Example 38: The Same Wrong Path Fails for Real on a Neighbouring Input."""  # => co-19: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from typing import NamedTuple  # => co-19: Trajectory is a typed record pairing steps and a final answer


class Trajectory(NamedTuple):  # => co-18: the same shape as ex-35/ex-37
    tool_sequence: tuple[str, ...]  # => co-18: which tools were called, in order
    final_answer: str  # => co-18: what the agent ultimately told the user


# ex-37's ticket #4821 case happened to still be OPEN and medium priority when the agent
# skipped verification and guessed. This is a DIFFERENT, neighbouring request -- same skipped
# path, but ticket #77 was ALREADY CLOSED, so blindly raising its priority is a real error.
NEIGHBOURING_TRAJECTORY = Trajectory(  # => co-19: the identical wrong PATH as ex-37, applied to a different real case
    tool_sequence=("search_ticket", "update_priority"),  # => co-19: the SAME skipped-verification path as ex-37
    final_answer="I found ticket #77 and raised its priority to high.",  # => co-19: WRONG this time -- ticket #77 is already closed
)  # => co-19: closes NEIGHBOURING_TRAJECTORY

TICKET_77_TRUE_STATE = {"status": "closed", "priority": "low"}  # => co-19: what get_ticket WOULD have revealed, had the agent called it


def process_score(trajectory: Trajectory, *, reference: tuple[str, ...] = ("search_ticket", "get_ticket", "update_priority")) -> bool:  # => co-19: the SAME process scorer as ex-37
    """Pass iff `trajectory.tool_sequence` matches the sanctioned reference path exactly."""  # => co-19: documents process_score's contract -- no runtime output, just sets its __doc__
    return trajectory.tool_sequence == reference  # => co-19


def would_have_been_caught_by_verification(true_state: dict[str, str]) -> bool:  # => co-19: what get_ticket's SKIPPED step would have revealed
    """Return True iff the ticket's real state makes raising priority an outright error (already closed)."""  # => co-19: documents would_have_been_caught_by_verification's contract -- no runtime output, just sets its __doc__
    return true_state["status"] == "closed"  # => co-19: a closed ticket should never have its priority raised at all


if __name__ == "__main__":  # => co-19: entry point -- runs only when this file executes directly, not on import
    process_verdict = process_score(NEIGHBOURING_TRAJECTORY)  # => co-19: the SAME wrong path, scored the SAME way as ex-37
    real_error = would_have_been_caught_by_verification(TICKET_77_TRUE_STATE)  # => co-19: was skipping verification a REAL mistake this time?
    print(f"Process score on the neighbouring case (same skipped-verification path): {process_verdict}")  # => co-19: prints the process verdict
    print(f"Ticket #77's true state: {TICKET_77_TRUE_STATE} -- skipping verification was a real error: {real_error}")  # => co-19

    assert process_verdict is False, "the identical skipped-verification path must fail process scoring, just as it did in ex-37"  # => co-19: consistent process verdict
    assert real_error is True, "on THIS input, the skipped verification step was a genuine, real mistake -- not just a technicality"  # => co-19: the rule this example proves
    print("MATCH: the SAME process-scoring failure from ex-37 is proven real here -- the skipped verification step causes an actual wrong action on ticket #77")  # => co-19
    # => co-19: this is exactly why co-19's process scoring exists -- ex-37's lucky pass hid a latent failure that this neighbouring input exposes for real
