#!/usr/bin/env python3
"""Sweep the embedded NL benchmark: for each instance, POST the plan to the
app's /jobs, poll to a terminal lifecycle state, and persist the snapshot +
per-constraint analysis to results/.
"""
import json
import os
import subprocess
import sys
import time
import urllib.request
import urllib.error

ROOT = os.path.dirname(os.path.abspath(__file__))
APP = os.path.abspath(os.path.join(ROOT, ".."))
RESULTS = os.path.join(ROOT, "..", "results")
PORT = int(os.environ.get("PORT", "7860"))
SECONDS = int(os.environ.get("SECONDS", "60"))
INSTANCES = ["NL4", "NL6", "NL8", "NL10", "NL12", "NL14", "NL16"]

# RobinX best-known values for the classic NL set (fetched from
# robinxval.ugent.be bounds/solutions XMLs; see README References):
# LB=UB for NL4-NL10 => proven optima. NL12+ have only best-known schedules.
BEST_KNOWN = {
    "NL4": 8276,      # proven optimum (Easton & Trick)
    "NL6": 23916,     # proven optimum
    "NL8": 39721,     # proven optimum (Irnich 2010)
    "NL10": 59436,    # proven optimum (Uthus et al.)
    "NL12": 115072,
    "NL14": 203407,
    "NL16": 271476,
}


def api(base, path, method="GET", body=None, timeout=30):
    req = urllib.request.Request(base + path, method=method)
    data = None
    if body is not None:
        data = json.dumps(body).encode()
        req.add_header("content-type", "application/json")
    with urllib.request.urlopen(req, data=data, timeout=timeout) as resp:
        return json.loads(resp.read().decode())


def wait_health(base, deadline=180):
    end = time.time() + deadline
    while time.time() < end:
        try:
            r = api(base, "/health", timeout=3)
            if r.get("status") == "UP":
                return True
        except Exception:
            time.sleep(0.5)
    return False


def solve_job(base, plan):
    job = api(base, "/jobs", method="POST", body=plan)
    job_id = job["id"]
    t0 = time.time()
    hard_timeout = SECONDS + 180
    terminal = None
    while time.time() - t0 < hard_timeout:
        status = api(base, f"/jobs/{job_id}/status")
        state = status.get("lifecycleState")
        if state in ("COMPLETED", "FAILED", "CANCELLED"):
            terminal = state
            break
        time.sleep(1.0)
    snap = api(base, f"/jobs/{job_id}")  # full snapshot with solution
    try:
        analysis = api(base, f"/jobs/{job_id}/analysis")
    except Exception:
        analysis = None
    return job_id, terminal, snap, analysis


def main():
    os.makedirs(RESULTS, exist_ok=True)
    only = sys.argv[1:] or INSTANCES
    base = f"http://127.0.0.1:{PORT}"
    env = dict(os.environ, PORT=str(PORT))
    log_path = os.path.join(RESULTS, "server.log")
    binary = os.path.join(APP, "target", "release", "ttp")
    with open(log_path, "w") as log:
        server = subprocess.Popen([binary], env=env, stdout=log, stderr=subprocess.STDOUT, cwd=APP)
        try:
            if not wait_health(base):
                print("server did not come up", file=sys.stderr)
                return 1
            print(f"server up on {base}")
            rows = []
            for inst in only:
                plan = api(base, f"/demo-data/{inst}")
                job_id, terminal, snap, analysis = solve_job(base, plan)
                out = {
                    "instance": inst,
                    "seconds": SECONDS,
                    "job_id": job_id,
                    "terminal": terminal,
                    "snapshot": snap,
                    "analysis": analysis,
                }
                with open(os.path.join(RESULTS, f"{inst}.json"), "w") as f:
                    json.dump(out, f)
                best = snap.get("bestScore")
                cur = snap.get("currentScore")
                bk = BEST_KNOWN.get(inst)
                travel = -int(best.split("/")[1]) if best else None
                gap = round(100.0 * (travel - bk) / bk, 2) if travel is not None and bk else None
                rows.append((inst, best, cur, travel, bk, gap))
                print(f"{inst:6s} {str(terminal):10s} best={best} current={cur} "
                      f"travel={travel} best_known={bk} gap={gap}%")
            with open(os.path.join(RESULTS, "sweep-summary.md"), "w") as f:
                f.write("| instance | seconds | best score | travel | best known | gap |\n")
                f.write("| --- | --- | --- | --- | --- | --- |\n")
                for inst, best, _cur, travel, bk, gap in rows:
                    f.write(f"| {inst} | {SECONDS} | {best} | {travel} | {bk} | {gap}% |\n")
            return 0
        finally:
            server.terminate()
            try:
                server.wait(timeout=10)
            except subprocess.TimeoutExpired:
                server.kill()


if __name__ == "__main__":
    raise SystemExit(main())