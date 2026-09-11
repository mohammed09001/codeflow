#!/usr/bin/env python3
"""Verify that the Execution 04 registry describes the actual initial state."""
from __future__ import annotations

from common import ALLOWED_STATES, GOAL_IDS, ROOT, fail, head_sha, load_json


def main() -> int:
    if not (ROOT / ".git").exists() or not (ROOT / "Cargo.toml").is_file():
        return fail("repository root is not valid")
    try:
        sha = head_sha()
        goals = load_json("Execution/exec04/goals.json")
        state = load_json("Execution/exec04/state.json")
    except (OSError, ValueError, RuntimeError) as error:
        return fail(str(error))
    records = goals.get("goals")
    if goals.get("execution") != "04" or goals.get("schema_version") != 1 or not isinstance(records, list):
        return fail("goals registry schema is invalid")
    by_id = {record.get("id"): record for record in records if isinstance(record, dict)}
    if set(by_id) != GOAL_IDS or len(records) != len(GOAL_IDS):
        return fail("registry must contain exactly G00 through G16")
    for goal_id, record in by_id.items():
        if record.get("status") not in ALLOWED_STATES:
            return fail(f"{goal_id} has an invalid status")
        verifier = record.get("verifier")
        if not isinstance(verifier, str) or not (ROOT / verifier).is_file():
            return fail(f"{goal_id} verifier is absent")
        if record["status"] == "PASS" and (not record.get("proof_sha") or not record.get("proof_file")):
            return fail(f"{goal_id} is PASS without proof")
    if state.get("frontend_readiness") != "NOT_READY":
        return fail("frontend readiness must begin NOT_READY")
    if state.get("baseline_sha") != sha:
        return fail("baseline SHA does not equal current HEAD")
    print(f"PASS: baseline registry validated at {sha}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
