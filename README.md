# signal-persona-harness

The Signal contract between **`persona-router`** and
**`persona-harness`** — bidirectional. The router sends
delivery requests; the harness pushes lifecycle + delivery
events back.

Read `src/lib.rs` for the public interface — two enums
(`HarnessRequest`, `HarnessEvent`) declared via the
`signal_channel!` macro.

## Quick reference

```rust
use signal_persona_harness::{
    DeliverMessage, Frame, HarnessName, HarnessRequest,
};
use signal_core::{FrameBody, Request};

let request = HarnessRequest::DeliverMessage(DeliverMessage {
    harness: HarnessName::new("designer"),
    sender: "operator".into(),
    body: "delivery test".into(),
    message_slot: 1024,
});
let frame = Frame::new(FrameBody::Request(Request::assert(request)));
let bytes = frame.encode_length_prefixed()?;
// router sends bytes to designer harness's UDS
```

The harness pushes `HarnessEvent::DeliveryCompleted` (or
`DeliveryFailed`) back over the same channel.

## See also

- `ARCHITECTURE.md` — channel role + boundaries
- `~/primary/reports/designer/72-harmonized-implementation-plan.md`
  §2.1 — channel inventory
- `~/primary/reports/operator/67-signal-actor-messaging-gap-audit.md`
  — the safety property + delivery state machine
- `~/primary/skills/contract-repo.md` — contract-repo discipline
- `signal-persona-message` — upstream channel that drives
  these deliveries
- `signal-persona-system` — companion channel carrying the
  focus + input-buffer facts the router uses to gate
  deliveries
