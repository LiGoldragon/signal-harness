# INTENT — signal-harness

*The Signal contract between `router` and `harness` —
bidirectional delivery, interaction, and observation. Companion to
`ARCHITECTURE.md` and `Cargo.toml`. Maintenance:
`primary/skills/repo-intent.md`.*

## Repo-scope only

This file carries only the intent that is FOR this `signal-harness`
contract. Workspace-shape intent stays in the primary workspace
`primary/INTENT.md`. Harness daemon intent stays in `harness/INTENT.md`.

## Why this repo exists

`signal-harness` is the **router-to-harness wire contract** — the
delivery channel between `router` and one or more harness
instances. The router asks for delivery, interaction, cancellation,
status, and transcript observation; the harness pushes acknowledgements,
interaction resolutions, status, lifecycle events, and
generic adapter events, and transcript-observation events. Runtime
actors, sockets, storage, and the harness's tables live in `harness`;
routing policy and delivery state live in `router`.

## The channel shape — bidirectional

| Side | Role |
|---|---|
| Request side | `router` sends delivery, interaction-prompt, cancellation, status-query, and transcript-subscription requests. |
| Reply / event side | `harness` emits delivery acks, interaction resolutions, skeleton honesty, status, lifecycle events, generic adapter events, transcript snapshot, retraction ack, and `TranscriptObservation` events on the open stream. |

Bidirectional steady state: the router sends one request; the harness
emits one or more events. Lifecycle events (`HarnessStarted` /
`HarnessStopped` / `HarnessCrashed`) and provider-neutral adapter
events (`AdapterReady`, `AdapterInputAccepted`, `AdapterOutput`,
`AdapterProgress`, `AdapterCompletion`,
`AdapterConfirmationNeeded`, `AdapterStalled`, `AdapterExited`) flow
without paired requests.
Transcript observation is push-based: the router watches once per
harness, the harness emits a snapshot then `TranscriptObservation`
deltas, and the subscription closes via the canonical
watch/unwatch stream lifecycle.

The generic adapter vocabulary describes launch/send/observe/ready/done
event responsibilities without provider-specific terms. Concrete
adapters own their provider's TUI behavior: how to launch, how to send
input, how to observe readiness and output, how to detect prompt-turn
completion, how to surface confirmation prompts, how to classify
stalls, and how to observe exit. `AdapterCompletion` is a done event for
one prompt turn, not a session close. Sessions stay open until runtime
exit or an explicit close-if-asked path later asks an adapter to close.
Confirmation prompts are first-class events; policy decides whether an
operator, automation rule, or escalation path answers them.

## Wire vocabulary discipline — three-layer direction

Per `primary/skills/contract-repo.md` §"Public contracts use
contract-local operation verbs" and `primary/skills/component-triad.md`
§"Verbs come in three layers", the implemented shape is:

- **Layer 1 (this crate):** contract-local operation roots over
  `signal-frame`. `MessageDelivery`, `InteractionPrompt`,
  `DeliveryCancellation`, `HarnessStatusQuery`,
  `WatchHarnessTranscript`, and `UnwatchHarnessTranscript` are the wire
  heads. No public operation carries `SignalVerb`, `Assert`, `Match`,
  `Subscribe`, or `Retract` as a root classification.
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
sit alongside that standardized observability; this crate currently has
the domain transcript stream and the observer-hook surface remains the
next contract addition.

## Channels are closed, boundaries are named

- Wire enums are closed. No `Unknown` escape hatch.
- Subscription close is request-side `UnwatchHarnessTranscript`
  carrying the per-stream
  token; the harness echoes the token and ends the stream after the
  final ack — the kernel grammar enforces the declared close operation.
- A skeleton honesty reply (`RequestUnimplemented`) answers a valid
  request the daemon does not yet implement — never panic or silent drop.

## Constraints

- This crate carries only typed wire vocabulary, explicit NOTA text
  codecs for CLI/tooling projection, and round-trip witnesses. No actors,
  sockets, tokio, or storage.
- The frame-layer dependency is `signal-frame`; `signal-core` is not a
  contract dependency.
- Contract types derive NOTA in this crate; clients do not carry shadow
  types.
- Every operation, reply, and event variant round-trips through both
  rkyv frames and NOTA text.
- `HarnessDaemonConfiguration` is the single typed startup contract
  for one `harness-daemon` component process. The contract still exposes
  NOTA round trips for authoring and tooling, but a live daemon consumes
  this record only as a signal-encoded/rkyv startup file; inline NOTA and
  `.nota` files are CLI/deploy-tool inputs, not daemon inputs. It carries
  daemon socket and supervision fields plus a `harnesses` list; each
  `HarnessInstanceConfiguration` carries the instance name, closed
  `HarnessKind`, optional terminal endpoint, and optional adapter-specific
  startup data such as the Pi RPC/JSONL adapter command, session
  directory, model selector, and delivery mode. Per-harness boundaries
  are internal records/actors/adapters, not separate daemon processes
  unless deployment later requires process isolation.

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
