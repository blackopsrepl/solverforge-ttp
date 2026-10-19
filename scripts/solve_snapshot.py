#!/usr/bin/env python3
"""Solve one embedded instance in one shot: health-check the app, POST the plan
with a duration, poll to a terminal lifecycle state, then print the score next
to the instance's best-known bound."""
import json
import re
import os
import sys
import time
import urllib.request

BASE = "http://127.0.0.1:" + os.environ.get("PORT", "7860")
instance = sys.argv[1] if len(sys.argv) > 1 else "NL4"
seconds = int(sys.argv[2]) if len(sys.argv) > 2 else 60

BEST_KNOWN = {"NL4": 8276}


def call(path, body=None):
    data = None if body is None else json.dumps(body).encode()
    extra = {} if data is None else {"content-type": "application/json"}
    with urllib.request.urlopen(
        urllib.request.Request(BASE + path, data=data, headers=extra)
    ) as resp:
        return json.loads(resp.read().decode())


while "status" not in json.dumps(call("/health")):
    time.sleep(0.5)
plan = call("/demo-data/" + instance)
plan["duration_seconds"] = seconds
job = call("/jobs", plan)["id"]
while True:
    s = call("/jobs/%s/status" % job)
    if s["lifecycleState"] in ("COMPLETED", "CANCELLED", "FAILED"):
        break
    time.sleep(0.5)
result = {
    "instance": instance,
    "seconds": seconds,
    "job_id": job,
    "current_score": s["currentScore"],
    "best_score": s["bestScore"],
}
bk = BEST_KNOWN.get(instance)
if bk is not None and s["bestScore"]:
    soft = int(re.search(r"-?\d+", s["bestScore"].split("/")[1]).group())
    # Travel is penalized, so the score's soft level is negative the distance.
    result["travel_distance"] = -soft
    result["best_known"] = bk
    result["gap_pct"] = round(100.0 * (-soft - bk) / bk, 2)
print(json.dumps(result, indent=1))