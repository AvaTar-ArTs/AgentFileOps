from __future__ import annotations

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
SCHEMA_DIR = ROOT / "protocol" / "schema"


def load_schema(name: str) -> dict:
    return json.loads((SCHEMA_DIR / name).read_text(encoding="utf-8"))


def test_ssh_transport_schema_requires_known_hosts_and_credential_refs():
    schema = load_schema("ssh-transport.schema.json")
    required = set(schema["required"])
    assert {"known_hosts_ref", "credential_ref"}.issubset(required)


def test_transport_schema_has_no_raw_secret_fields():
    text = json.dumps(load_schema("ssh-transport.schema.json")).lower()
    for forbidden in ["private_key", "password", "passphrase", "secret_value"]:
        assert forbidden not in text


def test_transport_timeouts_are_bounded():
    schema = load_schema("ssh-transport.schema.json")
    props = schema["properties"]
    assert props["connect_timeout_seconds"]["minimum"] == 1
    assert props["connect_timeout_seconds"]["maximum"] == 120
    assert props["operation_timeout_seconds"]["minimum"] == 1
    assert props["operation_timeout_seconds"]["maximum"] == 600


def test_inline_read_contract_is_bounded():
    schema = load_schema("ssh-transport.schema.json")
    read_limit = schema["properties"]["inline_read_bytes"]
    assert read_limit["minimum"] == 1
    assert read_limit["default"] == 1_048_576
    assert read_limit["maximum"] == 16_777_216


def test_transport_schema_uses_agentfileops_identity():
    schema = load_schema("ssh-transport.schema.json")
    assert schema["title"].startswith("AgentFileOps")
    assert "AvaTar-ArTs/AgentFileOps" in schema["$id"]
    assert "agentfs" not in schema["$id"].lower()


def test_connection_schema_references_transport_contract():
    connection = load_schema("connection.schema.json")
    assert connection["properties"]["transport"]["$ref"] == "ssh-transport.schema.json"
