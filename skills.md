# skills — signal-persona-harness

*Per-repo agent guide.*

## Checkpoint — read before editing

Before changing code in this repo, read:

- `~/primary/skills/contract-repo.md`
- `~/primary/skills/architecture-editor.md`
- `~/primary/skills/architectural-truth-tests.md`
- `~/primary/skills/push-not-pull.md` (harness events
  push to the router, never polled)
- `~/primary/skills/nix-discipline.md`
- this repo's `ARCHITECTURE.md`
- the consumers' `ARCHITECTURE.md` files
  (`persona-router/`, `persona-harness/`)

## What this repo owns

- `HarnessName` (typed name for one harness instance)
- The closed `HarnessRequest` enum (delivery requests +
  cancellations + interaction surfacing).
- The closed `HarnessEvent` enum (delivery acks +
  interaction resolutions + lifecycle events).
- `DeliveryFailureReason` (typed enum; no string-tagged
  reasons).
- The `Frame` type alias.
- The wire-form round-trip tests.

## What this repo does not own

- The router actor or its delivery state machine.
- The harness actor or its PTY adapter.
- Transport (UDS path, reconnect, timeouts).
- Terminal prompt cleanliness, input gates, and write-injection
  safety (owned by `signal-persona-terminal`, `persona-terminal`,
  and `terminal-cell`).
