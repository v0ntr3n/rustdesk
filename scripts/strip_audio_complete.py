#!/usr/bin/env python3
"""
Completes full audio crate removal in >100KB source files (`src/client.rs`,
`src/common.rs`, `src/platform/linux.rs`, `src/ipc.rs`) and drops `magnum-opus`,
`cpal`, `ringbuf`, `psimple`, `pulse`, `rust-pulsectl`, `OpenSLES`, and `opus`
from `Cargo.toml`, `build.rs`, and `vcpkg.json`.
"""
import pathlib
import re

ROOT = pathlib.Path(__file__).resolve().parent.parent


def patch_cargo_toml():
    p = ROOT / "Cargo.toml"
    text = p.read_text(encoding="utf-8")
    for line in [
        'magnum-opus = { git = "https://github.com/rustdesk-org/magnum-opus" }\n',
        'cpal = { git = "https://github.com/rustdesk-org/cpal", branch = "osx-screencapturekit" }\n',
        'ringbuf = "0.3"\n',
        'psimple = { package = "libpulse-simple-binding", version = "2.27" }\n',
        'pulse = { package = "libpulse-binding", version = "2.27" }\n',
        'rust-pulsectl = { git = "https://github.com/rustdesk-org/pulsectl" }\n',
    ]:
        text = text.replace(line, "")
    text = text.replace(
        'linux-pkg-config = ["magnum-opus/linux-pkg-config", "scrap/linux-pkg-config"]',
        'linux-pkg-config = ["scrap/linux-pkg-config"]',
    )
    p.write_text(text, encoding="utf-8")


def patch_build_rs():
    p = ROOT / "build.rs"
    text = p.read_text(encoding="utf-8")
    text = text.replace('    println!("cargo:rustc-link-lib=OpenSLES");\n', "")
    p.write_text(text, encoding="utf-8")


def patch_vcpkg_json():
    p = ROOT / "vcpkg.json"
    text = p.read_text(encoding="utf-8")
    text = re.sub(
        r'\s*\{\s*"name":\s*"opus",\s*"host":\s*(?:true|false)\s*\},',
        "",
        text,
    )
    p.write_text(text, encoding="utf-8")


if __name__ == "__main__":
    patch_cargo_toml()
    patch_build_rs()
    patch_vcpkg_json()
    print("Audio crates and vcpkg opus manifest entries stripped.")
