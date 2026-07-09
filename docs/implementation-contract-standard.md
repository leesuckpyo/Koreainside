# Implementation Contract Standard

## Document Metadata

Layer : L2
Status : Active
Authority Type : Workflow Standard
Primary Responsibility : Standard format for owner-approved implementation contracts
Source of Truth For : Implementation Contract structure, required contract fields, approval boundary, validation boundary, and stop conditions
Not Responsible For : Product philosophy, business strategy, Codex behavior rules, page-specific standards, reference records, history records, sprint history
Higher Priority Documents : Current User Instruction, AGENTS.md, PROJECT.md
Related Documents : AGENTS.md, PROJECT.md, docs/standards-hub.md
Change Policy : Owner approval required before change
Last Reviewed : 2026-07-09
Review Trigger : Codex workflow, implementation approval process, validation method, git boundary, or sprint operating rule changes

## Purpose

This document defines the standard Implementation Contract format for Korea Inside.

No implementation work may begin unless the owner has approved an explicit Implementation Contract.

## Core Rule

Conversation is for design.

Repository is the source of truth for implementation.

Codex must not implement from conversation memory.

Codex must implement only from:

- Current user instruction
- Current repository state
- Project Base documents
- Approved Implementation Contract

If the instruction, repository state, or documentation is unclear, incomplete, or conflicting, Codex must stop and report.

Unknown → STOP → Report → Ask → Never Assume.

## Required Implementation Contract Fields

Every Implementation Contract must define the following fields.

### 1. Task Goal

Define the exact goal of the task.

The goal must be specific enough to verify after implementation.

### 2. Target Files

List every file that may be created, modified, deleted, moved, or renamed.

If a file is not listed, Codex must not touch it.

### 3. Allowed Changes

Define what Codex is allowed to change.

Allowed changes must be limited to the approved task scope.

### 4. Prohibited Changes

Define what Codex must not change.

This must include any protected files, unrelated sections, formatting-only edits, broad refactoring, line-ending normalization, encoding changes, file movement, deletion, staging, commit, or push unless explicitly approved.

### 5. Required Source Documents

List the documents Codex must read before implementation.

Project Base must be included unless the owner explicitly limits the task.

Project Base documents are:

- PROJECT.md
- AGENTS.md
- docs/product-constitution.md
- docs/business-operating-system.md
- docs/standards-hub.md

### 6. Validation Method

Define how the result must be checked.

Examples:

- git diff
- git diff --check
- affected file review
- link check
- browser check
- responsive check
- build check
- lint check

### 7. Git Boundary

Define what Git actions are allowed.

Typical boundaries:

- Modify only, no stage
- Stage only approved files
- Commit only after approval
- Push only after separate approval

### 8. Stop Conditions

Define when Codex must stop and report instead of continuing.

Stop conditions must include at least:

- Target file missing
- Required source document missing
- Scope conflict
- Instruction conflict
- Existing document conflict
- Ambiguous owner intent
- Need for file creation not approved
- Need for deletion, movement, rename, merge, or broad refactor not approved
- Unexpected diff outside approved files

## Standard Contract Template

Use the following template when preparing an Implementation Contract.

```text
Implementation Contract

Task Goal:
-

Target Files:
-

Allowed Changes:
-

Prohibited Changes:
-

Required Source Documents:
-

Validation Method:
-

Git Boundary:
-

Stop Conditions:
-
```

## Approval Rule

Codex may diagnose and report without an Implementation Contract if the owner explicitly requests diagnosis only.

Codex may not implement, modify files, stage, commit, or push without an approved Implementation Contract.

## Review Rule

After implementation, Codex must report:

* Modified files
* Actual change scope
* Validation results
* Git status
* Diff summary
* Whether stage, commit, or push was performed

Codex must stop after reporting unless the owner explicitly approves the next Git step.
