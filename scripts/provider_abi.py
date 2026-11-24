# SPDX-License-Identifier: AGPL-3.0-or-later
"""Compare the ABI declared by a consumer with the descriptor Plugify loads."""


def check_abi(provider, consumer):
    if provider["name"] != consumer["name"]:
        raise ValueError(f"Expected {consumer['name']}, got {provider['name']}")

    def kind(part):
        return part["type"], bool(part.get("ref")), part.get("prototype"), part.get("enum")

    def signature(method):
        return (
            method.get("funcName", method["name"]),
            kind(method["retType"]),
            tuple(kind(part) for part in method.get("paramTypes", [])),
        )

    for field in ("methods", "prototypes"):
        actual = {entry["name"]: signature(entry) for entry in provider.get(field, [])}
        for entry in consumer.get(field, []):
            if actual.get(entry["name"]) != signature(entry):
                raise ValueError(f"{provider['name']} ABI mismatch: {field} {entry['name']}")
