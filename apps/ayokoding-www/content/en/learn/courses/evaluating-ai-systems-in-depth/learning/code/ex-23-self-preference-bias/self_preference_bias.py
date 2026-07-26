"""Worked Example 23: Have a Model Judge Its Own Output Versus Another's."""  # => co-12: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic


def model_alpha_style_reply(question: str) -> str:  # => co-12: "Model Alpha"'s own generation style -- terse, direct
    """Model Alpha's own generation style: terse and direct."""  # => co-12: documents model_alpha_style_reply's contract -- no runtime output, just sets its __doc__
    return f"Answer: {question.split()[-1].rstrip('?')} is confirmed."  # => co-12: Model Alpha's own signature phrasing


def model_beta_style_reply(question: str) -> str:  # => co-12: "Model Beta"'s own generation style -- warmer, more hedged
    """Model Beta's own generation style: warmer and slightly hedged."""  # => co-12: documents model_beta_style_reply's contract -- no runtime output, just sets its __doc__
    return f"I believe the answer relates to {question.split()[-1].rstrip('?')}, based on what I can see."  # => co-12: Model Beta's own signature phrasing


def judge_as_model_alpha(candidate: str) -> bool:  # => co-12: Model Alpha, ASKED TO JUDGE, prefers replies that sound like its own style
    """A mocked stand-in for Model Alpha acting as judge -- rewards phrasing that matches its OWN style."""  # => co-12: documents judge_as_model_alpha's contract -- no runtime output, just sets its __doc__
    return "confirmed" in candidate  # => co-13: rewards Alpha's own terse "confirmed" phrasing specifically -- self-preference bias


def judge_as_model_beta(candidate: str) -> bool:  # => co-12: Model Beta, ASKED TO JUDGE, prefers replies that sound like ITS own style
    """A mocked stand-in for Model Beta acting as judge -- rewards phrasing that matches its OWN style."""  # => co-12: documents judge_as_model_beta's contract -- no runtime output, just sets its __doc__
    return "i believe" in candidate.lower()  # => co-13: rewards Beta's own hedged phrasing specifically -- self-preference bias


if __name__ == "__main__":  # => co-12: entry point -- runs only when this file executes directly, not on import
    question = "Which plan supports offline sync?"  # => co-12: the SAME question, answered by both models
    alpha_reply = model_alpha_style_reply(question)  # => co-12: Model Alpha's own generated reply
    beta_reply = model_beta_style_reply(question)  # => co-12: Model Beta's own generated reply
    print(f"Alpha's reply: {alpha_reply!r}")  # => co-12: prints Alpha's own generation
    print(f"Beta's reply: {beta_reply!r}")  # => co-12: prints Beta's own generation

    alpha_judges_own = judge_as_model_alpha(alpha_reply)  # => co-12: Alpha-as-judge scoring ITS OWN reply
    alpha_judges_beta = judge_as_model_alpha(beta_reply)  # => co-12: Alpha-as-judge scoring the OTHER model's reply
    beta_judges_own = judge_as_model_beta(beta_reply)  # => co-12: Beta-as-judge scoring ITS OWN reply
    beta_judges_alpha = judge_as_model_beta(alpha_reply)  # => co-12: Beta-as-judge scoring the OTHER model's reply
    print(f"Alpha-as-judge: own={alpha_judges_own}, other's={alpha_judges_beta}")  # => co-12: prints the self-preference pattern
    print(f"Beta-as-judge: own={beta_judges_own}, other's={beta_judges_alpha}")  # => co-12: prints the self-preference pattern

    assert alpha_judges_own and not alpha_judges_beta, "Alpha-as-judge must favor its OWN generation style over Beta's"  # => co-13: the self-preference effect
    assert beta_judges_own and not beta_judges_alpha, "Beta-as-judge must favor its OWN generation style over Alpha's"  # => co-13: the self-preference effect, mirrored
    print("MATCH: BOTH models, judging, favor the reply that matches their OWN generation style -- self-preference bias, in both directions")  # => co-13
    # => co-12,co-13: a model judging its own output is not neutral -- it is systematically biased toward its own stylistic fingerprint
