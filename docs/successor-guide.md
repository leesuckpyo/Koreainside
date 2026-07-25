# Korea Inside Successor Guide

## Purpose

This is the stable onboarding guide for a new contributor or successor.

It explains where current context belongs. It does not store a current priority, next task, Git state, approval state, or temporary page instruction.

## Reading Order

For execution authority, read in this order:

1. The user's current direct instruction
2. A task-specific Codex instruction or Implementation Contract approved by the user
3. Root `AGENTS.md`
4. Relevant professional standards
5. Relevant page specifications and implementation references

For current state and context, verify in this order:

1. The actual repository or operating interface
2. The latest handoff explicitly designated by the user
3. Current research, official sources, and measurement data
4. Historical handoffs, Decision Log, Project Memory, and historical records

## Project Orientation

- Project purpose and identity: `../PROJECT.md`
- Product principles: `product-constitution.md`
- Business principles: `business-operating-system.md`
- Codex working rules: `../AGENTS.md`
- Documentation map: `standards-hub.md`
- Official decisions: `decision-log.md`
- Historical decision context: `project-memory.md`
- Backlog candidates: `project/TODO.md`
- Long-term stages: `project/ROADMAP.md`

TODO and ROADMAP are planning references, not execution approval.

`../PROJECT.md` and `product-constitution.md` are the source of truth for the project's higher product identity and principles. Read and follow both.

Do not change the Brand, Mission, or Vision without explicit user approval. Do not duplicate their full text here.

## Current and Historical Handoffs

Use a handoff as the latest handoff only when the user explicitly designates it.

Do not select a handoff automatically merely because it has the newest date in the repository.

If the user has not designated a latest handoff, verify the actual repository state and report the current context needed for the task to the user.

The latest handoff may contain:

- current state
- completed work
- next-task context
- Git and deployment state

These items provide state, completed-work, and next-task-candidate context. They do not grant execution approval.

Do not read every historical handoff by default. Read a historical handoff only to recover an earlier decision reason, before-and-after context, completed history, or background missing from the latest handoff.

When latest and historical handoffs conflict, the latest handoff governs current context. A historical next task, Git state, approval state, or Codex instruction must never be re-executed as current work.

The latest handoff does not grant execution authority. A next task in the latest handoff remains a candidate until the user approves it.

If the actual repository or operating state differs from the handoff, verify the actual state and report the difference.

Historical handoffs are project records and should be preserved.

## Working Safely

- Protect existing URLs, navigation, shared components, CSS, JavaScript, images, and content unless the current task explicitly includes them.
- Keep important information as visible HTML text.
- Reuse the nearest approved component and page structure.
- Separate official facts, editorial judgment, and user instructions.
- Follow the approved diff, QA, commit, and push boundaries in `../AGENTS.md`.
- Treat page-specific documents as reference for that page, not permission to start work.

## When Context Is Missing

1. Check the latest handoff if one is explicitly designated.
2. Check the relevant standard and page specification.
3. Check Decision Log or Project Memory only for historical reasoning.
4. If the current task or approval boundary remains unclear, stop and ask the user.

Korea Inside should be continued through current evidence, explicit approval, and stable standards—not through stale execution instructions.
