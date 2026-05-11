# ARCHITECTURE — signal-persona-harness

The Signal contract between `persona-router` and
`persona-harness` — bidirectional. It relates one router delivery
owner to one or more harnesses: the router requests delivery,
interaction, and cancellation vectors; harnesses push delivery and
lifecycle facts. The whole channel is one `signal_channel!`
invocation in `src/lib.rs`.

## Channel

| Side | Component |
|---|---|
| Request side | `persona-router` (sends `MessageDelivery` /
                 `InteractionPrompt` / `DeliveryCancellation`) |
| Event side | `persona-harness` (pushes
                 `Delivery*` acks + interaction resolutions
                 + lifecycle events) |

Bidirectional steady-state: router sends one request;
harness emits one or more events. Lifecycle events
(HarnessStarted / HarnessStopped / HarnessCrashed) flow
without paired requests.

## Record source

Records local to this contract:
- `HarnessName` (local until another concrete relation needs a matching
  contract)
- `MessageDelivery`, `InteractionPrompt`, `DeliveryCancellation`
- `DeliveryCompleted`, `DeliveryFailed`,
  `DeliveryFailureReason`
- `InteractionResolved`
- `HarnessStarted`, `HarnessStopped`, `HarnessCrashed`

The `MessageBody` on `MessageDelivery` is provisional. The destination is
a typed Nexus record written in NOTA syntax (per operator/77 §7 +
`primary-kxb` #3), not a new text format.

## Messages

```
HarnessRequest                   HarnessEvent
├─ MessageDelivery               ├─ DeliveryCompleted
├─ InteractionPrompt             ├─ DeliveryFailed { reason }
└─ DeliveryCancellation          ├─ InteractionResolved
                                 ├─ HarnessStarted
                                 ├─ HarnessStopped
                                 └─ HarnessCrashed
```

Closed enums; typed `DeliveryFailureReason` (3 variants:
`TransportRejected`, `HumanInputIntervened`,
`HarnessStoppedBeforeDelivery`).

## Versioning

`signal_core::Frame` carries the protocol version.
Schema-level changes are breaking; coordinate
`persona-router` + `persona-harness` upgrades.

## Examples

```text
;; router → harness: deliver a message after the safety gate cleared
HarnessRequest::MessageDelivery(MessageDelivery {
    harness: HarnessName::new("designer"),
    sender: MessageSender::new("operator"),
    body: MessageBody::new("stack test 2026-05-09"),
    message_slot: MessageSlot::new(1024),
})

;; harness → router: delivery succeeded
HarnessEvent::DeliveryCompleted(DeliveryCompleted {
    harness: HarnessName::new("designer"),
    message_slot: MessageSlot::new(1024),
})

;; harness → router: human typed during the gate window;
;; we aborted to preserve the draft
HarnessEvent::DeliveryFailed(DeliveryFailed {
    harness: HarnessName::new("designer"),
    message_slot: MessageSlot::new(1024),
    reason: DeliveryFailureReason::HumanInputIntervened,
})
```

## Round trips

11 round-trip tests in `tests/round_trip.rs` covering all
9 variants + the failure-reason enum + From-impl witnesses.

## Non-ownership

- No router daemon — that's `persona-router`.
- No harness daemon — that's `persona-harness`.
- No PTY adapter or terminal transport — that's `persona-terminal`, below
  the `signal-persona-terminal` contract.
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
  sending `MessageDelivery`
- `signal-core/src/channel.rs` — the macro
- `signal-persona-message` — upstream channel producing
  the messages this channel delivers
- `signal-persona-system` — companion channel carrying the
  focus/input-buffer facts the router uses to gate
- `signal-persona-terminal` — terminal contract for harness/terminal PTY
  coordination; downstream from this channel
