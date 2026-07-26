# learning/code/ex-21-source-from-production-traffic/production_traffic.py
"""Worked Example 21: Source from Production Traffic."""  # => co-13: this file's own restated purpose, doubling as its module __doc__

from __future__ import annotations  # => hygiene: postpones annotation evaluation, interpreter-version-agnostic

# => co-13: sourcing a dataset from real production ticket logs -- fast and free, but it inherits production's OWN skew
PRODUCTION_TRAFFIC_CATEGORY_COUNTS: dict[str, int] = {  # => co-13: category -> how many real tickets of that category exist in the last 90 days
    "password-reset": 4_200,  # => co-13: by far the most common real-world category
    "billing": 1_100,  # => co-13
    "bug": 640,  # => co-13
    "feature-request": 60,  # => co-13: rare -- customers file few of these compared to support tickets
}  # => co-13: closes PRODUCTION_TRAFFIC_CATEGORY_COUNTS

TARGET_TASK_IMPORTANCE: dict[str, float] = {  # => co-13: how much Vantage's PRODUCT team actually cares about each category, independent of volume
    "password-reset": 0.15,  # => co-13: routine, low product-risk
    "billing": 0.30,  # => co-13: real financial and trust impact
    "bug": 0.35,  # => co-13: real reliability impact
    "feature-request": 0.20,  # => co-13: real roadmap signal
}  # => co-13: closes TARGET_TASK_IMPORTANCE -- sums to 1.00, deliberately NOT proportional to raw traffic volume


if __name__ == "__main__":  # => co-13: entry point -- runs only when this file executes directly, not on import
    total_traffic = sum(PRODUCTION_TRAFFIC_CATEGORY_COUNTS.values())  # => co-13: total sampled volume across all categories
    for category, count in PRODUCTION_TRAFFIC_CATEGORY_COUNTS.items():  # => co-13: show the SKEW a naive traffic-proportional sample would inherit
        traffic_share = count / total_traffic  # => co-13: what fraction of a traffic-proportional dataset this category would get
        importance_share = TARGET_TASK_IMPORTANCE[category]  # => co-13: what fraction it SHOULD get, by product importance
        print(f"  {category}: {traffic_share:.0%} of traffic-sourced data vs. {importance_share:.0%} target importance")  # => co-13
    feature_request_share = PRODUCTION_TRAFFIC_CATEGORY_COUNTS["feature-request"] / total_traffic  # => co-13: the category this bias hurts most
    print(f"feature-request would get only {feature_request_share:.0%} of a naive traffic-proportional dataset")  # => co-13
    assert feature_request_share < TARGET_TASK_IMPORTANCE["feature-request"] / 2, "a naive traffic-proportional sample must badly under-represent feature-request"  # => co-13
    print("MATCH: sourcing purely from production traffic silently under-represents the categories that matter but occur rarely")  # => co-13
    # => co-13: this is co-13's bias profile made concrete -- production traffic is free and fast, and it inherits whatever skew real usage already has
