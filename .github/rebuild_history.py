from __future__ import annotations

import json
import os
import re
import shutil
import subprocess
import tarfile
import tempfile
from pathlib import Path

ROOT = Path.cwd()
FINAL_REF = "3e5041eba20df56fdf546eeabcce551a0141f386"
AUTHOR_NAME = "Tommaso Verardi"
AUTHOR_EMAIL = "129632197+tommy4377@users.noreply.github.com"

MILESTONES = [
    {
        "version": "0.1.0",
        "source": "8c2c458c5df735003548148664301dfc4f94ee8d",
        "date": "2026-01-12T10:57:40Z",
        "message": "feat(core): establish Tunevex Windows tuning engine",
        "title": "Core tuning engine",
        "summary": "The first Tunevex milestone establishes the Rust/Tauri desktop shell and the initial Windows tweak catalog.",
        "features": [
            "CPU, network, privacy, security, gaming, storage and input tweak modules",
            "stateful apply and undo operations",
            "Svelte desktop interface",
            "Windows-only elevated system utility foundation",
        ],
    },
    {
        "version": "0.2.0",
        "source": "83642ffce1f9f59e64db5004281b1720d6dcd6ba",
        "date": "2026-01-21T12:19:10Z",
        "message": "feat(ui): consolidate shared UI and Windows 11 shell",
        "title": "Shared UI and Windows 11 shell",
        "summary": "This milestone consolidates the interface around reusable components and a Windows 11-style desktop shell.",
        "features": [
            "shared cards, buttons, badges, section headers and dashboard layouts",
            "Mica and acrylic-style window treatment",
            "cleaner system monitoring and quick actions",
            "removal of abandoned Explorer Patcher integration",
        ],
    },
    {
        "version": "0.3.0",
        "source": "324e5566db65f913083e43cc147a3210bd43a30f",
        "date": "2026-04-09T00:15:41Z",
        "message": "refactor(core): migrate operations toward native Windows APIs",
        "title": "Native Windows backend",
        "summary": "Tunevex moves many checks and system operations away from PowerShell toward native Rust, registry, service and Windows API paths.",
        "features": [
            "native registry and service operations for more tweak families",
            "safer monitoring and device handling",
            "stronger state checks and rollback semantics",
            "large duplicate-tweak and reliability cleanup",
        ],
    },
    {
        "version": "0.4.0",
        "source": "197a8d88bb46822d45203c2e6475b1fdd51115aa",
        "date": "2026-04-11T22:03:59Z",
        "message": "feat(ai): add Gemini advisor and persistent context",
        "title": "AI Advisor",
        "summary": "The optional AI Advisor arrives as an explicit user-facing feature built around Gemini and the live Tunevex system profile.",
        "features": [
            "AI analyze, chat and diagnose workflows",
            "Gemini key storage through Windows Credential Manager",
            "system-profile-aware recommendations",
            "persistent AI context and chat sessions",
        ],
    },
    {
        "version": "0.5.0",
        "source": "d9369d029a1bb7da1a716e7fef2c85890e880d7e",
        "date": "2026-04-12T00:18:39Z",
        "message": "feat(startup): add safety-aware startup analysis",
        "title": "Startup analyzer",
        "summary": "Startup management becomes a first-class feature with safety classification and stronger protection for Windows-critical entries.",
        "features": [
            "startup scanning across common Windows autostart locations",
            "AI-assisted startup recommendations",
            "critical Windows process, service and task exclusions",
            "detailed startup scan reports",
        ],
    },
    {
        "version": "0.6.0",
        "source": "382f31a3d5d017b54714b159345a7ab70d9c2869",
        "date": "2026-09-05T21:02:01Z",
        "message": "feat(profiles): add portable profiles and reliable rollback",
        "title": "Profiles and rollback reliability",
        "summary": "The final pre-1.0 milestone focuses on portable profiles, accurate state detection and safer rollback behavior.",
        "features": [
            "JSON tweak profile import and export",
            "preview and validation before applying profiles",
            "more accurate live state detection",
            "device-aware backup and rollback improvements",
        ],
    },
    {
        "version": "1.0.0",
        "source": FINAL_REF,
        "date": "2026-09-21T13:35:00Z",
        "message": "release: launch Tunevex 1.0.0",
        "title": "Tunevex 1.0",
        "summary": "Tunevex 1.0 completes the product rename, compatibility migration and repository hardening for a clean portable release.",
        "features": [
            "complete Tunevex product, package and executable branding",
            "legacy TommyTweaker data, profile and credential compatibility",
            "repository governance, CI and Dependabot",
            "portable-only Windows release pipeline",
        ],
    },
]

DROP_DIRS = {
    ".opencode",
    ".serena",
    ".claude",
    ".cursor",
    ".windsurf",
    ".vscode",
    "tweak",
    "optimizer-master",
    "SecureUxTheme-master",
    "TommyTweaker-master",
    "winutil-main",
}

