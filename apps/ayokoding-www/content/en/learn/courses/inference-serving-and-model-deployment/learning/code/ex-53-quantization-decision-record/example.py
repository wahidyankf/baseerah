"""Example 53: Quantization Decision Record."""

from dataclasses import dataclass  # => stdlib only -- a decision record needs no framework


@dataclass
class QuantizationCandidate:  # => co-19: one candidate precision level, already measured (Example 52's shape)
    name: str  # => a label, useful only for reading print output
    precision_bits: int  # => kept for reference -- not directly used in the decision itself
    quality_score: float  # => co-19: the gate this candidate must clear to even be considered
    memory_gb: float  # => co-19: the tiebreaker among candidates that DO clear the gate


def decide(candidates: list[QuantizationCandidate], quality_floor: float) -> dict[str, str]:
    # => co-19: accept ONLY candidates clearing the quality floor; among those, prefer the smallest memory
    accepted = [c for c in candidates if c.quality_score >= quality_floor]  # => co-19: the SAFETY gate, applied first
    if not accepted:  # => every candidate failed the quality gate -- there is no safe choice
        return {"decision": "reject_all", "reason": f"no candidate clears quality floor {quality_floor}"}
    winner = min(accepted, key=lambda c: c.memory_gb)  # => co-19: among SAFE candidates, smallest wins
    # => "smallest memory that is still safe" is the whole decision rule, in one sentence
    return {"decision": winner.name, "reason": f"smallest memory ({winner.memory_gb} GB) among candidates >= {quality_floor}"}


candidates = [  # => the SAME three measured candidates from Example 52
    QuantizationCandidate("fp16", 16, 100.0, 13.04),  # => highest quality, largest memory
    QuantizationCandidate("int8", 8, 98.5, 6.52),  # => the usual middle-ground choice
    QuantizationCandidate("int4", 4, 94.0, 3.26),  # => smallest memory, lowest quality
]
record_strict = decide(candidates, quality_floor=97.0)  # => a strict floor rules out int4
record_loose = decide(candidates, quality_floor=90.0)  # => a loose floor admits everything -- smallest wins
# => the SAME three candidates, the SAME decision function -- only the quality floor input changes
print(record_strict)  # => Output: {'decision': 'int8', 'reason': 'smallest memory (6.52 GB) among candidates >= 97.0'}
print(record_loose)  # => Output: {'decision': 'int4', 'reason': 'smallest memory (3.26 GB) among candidates >= 90.0'}
# => this IS a decision record -- the "reason" field is what a real quantization ADR would document

assert record_strict["decision"] == "int8"  # => co-19: the floor EXCLUDED int4 despite its smaller memory
# => memory alone never wins -- it only breaks ties AMONG candidates that already passed the gate
assert record_loose["decision"] == "int4"  # => co-19: with a looser floor, the smallest safe option wins
# => same code path, same gate logic -- the OUTCOME changed only because the threshold moved
print("ex-53 OK")  # => a self-check marker confirming the decision flips correctly with the quality floor
