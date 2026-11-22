#!/usr/bin/env python3
# SPDX-License-Identifier: AGPL-3.0-or-later
"""Package Admin and optionally verify separate pinned dependency ZIPs."""

import argparse
import json
from pathlib import Path
from zipfile import ZIP_DEFLATED, ZipFile

from provider_abi import check_abi


ROOT = Path(__file__).resolve().parents[1]
PLUGIFY = "game/csgo/addons/plugify"


def verify_dependencies(folder, platform):
    lock = json.loads((ROOT / "runtime/s2sdk.json").read_bytes())
    admin = json.loads((ROOT / "imomoe_admin.pplugin").read_bytes())
    declared = {item["name"]: item["constraints"] for item in admin["dependencies"] if "constraints" in item}
    envs = f"{PLUGIFY}/envs"
    for name, path, asset in (
        ("s2sdk", "plugify-plugin-s2sdk/plugify-plugin-s2sdk.pplugin",
         f"s2sdk-fork-{lock['version']}-{lock['commit'][:7]}-{platform}.zip"),
        ("imomoe_native_base", "imomoe_native_base/imomoe_native_base.pplugin",
         f"imomoe-native-base-{declared['imomoe_native_base']}-{platform}.zip"),
    ):
        with ZipFile(folder / asset) as archive:
            loaded = json.loads(archive.read(f"{envs}/{path}"))
            check_abi(loaded, json.loads((ROOT / "bindings" / f"{name}.pplugin").read_bytes()))
            if declared[name] != loaded["version"]:
                raise ValueError(f"Admin does not pin the selected {name} version")
            if name == "s2sdk" and json.loads(archive.read(f"{envs}/plugify-plugin-s2sdk/s2sdk-fork.json")) != lock:
                raise ValueError("SDK fork identity mismatch")


def package(archive, binary, platform, dependency_dir=None):
    windows = platform == "windows-x64"
    magic = b"MZ" if windows else b"\x7fELF"
    prefix, suffix = ("", ".dll") if windows else ("lib", ".so")
    envs = f"{PLUGIFY}/envs"
    manifest = f"{envs}/imomoe_admin/imomoe_admin.pplugin"
    files = {
        manifest: ROOT / "imomoe_admin.pplugin",
        f"{envs}/imomoe_admin/bin/{prefix}imomoe_admin{suffix}": binary,
        "LICENSE": ROOT / "LICENSE",
    }
    for name in ("admin.jsonc", "announcements.jsonc", "translations/en.jsonc", "translations/zh-CN.jsonc",
                 "schemas/admin.schema.json", "schemas/announcements.schema.json", "schemas/translations.schema.json"):
        files[f"{PLUGIFY}/configs/imomoe_admin/{name}"] = ROOT / "configs/imomoe_admin" / name

    # Validate source material before writing, then verify the actual ZIP entries.
    for name, path in files.items():
        if not path.is_file():
            raise FileNotFoundError(path)
        if "/bin/" in name:
            with path.open("rb") as stream:
                if stream.read(len(magic)) != magic:
                    raise ValueError(f"Binary does not match {platform}: {path}")
    if dependency_dir:
        verify_dependencies(dependency_dir, platform)
    archive.parent.mkdir(parents=True, exist_ok=True)
    with ZipFile(archive, "x", compression=ZIP_DEFLATED) as out:
        for name, path in files.items():
            out.write(path, name)
    with ZipFile(archive) as out:
        if out.read(manifest) != files[manifest].read_bytes():
            raise ValueError("Packaged Admin manifest differs from source")
        if set(out.namelist()) != files.keys() or out.testzip() is not None:
            raise ValueError("Admin ZIP failed inventory or CRC validation")
    print(f"Verified Admin package: {archive}")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--platform", choices=("windows-x64", "linux-x64"), required=True)
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--dependency-dir", type=Path, help="directory containing both pinned shared ZIPs for pairing verification")
    parser.add_argument("--archive", type=Path, required=True)
    args = parser.parse_args()
    package(args.archive, args.binary, args.platform, args.dependency_dir)
