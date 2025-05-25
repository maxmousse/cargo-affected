# 🧠 cargo-affected

> A tool to detect and operate on crates affected by source code changes in a Rust workspace.

## 🔍 Goal

`cargo-affected` helps Rust developers and teams working in **monorepos** or **multi-crate workspaces** identify which crates are impacted by code changes. It aims to provide a fast and reliable way to run operations — such as testing, linting, or building — only on the affected parts of a workspace.

## 🧩 Problem It Solves

In large Rust workspaces, running `cargo test` or `cargo build` across all crates can be:
- ❌ Time-consuming
- ❌ Resource-inefficient
- ❌ Overkill when only a small portion of the codebase has changed

### `cargo-affected` solves this by:
- Analyzing **changes in the Git history** (e.g., between two commits or branches)
- Mapping those changes to **specific crates** using `cargo metadata`
- Traversing **reverse dependencies** to detect indirect impact
- Allowing developers and CI pipelines to act **only where needed**

## ✅ Example Use Cases

- Speed up CI by only testing what’s changed
- Improve local iteration time for large workspaces
- Pre-commit or pre-push hooks that guard only impacted code
- Planning releases or changelogs based on actual impact

---

> 🚧 This project is in early development. The initial goal is to support affected crate detection and command execution based on Git diffs.
