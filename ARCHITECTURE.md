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
                 `InteractionPrompt` / `DeliveryCancellation` /
                 `HarnessStatusQuery`) |
| Event side | `persona-harness` (pushes
                 `Delivery*` acks + interaction resolutions
                 + skeleton honesty + lifecycle events) |

Bidirectional steady-state: router sends one request;
harness emits one or more events. Lifecycle events
(HarnessStarted / HarnessStopped / HarnessCrashed) flow
without paired requests.

## Record source

Records local to this contract:
- `HarnessName` (local until another concrete relation needs a matching
  contract)
- `MessageDelivery`, `InteractionPrompt`, `DeliveryCancellation`,
  `HarnessStatusQuery`
- `DeliveryCompleted`, `DeliveryFailed`,
  `DeliveryFailureReason`
- `InteractionResolved`
- `HarnessRequestUnimplemented`, `HarnessUnimplementedReason`
- `HarnessStatus`, `HarnessHealth`, `HarnessReadiness`
- `HarnessStarted`, `HarnessStopped`, `HarnessCrashed`

The `MessageBody` on `MessageDelivery` is provisional. The destination
is a typed Nexus record written in NOTA syntax, not a new text format.

## Recipient → harness → terminal resolution mapping

The prototype-one resolution chain is:

```text
MessageRecipient (role name, e.g. "designer")
  → HarnessName (same role-named harness from harness registry)
  → TerminalName (same role-named terminal session, per
                  signal-persona-terminal's TerminalName namespace)
  → terminal-cell session (the cell bound to the role-named terminal)
```

**One harness per role for prototype one.** The harness registry
maps `MessageRecipient` → `HarnessName` by string equality at the
role-name level. The `HarnessName` and `TerminalName` namespaces
**align**: a harness named `"designer"` writes into the terminal
session named `"designer"`. Future cases (multiple harnesses per
role, harness pools, separate identity/transport namespaces) get a
richer resolution when they surface.

The constraint witness:

```text
recipient_resolves_to_role_named_harness_and_terminal
  — assert MessageRecipient::new("designer") routes through
    HarnessName::new("designer") which writes to
    TerminalName::new("designer"). The three names match exactly.
```

## Messages

```
HarnessRequest                   HarnessEvent
├─ MessageDelivery               ├─ DeliveryCompleted
├─ InteractionPrompt             ├─ DeliveryFailed { reason }
├─ DeliveryCancellation          ├─ InteractionResolved
└─ HarnessStatusQuery            ├─ HarnessRequestUnimplemented
                                 ├─ HarnessStatus
                                 ├─ HarnessStarted
                                 ├─ HarnessStopped
                                 └─ HarnessCrashed
```

Closed enums; typed `DeliveryFailureReason` (3 variants:
`TransportRejected`, `HumanInputIntervened`,
`HarnessStoppedBeforeDelivery`). `HarnessOperationKind` is the closed
request discriminator used by skeleton honesty events.

### Signal root verbs

Every `HarnessRequest` variant declares its root verb in the
`signal_channel!` declaration. `signal-core` generates
`HarnessRequest::signal_verb()` and `HarnessRequest::into_request()`
from that declaration.

```text
MessageDelivery      -> Assert
InteractionPrompt    -> Assert
DeliveryCancellation -> Retract
HarnessStatusQuery   -> Match
```

Delivery and interaction prompts assert new harness work. Cancellation
retracts pending work. Status is a read and must not be wrapped as
`Assert`.

## Constraints

- A harness skeleton can answer `HarnessStatusQuery` with typed health and
  readiness.
- A valid request that reaches a skeleton harness daemon but is not implemented
  yet returns `HarnessRequestUnimplemented`.
- `HarnessRequestUnimplemented.operation` is a closed `HarnessOperationKind`,
  not a string.
- Skeleton honesty uses `HarnessUnimplementedReason`, not free text.
- Prompt cleanliness and input gates stay below this contract in
  `signal-persona-terminal`.

## Versioning

`signal_core::Frame` carries the protocol version.
Schema-level changes are breaking; coordinate
`persona-router` + `persona-harness` upgrades.

## Examples

```text
;; router → harness: deliver a routed message
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

;; harness → router: terminal input gate saw human intervention;
;; delivery aborted to preserve the draft
HarnessEvent::DeliveryFailed(DeliveryFailed {
    harness: HarnessName::new("designer"),
    message_slot: MessageSlot::new(1024),
    reason: DeliveryFailureReason::HumanInputIntervened,
})
```

## Round trips

Round-trip tests in `tests/round_trip.rs` cover every request/event variant,
the operation-kind and failure-reason enums, From-impl witnesses, and
representative NOTA text witnesses for `MessageDelivery`, `DeliveryFailed`, and
`HarnessRequestUnimplemented`.
Request frame tests assert each variant's `signal_verb()` mapping.

## Non-ownership

- No router daemon — that's `persona-router`.
- No harness daemon — that's `persona-harness`.
- No PTY adapter or terminal transport — that's `persona-terminal`, below
  the `signal-persona-terminal` contract.
- No terminal prompt cleanliness or input-gate enforcement. Those are
  `signal-persona-terminal`, `persona-terminal`, and `terminal-cell`
  concerns.
- No transport.

## Code map

```
src/
└── lib.rs    — payloads + signal_channel! invocation
tests/
└── round_trip.rs — per-variant frame round trips + NOTA text witnesses
```

## See also

- `signal-core/src/channel.rs` — the macro
- `signal-persona-message` — upstream channel producing
  the messages this channel delivers
- `signal-persona-terminal` — terminal contract for harness/terminal PTY
  coordination; downstream from this channel
