# INTENT — signal-harness

*The Signal contract between `persona-router` and `harness` —
bidirectional delivery, interaction, and observation. Companion to
`ARCHITECTURE.md` and `Cargo.toml`. Maintenance:
`primary/skills/repo-intent.md`.*

## Repo-scope only

This file carries only the intent that is FOR this `signal-harness`
contract. Workspace-shape intent stays in the primary workspace
`primary/INTENT.md`. Harness daemon intent stays in `harness/INTENT.md`.

## Why this repo exists

`signal-harness` is the **router-to-harness wire contract** — the
delivery channel between `persona-router` and one or more harness
instances. The router asks for delivery, interaction, cancellation,
status, and transcript observation; the harness pushes acknowledgements,
interaction resolutions, status, lifecycle events, and
transcript-observation events. Runtime actors, sockets, storage, and the
harness's tables live in `harness`; routing policy and delivery state
live in `router`.

## The channel shape — bidirectional

| Side | Role |
|---|---|
| Request side | `persona-router` sends delivery, interaction-prompt, cancellation, status-query, and transcript-subscription requests. |
| Reply / event side | `harness` emits delivery acks, interaction resolutions, skeleton honesty, status, lifecycle events, transcript snapshot, retraction ack, and `TranscriptObservation` events on the open stream. |

Bidirectional steady state: the router sends one request; the harness
emits one or more events. Lifecycle events (`HarnessStarted` /
`HarnessStopped` / `HarnessCrashed`) flow without paired requests.
Transcript observation is push-based: the router subscribes once per
harness, the harness emits a snapshot then `TranscriptObservation`
deltas, and the subscription closes via the canonical
Retract-closes-the-stream lifecycle.

## Wire vocabulary discipline — three-layer direction

Per `primary/skills/contract-repo.md` §"Public contracts use
contract-local operation verbs" and `primary/skills/component-triad.md`
§"Verbs come in three layers", the intended shape is:

- **Layer 1 (this crate):** contract-local operation roots in verb form.
  The `SignalVerb` wrappers retire; candidate domain verbs are `Deliver`
  (carries a `Message`), `Prompt`, `Cancel`, `Query` (carries a
  `Status`). Redundant `Harness*` prefixes drop where the crate
  namespace already supplies the context.
- **Layer 2 (daemon):** the harness's own typed Command enum, lowered
  from contract operations inside the daemon — never in this contract
  crate.
- **Layer 3 (observation):** payloadless Sema class labels via
  `ToSemaOperation`, for cross-component introspection only.

The harness IS a Persona component, so its observable surface is
standardized: the macro-injected `Tap(ObserverFilter)` /
`Untap(...)` observer-hook surface (operation / effect events) is
mandatory and is what `persona-introspect` subscribes to uniformly
across every Persona daemon. A domain-specific transcript stream may
sit alongside that standardized observability. The migration to this
shape is in progress; the target above is the intent.

## Channels are closed, boundaries are named

- Wire enums are closed. No `Unknown` escape hatch.
- Subscription close is request-side Retract carrying the per-stream
  token; the harness echoes the token and ends the stream after the
  final ack — the kernel grammar enforces close-is-Retract.
- A skeleton honesty reply (`RequestUnimplemented`) answers a valid
  request the daemon does not yet implement — never panic or silent drop.

## Constraints

- This crate carries only typed wire vocabulary, NOTA codecs, and
  round-trip witnesses. No actors, sockets, tokio, or storage.
- The frame-layer dependency moves from `signal-core` to `signal-frame`
  as the migration lands.
- Contract types derive NOTA in this crate; clients do not carry shadow
  types.
- Every operation, reply, and event variant round-trips through both
  rkyv frames and NOTA text.

## Non-ownership

This crate does not own:

- the `harness` daemon runtime, actor topology, redb tables, or
  transcript storage;
- routing policy or delivery state — those live in `router`;
- terminal byte transport — raw transcript bytes flow outside this
  control contract.

## See also

- `ARCHITECTURE.md` — detailed channel shape, the subscription
  lifecycle, and the three-layer migration plan.
- `../harness/INTENT.md` — daemon-side intent when it lands.
- `signal-message` — the upstream channel that drives these deliveries.
- `signal-terminal` — the terminal control channel.
- `primary/skills/contract-repo.md` — contract repo discipline and
  naming rules.
- `primary/skills/component-triad.md` — repo triad structure and wire
  layers.