DROP_FILES = {
    "AGENTS.MD",
    "STATUS.MD",
    "BUGFIX_CHECKLIST.md",
    "# TommyTweaker — Bug Fix Checklist.md",
    "problems.md",
    "ai_advisor.md",
    "tommy-tweaker-memory.json",
    "all_tweaks_audit.txt",
    "audit_tweaks.txt",
    "implementation_plan.md",
    "prompt.md",
    "promptEAnalisi.txt",
    "src-tauri/build_log.txt",
    "src-tauri/output.txt",
    "src-tauri/verify_display_log.txt",
    "static/Thumbs.db",
}

DROP_PREFIXES = (
    "FINAL_CORRECTED_ROADMAP",
    "IMPLEMENTATION_LOG",
    "IMPLEMENTATION_ROADMAP",
    "MASTER_IMPLEMENTATION_GUIDE",
    "MASTER_IMPLEMENTATION_PLAN",
    "QA_AUDIT_REPORT",
)

TEXT_EXTENSIONS = {
    ".rs", ".toml", ".lock", ".json", ".js", ".ts", ".svelte", ".md",
    ".yml", ".yaml", ".html", ".css", ".ps1", ".bat", ".txt", ".xml",
}

HISTORICAL_METADATA = [
    "LICENSE",
    ".gitignore",
    "src-tauri/.gitignore",
    ".github/workflows/ci.yml",
    ".github/workflows/release.yml",
]

FINAL_METADATA = [
    "CONTRIBUTING.md",
    "SECURITY.md",
    "SUPPORT.md",
    "CODE_OF_CONDUCT.md",
    "NOTICE.md",
    ".github/CODEOWNERS",
    ".github/dependabot.yml",
    ".github/pull_request_template.md",
    ".github/ISSUE_TEMPLATE/bug_report.yml",
    ".github/ISSUE_TEMPLATE/feature_request.yml",
    ".github/ISSUE_TEMPLATE/config.yml",
]

def run(args, cwd=None, env=None, check=True):
    result = subprocess.run(
        args,
        cwd=str(cwd or ROOT),
        env=env,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
    )
    if check and result.returncode != 0:
        print(result.stdout)
        raise RuntimeError("Command failed: " + " ".join(args))
    return result

def copy_path(source, destination):
    source = Path(source)
    destination = Path(destination)
    if source.is_dir():
        if destination.exists():
            shutil.rmtree(destination)
        destination.parent.mkdir(parents=True, exist_ok=True)
        shutil.copytree(source, destination)
    else:
        destination.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(source, destination)

def extract_ref(ref, destination):
    destination.mkdir(parents=True, exist_ok=True)
    archive = destination.parent / (ref[:12] + ".tar")
    with archive.open("wb") as handle:
        result = subprocess.run(["git", "archive", "--format=tar", ref], stdout=handle)
    if result.returncode != 0:
        raise RuntimeError("git archive failed for " + ref)
    with tarfile.open(archive, "r") as bundle:
        bundle.extractall(destination)
    archive.unlink()

def remove_noise(stage, is_final):
    for directory in sorted(stage.rglob("*"), key=lambda item: len(item.parts), reverse=True):
        if directory.is_dir() and directory.name in DROP_DIRS:
            shutil.rmtree(directory, ignore_errors=True)

    for relative in DROP_FILES:
        target = stage / relative
        if target.is_dir():
            shutil.rmtree(target, ignore_errors=True)
        elif target.exists():
            target.unlink()

    for target in list(stage.iterdir()):
        if any(target.name.upper().startswith(prefix) for prefix in DROP_PREFIXES):
            if target.is_dir():
                shutil.rmtree(target, ignore_errors=True)
            else:
                target.unlink()
        elif target.is_file() and target.suffix.lower() in {".log", ".txt"}:
            target.unlink()

    tauri = stage / "src-tauri"
    if tauri.exists():
        for target in tauri.glob("verify_*.ps1"):
            target.unlink()

    for relative in [
        "src-tauri/resources/startallback",
        "src-tauri/resources/secureuxtheme",
        "src-tauri/public/startallback files",
    ]:
        target = stage / relative
        if target.exists():
            shutil.rmtree(target, ignore_errors=True)

    for target in list(stage.rglob("*")):
        lowered = target.name.lower()
        if target.is_file() and any(word in lowered for word in ["screenshot", "screen-shot", "debug-shot"]):
            target.unlink()

    if not is_final:
        github_dir = stage / ".github"
        if github_dir.exists():
            shutil.rmtree(github_dir)
        for target in list(stage.glob("*.md")) + list(stage.glob("*.MD")):
            if target.name.lower() != "readme.md":
                target.unlink()

