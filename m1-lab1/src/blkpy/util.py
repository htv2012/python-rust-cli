#!/usr/bin/env python3
import json
import subprocess

__all__ = ["run_lsblk", "find_all", "find_first"]


def run_lsblk() -> dict:
    cmd = ["lsblk", "-J"]  # , "-o", "name,type,mountpoints"]
    process = subprocess.run(
        cmd,
        text=True,
        capture_output=True,
    )

    data = json.loads(process.stdout)
    return data


def find_all(root):
    if isinstance(root, list):
        for sub_root in root:
            yield from find_all(sub_root)
    elif isinstance(root, dict):
        if "name" in root:
            yield root
        yield from find_all(root.get("children", []))
    else:
        raise ValueError(f"Value is not a dict nor a list: {root!r}")


def find_first(root, target: str):
    for node in find_all(root):
        if node["name"] == target:
            return node
    raise ValueError(f"Not found: {target}")