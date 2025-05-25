# 🛣️ Roadmap: cargo-affected

This document outlines the key development milestones for `cargo-affected`. The goal is to build a focused, reliable CLI tool that understands and operates on impacted parts of a Rust workspace.

Each step aims to be small, testable, and to add meaningful utility.

---

## ✅ 1. List Workspace Crates

**Goal**: Display all crates in the current workspace.

- Use `cargo metadata` to extract crate names and paths.
- CLI:
  '''
  cargo affected list
  '''

---

## 🛠 2. Display Workspace Graph

**Goal**: Print or visualize the dependency graph of the workspace.

- Show which crates depend on which.
- Start with basic text output; later support DOT/JSON.
- CLI:
  '''
  cargo affected graph
  '''

---

## ⚙️ 3. Query the Graph Manually

**Goal**: Accept a set of “updated” crates and return affected dependents.

- Useful for scripted workflows.
- CLI:
  '''
  cargo affected query --updated crate-a crate-b
  '''

---

## ⚙️ 4. Diff: Detect Crates Affected by Uncommitted Changes

**Goal**: Use `git diff` against HEAD to find affected crates.

- Map file changes to crate ownership.
- Traverse graph to find dependents.
- CLI:
  '''
  cargo affected detect --uncommitted
  '''

---

## ⚙️ 5. Diff: Detect Crates Affected Between Two Commits

**Goal**: Allow Git range queries.

- Use `git diff <rev1> <rev2>` or `--since`/`--until`.
- CLI:
  '''
  cargo affected detect --since main
  cargo affected detect --since abc123 --until def456
  '''

---

## ⚙️ 6. Diff: Handle Branch Comparisons

**Goal**: Improve accuracy when comparing branches (e.g., merge-base).

- Detect common ancestor and changes since.
- CLI:
  '''
  cargo affected detect --since origin/main
  '''

---

## ⚙️ 7. Config: Customize Files That Affect a Crate

**Goal**: Let users define which files should trigger a crate as “touched”.

- Use `[package.metadata.affected]` in `Cargo.toml`.
- Example:
  '''
  [package.metadata.affected]
  watch = ["schemas/", "README.md"]
  '''

---

## ⚙️ 8. Config: Define Implicit Crate Dependencies

**Goal**: Add virtual links between crates that aren't in `Cargo.toml`.

- Useful for shared resources or external conventions.
- Example:
  '''
  [workspace.metadata.affected.links]
  "crate-a" = ["crate-b", "crate-utils"]
  '''

---

> ℹ️ These milestones define the initial working version of `cargo-affected`. Each one is a self-contained, shippable feature.
