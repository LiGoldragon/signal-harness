//! Signal contract — `persona-router` ↔ `persona-harness`.
//!
//! Read this file as the public interface of the
//! delivery channel between the routing actor and the
//! harness actors. The channel carries:
//!
//! - **Delivery requests** from the router to a harness:
//!   "deliver this typed payload (a message, a system
//!   notification, a prompt) to the human inhabiting this
//!   harness."
//! - **Harness observations** from the harness back to the
//!   router: lifecycle events (started / stopped /
//!   crashed), input acknowledgements, interaction
//!   resolutions.
//!
//! The channel is **bidirectional**: both sides initiate.
//! The router sends `Deliver*` requests; the harness
//! pushes `Harness*` events independent of any request.
//!
//! See `ARCHITECTURE.md` for the channel's role and
//! boundaries; `~/primary/reports/designer/72-harmonized-implementation-plan.md`
//! §6 for the contract-creation discipline.

use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_core::signal_channel;

// ─── Harness identity ─────────────────────────────────────

/// A typed name for one harness instance. Multiple
/// harnesses on one machine each have their own
/// `HarnessName`; the router routes by name.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct HarnessName(String);

impl HarnessName {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// ─── Delivery requests (router → harness) ─────────────────

/// Deliver a message to the harness's input surface. The
/// router has already verified the safety property (focus
/// not human-owned + input buffer empty); the harness
/// performs the actual injection.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct DeliverMessage {
    pub harness: HarnessName,
    pub sender: String,
    pub body: String,
    /// The router-minted slot from `persona-sema` so the
    /// harness can reference the message in subsequent
    /// observations (e.g. "delivered slot N").
    pub message_slot: u64,
}

/// Surface an interaction (a typed prompt awaiting human
/// input) in the harness — used for authorization decisions
/// and any place the system needs human confirmation. The
/// harness shows the prompt; the human's response comes
/// back via `InteractionResolution` event.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct SurfaceInteraction {
    pub harness: HarnessName,
    pub interaction_id: String,
    pub prompt: String,
    pub options: Vec<String>,
}

/// Cancel a pending delivery (e.g. the recipient went
/// offline before delivery completed, or the router is
/// shutting down).
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct CancelDelivery {
    pub harness: HarnessName,
    pub message_slot: u64,
}

// ─── Delivery acknowledgements (harness → router) ─────────

/// The harness successfully delivered the message — the
/// bytes hit the input surface. The router can mark the
/// message as delivered in its store.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct DeliveryCompleted {
    pub harness: HarnessName,
    pub message_slot: u64,
}

/// Delivery failed — typed reason carried.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct DeliveryFailed {
    pub harness: HarnessName,
    pub message_slot: u64,
    pub reason: DeliveryFailureReason,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum DeliveryFailureReason {
    /// The harness's transport (PTY, terminal) couldn't
    /// accept the bytes.
    TransportRejected,
    /// The human typed into the input buffer between the
    /// router's safety check and the harness's injection.
    /// The harness aborted to preserve the human's draft.
    HumanRaceLost,
    /// The harness was tearing down when the delivery
    /// arrived.
    HarnessTeardown,
}

/// Human resolved a previously-surfaced interaction — they
/// picked one of the options.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct InteractionResolved {
    pub harness: HarnessName,
    pub interaction_id: String,
    pub chosen: String,
}

// ─── Lifecycle observations (harness → router) ────────────

/// Harness started; ready to receive deliveries.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct HarnessStarted {
    pub harness: HarnessName,
}

/// Harness shut down cleanly. The router stops sending
/// deliveries to this harness.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct HarnessStopped {
    pub harness: HarnessName,
}

/// Harness crashed / died unexpectedly. The router needs
/// to retry or escalate.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct HarnessCrashed {
    pub harness: HarnessName,
    pub detail: String,
}

// ─── Channel declaration ───────────────────────────────────

signal_channel! {
    request HarnessRequest {
        DeliverMessage(DeliverMessage),
        SurfaceInteraction(SurfaceInteraction),
        CancelDelivery(CancelDelivery),
    }
    reply HarnessEvent {
        DeliveryCompleted(DeliveryCompleted),
        DeliveryFailed(DeliveryFailed),
        InteractionResolved(InteractionResolved),
        HarnessStarted(HarnessStarted),
        HarnessStopped(HarnessStopped),
        HarnessCrashed(HarnessCrashed),
    }
}
