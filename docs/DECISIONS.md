# Decisions

## Global Apply/Reset architecture (Option B)

Decision: keep the Rust command contract unchanged and coordinate draft application in the frontend through a global draft store. The store plans editor commands synchronously, sends batchable edits in one `apply_batch`, then handles inventory clone/remove structure edits in the required sequence.

Why: Rust `apply_batch` is already atomic for batchable save edits, so the frontend can remove cross-panel partial-apply behavior without adding a new compound Rust command. Reset is global-only and returns every editor to the latest committed checkpoint.

Migration note: this keeps a future Option A path open. If the inventory structural window becomes a real problem, the frontend planner API can feed a Rust `apply_drafts` command later without changing panel-level UI semantics.
