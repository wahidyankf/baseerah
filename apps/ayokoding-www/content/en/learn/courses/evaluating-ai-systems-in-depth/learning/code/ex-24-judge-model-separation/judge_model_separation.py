"""Worked Example 24: Swap In a Different Judge Model to Remove the Correlated Blind Spot."""  # => co-12: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic


def model_alpha_generate(question: str) -> str:  # => co-12: the GENERATING model under test
    """Model Alpha's generation -- confidently states a plausible but WRONG plan name."""  # => co-12: documents model_alpha_generate's contract -- no runtime output, just sets its __doc__
    del question  # => co-12: this mock always produces the same deliberately-wrong answer, regardless of the question text
    return "Offline sync is available on the Free plan."  # => co-12: WRONG -- offline sync is actually Pro-only, a shared blind spot Alpha holds


def judge_same_as_generator(reply: str) -> bool:  # => co-12: Model Alpha ALSO acting as its own judge -- shares the generator's own blind spot
    """A mocked stand-in for Model Alpha judging its own output -- shares its own factual blind spot."""  # => co-12: documents judge_same_as_generator's contract -- no runtime output, just sets its __doc__
    return "offline sync is available on the" in reply.lower()  # => co-12: checks only STRUCTURE, not the actual plan name -- the SAME blind spot as the generator


def judge_different_model(reply: str, *, known_correct_plan: str = "Pro") -> bool:  # => co-12: a DIFFERENT model, with no shared training blind spot on this fact
    """A mocked stand-in for a different judge model -- independently checks the ACTUAL correct plan name."""  # => co-12: documents judge_different_model's contract -- no runtime output, just sets its __doc__
    return known_correct_plan in reply  # => co-12: this judge's OWN knowledge of the correct fact catches what the shared blind spot missed


if __name__ == "__main__":  # => co-12: entry point -- runs only when this file executes directly, not on import
    reply = model_alpha_generate("Which plan supports offline sync?")  # => co-12: Alpha's own, factually WRONG reply
    print(f"Generated reply: {reply!r}")  # => co-12: prints the wrong answer

    same_model_verdict = judge_same_as_generator(reply)  # => co-12: judged by the SAME model that generated it
    different_model_verdict = judge_different_model(reply)  # => co-12: judged by a genuinely DIFFERENT model
    print(f"Judged by the SAME model as generator: passed={same_model_verdict}")  # => co-12: prints the shared-blind-spot verdict
    print(f"Judged by a DIFFERENT model: passed={different_model_verdict}")  # => co-12: prints the independent verdict

    assert same_model_verdict is True, "the same-model judge must wrongly pass its own factually incorrect reply"  # => co-12: the blind spot, made concrete
    assert different_model_verdict is False, "a genuinely different judge model must catch the factual error"  # => co-12: the rule this example proves
    assert same_model_verdict != different_model_verdict, "the two judges must disagree on the identical reply"  # => co-12
    print("MATCH: swapping the judge model changed the verdict on the IDENTICAL reply -- the correlated blind spot, not fairness, is why judge-model separation matters")  # => co-12
    # => co-12: this is exactly why co-09's judge should never be the same model that generated the output under test
