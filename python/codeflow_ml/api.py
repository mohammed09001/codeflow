"""Versioned local JSON backend contract used by codeflowd/CLI adapters."""
import json

API_VERSION = "v1"


REQUIRED_VIEW_TYPES = ("ProjectView", "RevisionView", "CapabilityView", "GraphNodeView", "GraphEdgeView", "FeatureView", "SubsystemView", "WorkflowView", "EvidenceView", "SemanticZoomView", "QueryResult", "Cursor", "ErrorEnvelope", "PartialStatus", "UnknownStatus")


def revision_envelope(project_id: str, revision_id: str, payload: dict, partial: bool = False) -> dict:
    if not project_id or not revision_id:
        raise ValueError("revision-safe response requires project and revision IDs")
    return {"api_version": API_VERSION, "project_id": project_id, "revision_id": revision_id, "partial": partial, "result": payload}


def revision_cursor(project_id: str, revision_id: str, offset: int) -> str:
    if offset < 0:
        raise ValueError("invalid cursor offset")
    return json.dumps({"project_id": project_id, "revision_id": revision_id, "offset": offset}, sort_keys=True, separators=(",", ":"))


def validate_cursor(cursor: str, project_id: str, revision_id: str) -> int:
    value = json.loads(cursor)
    if value.get("project_id") != project_id or value.get("revision_id") != revision_id:
        raise ValueError("cursor revision mismatch")
    return int(value["offset"])


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
