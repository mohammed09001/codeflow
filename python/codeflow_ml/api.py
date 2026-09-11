"""Versioned local JSON backend contract used by codeflowd/CLI adapters."""
import json

API_VERSION = "v1"


def paginate(items: list[dict], limit: int = 100, cursor: int = 0) -> dict:
    if limit < 0 or cursor < 0:
        raise ValueError("invalid pagination budget")
    page = items[cursor:cursor + limit]
    return {"api_version": API_VERSION, "items": page, "next_cursor": cursor + len(page) if cursor + len(page) < len(items) else None}


def dispatch(request: dict, views: dict[str, object]) -> str:
    if request.get("api_version", API_VERSION) != API_VERSION:
        return json.dumps({"error": "unsupported_api_version", "api_version": API_VERSION}, sort_keys=True)
    operation = request.get("operation")
    if operation not in views:
        return json.dumps({"error": "unknown_operation"}, sort_keys=True)
    return json.dumps({"api_version": API_VERSION, "operation": operation, "result": views[operation]}, sort_keys=True)