def remove_mcp_dev_tooling(stage):
    cargo = stage / "src-tauri" / "Cargo.toml"
    if cargo.exists():
        text = cargo.read_text(encoding="utf-8")
        text = "\n".join(
            line for line in text.splitlines()
            if "tauri-plugin-mcp-bridge" not in line
        ) + "\n"
        cargo.write_text(text, encoding="utf-8")

    lib_rs = stage / "src-tauri" / "src" / "lib.rs"
    if lib_rs.exists():
        text = lib_rs.read_text(encoding="utf-8")
        text = re.sub(
            r"(?ms)\n\s*#\[cfg\(debug_assertions\)\]\s*\n\s*let builder = builder\.plugin\(\s*tauri_plugin_mcp_bridge::Builder::new\(\).*?\);\s*",
            "\n",
            text,
        )
        lib_rs.write_text(text, encoding="utf-8")

    capabilities = stage / "src-tauri" / "capabilities"
    if capabilities.exists():
        for target in capabilities.rglob("*.json"):
            text = target.read_text(encoding="utf-8")
            lines = [line for line in text.splitlines() if "mcp-bridge" not in line]
            target.write_text("\n".join(lines) + "\n", encoding="utf-8")

def rebrand_historical(stage):
    replacements = [
        ("Tommy Tweaker", "Tunevex"),
        ("TommyTweaker", "Tunevex"),
        ("TOMMYTWEAKER", "TUNEVEX"),
        ("tommytweaker", "tunevex"),
        ("tommy_tweaker", "tunevex"),
        ("tommy-tweaker", "tunevex"),
    ]

    for target in stage.rglob("*"):
        if not target.is_file():
            continue
        if target.name == ".gitignore" or target.suffix.lower() in TEXT_EXTENSIONS:
            try:
                text = target.read_text(encoding="utf-8")
            except UnicodeDecodeError:
                continue
            for old, new in replacements:
                text = text.replace(old, new)
            target.write_text(text, encoding="utf-8")

def set_version(stage, version):
    package = stage / "package.json"
    if package.exists():
        data = json.loads(package.read_text(encoding="utf-8"))
        data["name"] = "tunevex"
        data["version"] = version
        data["description"] = "Tunevex - Windows system tuning and optimization utility"
        data["license"] = "MIT"
        package.write_text(json.dumps(data, indent=2) + "\n", encoding="utf-8")

    package_lock = stage / "package-lock.json"
    if package_lock.exists():
        data = json.loads(package_lock.read_text(encoding="utf-8"))
        data["name"] = "tunevex"
        data["version"] = version
        if isinstance(data.get("packages"), dict) and isinstance(data["packages"].get(""), dict):
            data["packages"][""]["name"] = "tunevex"
            data["packages"][""]["version"] = version
        package_lock.write_text(json.dumps(data, indent=2) + "\n", encoding="utf-8")

    cargo = stage / "src-tauri" / "Cargo.toml"
    if cargo.exists():
        text = cargo.read_text(encoding="utf-8")
        text = re.sub(r'(?m)^name = "[^"]+"', 'name = "tunevex"', text, count=1)
        text = re.sub(r'(?m)^version = "[^"]+"', 'version = "' + version + '"', text, count=1)
        text = re.sub(r'(?m)^description = "[^"]+"', 'description = "Windows system tuning and optimization utility"', text, count=1)
        text = re.sub(r'(?m)^authors = \[[^\]]*\]', 'authors = ["Tommaso Verardi"]', text, count=1)
        if "[lib]" in text:
            before, after = text.split("[lib]", 1)
            after = re.sub(r'(?m)^name = "[^"]+"', 'name = "tunevex_lib"', after, count=1)
            text = before + "[lib]" + after
        package_section = text.split("[dependencies]", 1)[0]
        if "license =" not in package_section:
            text = text.replace('edition = "2021"', 'edition = "2021"\nlicense = "MIT"\npublish = false', 1)
        cargo.write_text(text, encoding="utf-8")

    main_rs = stage / "src-tauri" / "src" / "main.rs"
    if main_rs.exists():
        text = main_rs.read_text(encoding="utf-8")
        text = re.sub(r"[A-Za-z0-9_]+_lib::run\(\)", "tunevex_lib::run()", text)
        main_rs.write_text(text, encoding="utf-8")

    tauri = stage / "src-tauri" / "tauri.conf.json"
    if tauri.exists():
        data = json.loads(tauri.read_text(encoding="utf-8"))
        data["productName"] = "Tunevex"
        data["version"] = version
        data["identifier"] = "com.tommy4377.tunevex"
        windows = data.get("app", {}).get("windows", [])
        if windows:
            windows[0]["title"] = "Tunevex"
        bundle = data.get("bundle")
        if isinstance(bundle, dict):
            bundle["active"] = False
        tauri.write_text(json.dumps(data, indent=2) + "\n", encoding="utf-8")

