# ARCHITECTURE — signal-persona-harness

The Signal contract between `persona-router` and
`persona-harness` — bidirectional. The whole channel is one
`signal_channel!` invocation in `src/lib.rs`.

## Channel

| Side | Component |
|---|---|
| Request side | `persona-router` (sends `Deliver*` /
                 `SurfaceInteraction` / `CancelDelivery`) |
| Event side | `persona-harness` (pushes
                 `Delivery*` acks + interaction resolutions
                 + lifecycle events) |

Bidirectional steady-state: router sends one request;
harness emits one or more events. Lifecycle events
(HarnessStarted / HarnessStopped / HarnessCrashed) flow
without paired requests.

## Record source

Records local to this contract:
- `HarnessName` (could lift to umbrella later if other
  channels need it)
- `DeliverMessage`, `SurfaceInteraction`, `CancelDelivery`
- `DeliveryCompleted`, `DeliveryFailed`,
  `DeliveryFailureReason`
- `InteractionResolved`
- `HarnessStarted`, `HarnessStopped`, `HarnessCrashed`

The `body: String` on `DeliverMessage` is provisional —
will become the typed text payload chosen by the
Nexus-in-NOTA path (per operator/77 §7 +
`primary-kxb` #3).

## Messages

```
HarnessRequest                   HarnessEvent
├─ DeliverMessage                ├─ DeliveryCompleted
├─ SurfaceInteraction            ├─ DeliveryFailed { reason }
└─ CancelDelivery                ├─ InteractionResolved
                                 ├─ HarnessStarted
                                 ├─ HarnessStopped
                                 └─ HarnessCrashed
```

Closed enums; typed `DeliveryFailureReason` (3 variants:
`TransportRejected`, `HumanRaceLost`, `HarnessTeardown`).

## Versioning

`signal_core::Frame` carries the protocol version.
Schema-level changes are breaking; coordinate
`persona-router` + `persona-harness` upgrades.

## Examples

```text
;; router → harness: deliver a message after the safety gate cleared
HarnessRequest::DeliverMessage(DeliverMessage {
    harness: HarnessName::new("designer"),
    sender: "operator".to_string(),
    body: "stack test 2026-05-09".to_string(),
    message_slot: 1024,
})

;; harness → router: delivery succeeded
HarnessEvent::DeliveryCompleted(DeliveryCompleted {
    harness: HarnessName::new("designer"),
    message_slot: 1024,
})

;; harness → router: human typed during the gate window;
;; we aborted to preserve the draft
HarnessEvent::DeliveryFailed(DeliveryFailed {
    harness: HarnessName::new("designer"),
    message_slot: 1024,
    reason: DeliveryFailureReason::HumanRaceLost,
})
```

## Round trips

13 round-trip tests in `tests/round_trip.rs` covering all
9 variants + the failure-reason enum + From-impl witnesses.

## Non-ownership

- No router daemon — that's `persona-router`.
- No harness daemon — that's `persona-harness`.
- No PTY adapter — that's `persona-wezterm` (via
  `signal-persona-terminal`, future channel).
- No safety-property enforcement (router-side; gated by
  `signal-persona-system` observations).
- No transport.

## Code map

```
src/
└── lib.rs    — payloads + signal_channel! invocation
tests/
└── round_trip.rs — per-variant wire-form round trips
```

## See also

- `~/primary/reports/designer/72-harmonized-implementation-plan.md`
  §2.1 — channel inventory
- `~/primary/reports/operator/67-signal-actor-messaging-gap-audit.md`
  — the safety property the router enforces before
  sending DeliverMessage
- `signal-core/src/channel.rs` — the macro
- `signal-persona-message` — upstream channel producing
  the messages this channel delivers
- `signal-persona-system` — companion channel carrying the
  focus/input-buffer facts the router uses to gate
- `signal-persona-terminal` (future) — harness ↔ wezterm
  internal-PTY channel; downstream from this one
