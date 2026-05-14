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
    Frame, HarnessName, HarnessRequest, MessageBody, MessageDelivery,
    MessageSender, MessageSlot,
};
use signal_core::FrameBody;

let request = HarnessRequest::MessageDelivery(MessageDelivery {
    harness: HarnessName::new("designer"),
    sender: MessageSender::new("operator"),
    body: MessageBody::new("delivery test"),
    message_slot: MessageSlot::new(1024),
});
let frame = Frame::new(FrameBody::Request(request.into_signal_request()));
let bytes = frame.encode_length_prefixed()?;
// router sends bytes to designer harness's UDS
```

The harness pushes `HarnessEvent::DeliveryCompleted` (or
`DeliveryFailed`) back over the same channel.

Delivery and interaction prompts use `Assert`; delivery cancellation uses
`Retract`; status reads use `Match`.

## See also

- `ARCHITECTURE.md` — channel role + boundaries
- `~/primary/skills/contract-repo.md` — contract-repo discipline
- `signal-persona-message` — upstream channel that drives
  these deliveries
- `signal-persona-terminal` — terminal control channel carrying
  prompt patterns, input gates, and write-injection acknowledgements