def write_historical_readme(stage, milestone):
    features = "\n".join("- " + item for item in milestone["features"])
    content = (
        "# Tunevex v" + milestone["version"] + "\n\n"
        + "## " + milestone["title"] + "\n\n"
        + milestone["summary"] + "\n\n"
        + "### Highlights\n\n"
        + features + "\n\n"
        + "### Safety\n\n"
        + "Tunevex changes Windows system settings. Create a restore point before applying groups of tweaks and review higher-risk controls individually.\n\n"
        + "### Development\n\n"
        + "    npm ci\n"
        + "    npm run check\n"
        + "    npm run tauri dev\n\n"
        + "### Release\n\n"
        + "This version is published as a single portable Windows executable named Tunevex.exe.\n\n"
        + "### License\n\n"
        + "MIT License. Copyright 2026 Tommaso Verardi.\n"
    )
    for candidate in [stage / "README.md", stage / "README.MD"]:
        if candidate.exists():
            candidate.unlink()
    (stage / "README.md").write_text(content, encoding="utf-8")

def copy_metadata(final_stage, stage, is_final):
    if not is_final:
        for relative in HISTORICAL_METADATA:
            source = final_stage / relative
            if source.exists():
                copy_path(source, stage / relative)
    else:
        for relative in FINAL_METADATA:
            source = final_stage / relative
            if source.exists():
                copy_path(source, stage / relative)

def refresh_lockfile(stage):
    cargo = stage / "src-tauri" / "Cargo.toml"
    if not cargo.exists():
        return
    lock = stage / "src-tauri" / "Cargo.lock"
    if lock.exists():
        lock.unlink()
    result = run(
        ["cargo", "generate-lockfile", "--manifest-path", str(cargo)],
        cwd=stage,
        check=False,
    )
    if result.returncode != 0:
        print("Lockfile refresh warning for", stage)
        print(result.stdout)

def prepare_stage(temp_root, final_stage, milestone):
    version = milestone["version"]
    is_final = version == "1.0.0"
    stage = temp_root / ("stage-" + version)
    extract_ref(milestone["source"], stage)
    remove_noise(stage, is_final)

    if not is_final:
        remove_mcp_dev_tooling(stage)
        rebrand_historical(stage)
        set_version(stage, version)
        copy_metadata(final_stage, stage, False)
        write_historical_readme(stage, milestone)
        refresh_lockfile(stage)

    return stage

def clear_worktree():
    for child in ROOT.iterdir():
        if child.name == ".git":
            continue
        if child.is_dir():
            shutil.rmtree(child)
        else:
            child.unlink()

def commit_stage(stage, milestone):
    clear_worktree()
    for child in stage.iterdir():
        copy_path(child, ROOT / child.name)

    run(["git", "add", "-A"])
    env = os.environ.copy()
    env["GIT_AUTHOR_NAME"] = AUTHOR_NAME
    env["GIT_AUTHOR_EMAIL"] = AUTHOR_EMAIL
    env["GIT_COMMITTER_NAME"] = AUTHOR_NAME
    env["GIT_COMMITTER_EMAIL"] = AUTHOR_EMAIL
    env["GIT_AUTHOR_DATE"] = milestone["date"]
    env["GIT_COMMITTER_DATE"] = milestone["date"]
    run(["git", "commit", "-m", milestone["message"]], env=env)
    return run(["git", "rev-parse", "HEAD"]).stdout.strip()

def main():
    temp_root = Path(tempfile.mkdtemp(prefix="tunevex-rebuild-"))
    final_stage = temp_root / "final-source"
    extract_ref(FINAL_REF, final_stage)
    remove_noise(final_stage, True)

    stages = []
    for milestone in MILESTONES:
        if milestone["version"] == "1.0.0":
            stage = temp_root / "stage-1.0.0"
            shutil.copytree(final_stage, stage)
        else:
            stage = prepare_stage(temp_root, final_stage, milestone)
        stages.append((milestone, stage))

    run(["git", "config", "user.name", AUTHOR_NAME])
    run(["git", "config", "user.email", AUTHOR_EMAIL])
    run(["git", "switch", "--orphan", "rebuilt-history"])
    clear_worktree()

    commits = []
    for milestone, stage in stages:
        sha = commit_stage(stage, milestone)
        commits.append((milestone["version"], sha))
        print("BUILT", milestone["version"], sha)

    run(["git", "push", "--force", "origin", "HEAD:refs/heads/rebuilt-history"])

    for version, sha in commits:
        tag = "v" + version
        run(["git", "tag", "-f", tag, sha])
        run(["git", "push", "--force", "origin", "refs/tags/" + tag])

    print("REBUILT_HEAD=" + commits[-1][1])
    for version, sha in commits:
        print("TAG v" + version + "=" + sha)

if __name__ == "__main__":
    main()
