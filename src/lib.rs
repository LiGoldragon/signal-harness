//! Signal contract — `persona-router` ↔ `persona-harness`.
//!
//! Read this file as the public interface of the
//! delivery channel between the routing actor and the
//! harness actors. The channel carries:
//!
//! - **Delivery requests** from the router to a harness:
//!   "deliver this typed payload (a message, a system
//!   notification, a prompt) through this harness's terminal
//!   delivery path."
//! - **Harness observations** from the harness back to the
//!   router: lifecycle events (started / stopped /
//!   crashed), input acknowledgements, interaction
//!   resolutions.
//!
//! The channel is **bidirectional**: both sides initiate.
//! The router sends `MessageDelivery` / `InteractionPrompt`
//! / `DeliveryCancellation` requests; the harness pushes
//! lifecycle and resolution events independent of any request.
//!
//! See `ARCHITECTURE.md` for the channel's role and
//! boundaries; `~/primary/reports/designer/72-harmonized-implementation-plan.md`
//! §6 for the contract-creation discipline.

use nota_codec::{NotaEnum, NotaRecord, NotaTransparent};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_core::{SemaVerb, signal_channel};

// ─── Harness identity ─────────────────────────────────────

/// A typed name for one harness instance. Multiple
/// harnesses on one machine each have their own
/// `HarnessName`; the router routes by name.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent, Debug, Clone, PartialEq, Eq, Hash,
)]
pub struct HarnessName(String);

impl HarnessName {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent, Debug, Clone, PartialEq, Eq, Hash,
)]
pub struct MessageSender(String);

impl MessageSender {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent, Debug, Clone, PartialEq, Eq, Hash,
)]
pub struct MessageBody(String);

impl MessageBody {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaTransparent,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub struct MessageSlot(u64);

impl MessageSlot {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn into_u64(self) -> u64 {
        self.0
    }
}

// ─── Delivery requests (router → harness) ─────────────────

/// Deliver a message through the harness's terminal path.
/// This request does not certify prompt cleanliness. The
/// harness / terminal adapter must acquire the terminal input
/// gate before programmatic injection.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct MessageDelivery {
    pub harness: HarnessName,
    pub sender: MessageSender,
    pub body: MessageBody,
    /// The router-minted durable message slot so the
    /// harness can reference the message in subsequent
    /// observations (e.g. "delivered slot N").
    pub message_slot: MessageSlot,
}

/// Surface an interaction (a typed prompt awaiting human
/// input) in the harness — used for authorization decisions
/// and any place the system needs human confirmation. The
/// harness shows the prompt; the human's response comes
/// back via `InteractionResolved` event.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct InteractionPrompt {
    pub harness: HarnessName,
    pub interaction_id: String,
    pub prompt: String,
    pub options: Vec<String>,
}

/// Cancel a pending delivery (e.g. the recipient went
/// offline before delivery completed, or the router is
/// shutting down).
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct DeliveryCancellation {
    pub harness: HarnessName,
    pub message_slot: MessageSlot,
}

/// Ask the harness daemon for its current minimal readiness facts.
///
/// This is intentionally small. Detailed lifecycle and transcript history are
/// harness-owned state, but a supervised engine needs one cheap typed probe
/// before it treats the daemon as started.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct HarnessStatusQuery {
    pub harness: HarnessName,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum HarnessOperationKind {
    MessageDelivery,
    InteractionPrompt,
    DeliveryCancellation,
    HarnessStatusQuery,
}

// ─── Delivery acknowledgements (harness → router) ─────────

/// The harness successfully delivered the message — the
/// bytes hit the input surface. The router can mark the
/// message as delivered in its store.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct DeliveryCompleted {
    pub harness: HarnessName,
    pub message_slot: MessageSlot,
}

/// Delivery failed — typed reason carried.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct DeliveryFailed {
    pub harness: HarnessName,
    pub message_slot: MessageSlot,
    pub reason: DeliveryFailureReason,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, PartialEq, Eq)]
pub enum DeliveryFailureReason {
    /// The harness's transport (PTY, terminal) couldn't
    /// accept the bytes.
    TransportRejected,
    /// The terminal input gate observed human input before
    /// programmatic injection. The harness aborted to preserve
    /// the human's draft.
    HumanInputIntervened,
    /// The harness was tearing down when the delivery
    /// arrived.
    HarnessStoppedBeforeDelivery,
}

/// Human resolved a previously-surfaced interaction — they
/// picked one of the options.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct InteractionResolved {
    pub harness: HarnessName,
    pub interaction_id: String,
    pub chosen: String,
}

/// A valid request reached a harness daemon, but the daemon's current runtime
/// does not implement the operation yet.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct HarnessRequestUnimplemented {
    pub harness: HarnessName,
    pub operation: HarnessOperationKind,
    pub reason: HarnessUnimplementedReason,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum HarnessUnimplementedReason {
    NotBuiltYet,
    DependencyTrackNotLanded,
}

/// Minimal health surface for the daemon skeleton and supervisor witness.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct HarnessStatus {
    pub harness: HarnessName,
    pub health: HarnessHealth,
    pub readiness: HarnessReadiness,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum HarnessHealth {
    Running,
    Degraded,
    Stopped,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum HarnessReadiness {
    Ready,
    Starting,
    Unavailable,
}

// ─── Lifecycle observations (harness → router) ────────────

/// Harness started; ready to receive deliveries.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct HarnessStarted {
    pub harness: HarnessName,
}

/// Harness shut down cleanly. The router stops sending
/// deliveries to this harness.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct HarnessStopped {
    pub harness: HarnessName,
}

/// Harness crashed / died unexpectedly. The router needs
/// to retry or escalate.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct HarnessCrashed {
    pub harness: HarnessName,
    pub detail: String,
}

// ─── Channel declaration ───────────────────────────────────

signal_channel! {
    request HarnessRequest {
        MessageDelivery(MessageDelivery),
        InteractionPrompt(InteractionPrompt),
        DeliveryCancellation(DeliveryCancellation),
        HarnessStatusQuery(HarnessStatusQuery),
    }
    reply HarnessEvent {
        DeliveryCompleted(DeliveryCompleted),
        DeliveryFailed(DeliveryFailed),
        InteractionResolved(InteractionResolved),
        HarnessRequestUnimplemented(HarnessRequestUnimplemented),
        HarnessStatus(HarnessStatus),
        HarnessStarted(HarnessStarted),
        HarnessStopped(HarnessStopped),
        HarnessCrashed(HarnessCrashed),
    }
}

impl HarnessRequest {
    pub const fn signal_verb(&self) -> SemaVerb {
        match self {
            Self::MessageDelivery(_) | Self::InteractionPrompt(_) => SemaVerb::Assert,
            Self::DeliveryCancellation(_) => SemaVerb::Retract,
            Self::HarnessStatusQuery(_) => SemaVerb::Match,
        }
    }

    pub fn operation_kind(&self) -> HarnessOperationKind {
        match self {
            Self::MessageDelivery(_) => HarnessOperationKind::MessageDelivery,
            Self::InteractionPrompt(_) => HarnessOperationKind::InteractionPrompt,
            Self::DeliveryCancellation(_) => HarnessOperationKind::DeliveryCancellation,
            Self::HarnessStatusQuery(_) => HarnessOperationKind::HarnessStatusQuery,
        }
    }
}
