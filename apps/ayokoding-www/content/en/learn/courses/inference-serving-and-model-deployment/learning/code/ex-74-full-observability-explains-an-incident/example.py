"""Example 74: Full Observability Explains an Incident."""


def diagnose(dashboard: dict[str, float | int]) -> str:
    # => co-26: a simple, deterministic rule chain -- real observability tooling automates exactly this reasoning
    if dashboard["preemption_rate"] > 0.3:  # => co-14: the FIRST, highest-priority signal this rule chain checks
        return "likely cause: cache pressure causing frequent preemption -- check admission control / batch size"
    if dashboard["queue_depth"] > 15 and dashboard["batch_occupancy"] < 0.5:  # => co-13/co-12: queueing WHILE underused
        return "likely cause: undersized replica count -- requests queueing while GPU sits underused"
    if dashboard["itl_p50_ms"] > 40:  # => co-15: the LAST signal checked -- a pure latency/throughput tradeoff symptom
        return "likely cause: oversized batch -- trading latency for throughput beyond the configured SLO"
    return "no anomaly detected"  # => none of the three rules fired -- the dashboard looks healthy


incident_dashboard = {"queue_depth": 12, "batch_occupancy": 0.92, "itl_p50_ms": 20, "preemption_rate": 0.4}
# => co-26: the SAME shape Example 64's build_dashboard() produced -- preemption_rate is the giveaway here
healthy_dashboard = {"queue_depth": 3, "batch_occupancy": 0.6, "itl_p50_ms": 18, "preemption_rate": 0.05}
# => low queue, low occupancy, low preemption -- none of the three rules above have a reason to fire

print(diagnose(incident_dashboard))
# => Output: likely cause: cache pressure causing frequent preemption -- check admission control / batch size
print(diagnose(healthy_dashboard))  # => Output: no anomaly detected
# => the SAME function, the SAME rule chain -- only the input dashboard's numbers changed

assert diagnose(incident_dashboard).startswith("likely cause: cache pressure")  # => co-26: correctly diagnosed the incident
# => this is the payoff of everything from co-06 through co-26: one dashboard number resolves to one root cause
assert diagnose(healthy_dashboard) == "no anomaly detected"  # => co-26: observability also confirms when NOTHING is wrong
print("ex-74 OK")  # => a self-check marker confirming the rule chain diagnosed both dashboards correctly
# => capacity, batching, memory, deployment, and observability all converge into this one function
# => a real on-call runbook is this same if-chain, just with links to the matching dashboard panels
