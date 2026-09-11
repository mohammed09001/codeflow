"""Explicit failure for goals whose behavioral verifier has not been implemented."""
from __future__ import annotations

import sys


def main(goal: str) -> int:
    print(f"FAIL: {goal} verifier is not implemented; the goal remains OPEN.", file=sys.stderr)
    return 1
