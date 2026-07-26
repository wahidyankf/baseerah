# learning/code/ex-54-the-decision-procedure-as-code/decision_as_code.py
"""Worked Example 54: The Decision Procedure as Code, Table-Tested."""  # => co-06: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

from dataclasses import dataclass  # => co-06: a table of typed cases beats four one-off scripts (ex-08, ex-09, and their siblings)


@dataclass(frozen=True)  # => co-06: frozen -- one case in a table should not mutate while the table is iterated
class GateCase:  # => co-06: one row in the table -- inputs, plus the EXPECTED verdict, so this file is its own test
    name: str  # => co-06: a human-readable label for this row
    all_checks_pass: bool  # => co-06: a simplified single flag standing in for ex-08's five ordered booleans, for table brevity
    expected_go: bool  # => co-06: what the gate SHOULD decide for this row -- makes the table self-verifying


def decide(all_checks_pass: bool) -> bool:  # => co-06: the gate, reduced to its essential shape for this table-driven check
    """Return True (GO) iff every ordered gate check passed."""  # => co-06: documents decide's contract -- no runtime output, just sets its __doc__
    return all_checks_pass  # => co-06: a one-line body -- the real logic lives in ex-08's fuller DecisionGateInputs version


CASES = [  # => co-06: four labelled rows, each with its OWN expected outcome, run through the identical decide() function
    GateCase(name="vocabulary case (ex-08)", all_checks_pass=True, expected_go=True),  # => co-06: a known GO
    GateCase(name="pricing case (ex-09)", all_checks_pass=False, expected_go=False),  # => co-06: a known NO-GO
    GateCase(name="tool-use case (ex-51)", all_checks_pass=True, expected_go=True),  # => co-06: a known GO
    GateCase(name="knowledge-heavy request (ex-52)", all_checks_pass=False, expected_go=False),  # => co-06: a known NO-GO
]  # => co-06: closes CASES -- two GOs and two NO-GOs, deliberately mixed


if __name__ == "__main__":  # => co-06: entry point -- runs only when this file executes directly, not on import
    for case in CASES:  # => co-06: run the SAME gate function across every labelled case
        actual_go = decide(case.all_checks_pass)  # => co-06: the gate's real verdict for this row
        status = "OK" if actual_go == case.expected_go else "MISMATCH"  # => co-06: does the gate's verdict match what this row expects?
        print(f"  {case.name}: gate says {'GO' if actual_go else 'NO-GO'} (expected {'GO' if case.expected_go else 'NO-GO'}) -- {status}")  # => co-06
        assert actual_go == case.expected_go, f"{case.name} must match its own expected outcome"  # => co-06: the whole point of tabling it
    print(f"MATCH: all {len(CASES)} table rows agree with their expected outcome -- the gate behaves consistently as a function")  # => co-06
    # => co-06: writing the gate as a pure function over a table of cases is what lets it be regression-tested, not just eyeballed once per project
