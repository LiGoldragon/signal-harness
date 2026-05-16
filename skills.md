# skills — signal-persona-harness

*Per-repo agent guide for the delivery and transcript-observation
contract between `persona-router` and `persona-harness`.*

---

## Checkpoint — read before editing

Before changing code in this repo, read:

- `~/primary/skills/contract-repo.md`
- `~/primary/skills/architecture-editor.md`
- `~/primary/skills/architectural-truth-tests.md`
- `~/primary/skills/push-not-pull.md` (harness events push to the
  router, never polled)
- `~/primary/skills/subscription-lifecycle.md` (the canonical
  subscription FSM the transcript stream implements)
- `~/primary/skills/nix-discipline.md`
- this repo's `ARCHITECTURE.md`
- the consumers' `ARCHITECTURE.md` files
  (`persona-router/`, `persona-harness/`).

---

## What this repo is for

`signal-persona-harness` carries the delivery channel between the
router (request side) and one or more harness instances (reply / event
side). The router asks for delivery, interaction, cancellation,
status, and transcript observation; the harness pushes acks,
interaction resolutions, status, lifecycle events, and
transcript-observation events.

The transcript-observation subscription follows the canonical
lifecycle in `~/primary/skills/subscription-lifecycle.md`: open with
a typed `Subscribe`, push typed `TranscriptObservation` events,
close with a typed request-side `Retract` carrying the per-stream
token, end with a typed reply-side `HarnessSubscriptionRetracted`
ack echoing the token.

---

## What this repo owns

- `HarnessName` (typed name for one harness instance).
- The closed `HarnessRequest` enum (delivery requests, cancellations,
  interaction surfacing, status query, transcript subscribe +
  retract).
- The closed `HarnessEvent` enum (delivery acks, interaction
  resolutions, lifecycle events, transcript snapshot, transcript
  retraction ack).
- `DeliveryFailureReason`, `HarnessUnimplementedReason`,
  `HarnessOperationKind`, `HarnessHealth`, `HarnessReadiness`,
  `SubscriptionKind` — closed typed enums.
- `HarnessTranscriptToken`, `HarnessTranscriptSequence`,
  `HarnessSubscriptionRetracted` — transcript-stream identity and
  ack.
- The `Frame` type alias.
- Wire-form round-trip tests.

## What this repo does not own

- The router actor or its delivery state machine.
- The harness actor or its PTY adapter.
- Transport (UDS path, reconnect, timeouts).
- Terminal prompt cleanliness, input gates, and write-injection
  safety (owned by `signal-persona-terminal`, `persona-terminal`,
  and `terminal-cell`).

---

## Load-bearing invariants

- **Subscription close uses both sides.** The kernel grammar at
  `signal-core/macros/src/validate.rs:303–331` requires the
  `stream` block to name a request-side `Retract` variant; the
  reply-side `HarnessSubscriptionRetracted` ack is the final event
  consumers bind to. Both are present in `src/lib.rs`. Do not
  remove either.
- **Wire enums are closed.** No `Unknown` variant on any wire enum.
  `HarnessKind` is closed: `Codex`, `Claude`, `Pi`, `Fixture` — no
  `Other`. A fixture harness types as `Fixture`, not as a
  production kind. `DeliveryFailureReason` has three closed
  causes.
- **Every request variant declares a Signal root verb.** The
  `signal_channel!` declaration is the source of truth; the macro
  generates `HarnessRequest::signal_verb()` and round-trip tests
  assert every variant.
- **Skeleton honesty uses typed reasons.** A request that reaches
  a skeleton harness daemon and is not built yet returns
  `HarnessRequestUnimplemented` carrying typed
  `HarnessOperationKind` and `HarnessUnimplementedReason`, not a
  text error or a hang.
- **Transcript observation is pushed, never polled.** The harness's
  internal transcript event count is not the observation surface;
  `TranscriptObservation` on `HarnessTranscriptStream` is the only
  sanctioned way to read transcript progress.
- **Every transcript event carries a monotonic sequence.**
  `HarnessTranscriptSequence` is the per-event ordering field; the
  subscriber uses it to detect gaps and re-anchor after reconnect.
- **No runtime code.** No Kameo, Tokio, socket, redb, or daemon
  glue in this crate.
- **Round trips cover every variant.** rkyv length-prefixed frame
  round trips in `tests/round_trip.rs`; canonical NOTA examples in
  `examples/canonical.nota` with a parser test.
- **Pin upstream contracts via a named API reference.** Cargo deps
  declare `git = "..."` with a named branch/bookmark, never raw
  `rev = "..."`.

---

## Editing patterns

### Adding a new delivery failure reason

1. Add the variant to `DeliveryFailureReason`.
2. Add round-trip witnesses through rkyv and NOTA.
3. Update consumers' delivery error handling.

### Adding the `Fixture` harness kind (next contract bump)

`HarnessKind` is the closed kind enum carried on `HarnessBinding`.
Current daemon code carries three variants; `Fixture` is the next
bump:

1. Add `Fixture` to `HarnessKind`.
2. Add round-trip witness for the new variant.
3. Update `persona-harness` to surface fixture harnesses as
   `HarnessKind::Fixture`, not as `Codex` or `Claude`.
4. Update consumers' kind-dispatching code.

### Adding a new subscription kind

1. Read `~/primary/skills/subscription-lifecycle.md` end-to-end.
2. Add the typed subscribe payload, token, snapshot, and event
   records.
3. Add the new `stream` block in `signal_channel!`, with the
   subscribe request, the request-side retract variant, the
   reply-side ack, and the typed event variant. The kernel grammar
   enforces the close-is-Retract shape.
4. Witness the full subscribe → event → retract → ack → end
   lifecycle.

---

## NOTA codec quirk

The `signal_channel!` macro emits a request variant's NOTA head as
the **payload's record head**, not the Rust variant name. For
example, `HarnessRequest::HarnessTranscriptRetraction(HarnessTranscriptToken { .. })`
encodes as `(HarnessTranscriptToken (...))`, not
`(HarnessTranscriptRetraction ...)`. Canonical examples and
round-trip tests use the payload heads.

---

## See also

- this workspace's `skills/contract-repo.md`.
- this workspace's `skills/subscription-lifecycle.md`.
- this workspace's `skills/push-not-pull.md`.
- this workspace's `skills/architectural-truth-tests.md`.
- `signal-persona-system`'s `skills.md`,
  `signal-persona-terminal`'s `skills.md`, and `signal-criome`'s
  `skills.md` — sibling contracts using the same Path A subscription
  discipline.
