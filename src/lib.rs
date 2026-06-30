//! Signal contract — `router` ↔ `harness`.
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

use nota::{NotaDecode, NotaEncode};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_frame::signal_channel;
use signal_persona::{
    DomainSocketMode, DomainSocketPath, EngineManagementSocketMode, EngineManagementSocketPath,
    OwnerIdentity,
};

// ─── Harness identity ─────────────────────────────────────

/// A typed name for one harness instance. Multiple
/// harnesses on one machine each have their own
/// `HarnessName`; the router routes by name.
#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
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
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
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
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
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
    NotaEncode,
    NotaDecode,
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
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
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
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct InteractionPrompt {
    pub harness: HarnessName,
    pub interaction_id: String,
    pub prompt: String,
    pub options: Vec<String>,
}

/// Cancel a pending delivery (e.g. the recipient went
/// offline before delivery completed, or the router is
/// shutting down).
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct DeliveryCancellation {
    pub harness: HarnessName,
    pub message_slot: MessageSlot,
}

/// Ask the harness daemon for its current minimal readiness facts.
///
/// This is intentionally small. Detailed lifecycle and transcript history are
/// harness-owned state, but a supervised engine needs one cheap typed probe
/// before it treats the daemon as started.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct HarnessStatusQuery {
    pub harness: HarnessName,
}

// ─── Delivery acknowledgements (harness → router) ─────────

/// The harness successfully delivered the message — the
/// bytes hit the input surface. The router can mark the
/// message as delivered in its store.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct DeliveryCompleted {
    pub harness: HarnessName,
    pub message_slot: MessageSlot,
}

/// Delivery failed — typed reason carried.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct DeliveryFailed {
    pub harness: HarnessName,
    pub message_slot: MessageSlot,
    pub reason: DeliveryFailureReason,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
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
    /// The daemon that received the request does not serve
    /// the named harness instance.
    HarnessUnavailable,
}

/// Human resolved a previously-surfaced interaction — they
/// picked one of the options.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct InteractionResolved {
    pub harness: HarnessName,
    pub interaction_id: String,
    pub chosen: String,
}

/// A valid request reached a harness daemon, but the daemon's current runtime
/// does not implement the operation yet.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "nota-text", derive(NotaEncode, NotaDecode))]
pub struct HarnessRequestUnimplemented {
    pub harness: HarnessName,
    pub operation: HarnessOperationKind,
    pub reason: HarnessUnimplementedReason,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub enum HarnessUnimplementedReason {
    NotBuiltYet,
    DependencyTrackNotLanded,
}

/// Minimal health surface for the daemon skeleton and supervisor witness.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct HarnessStatus {
    pub harness: HarnessName,
    pub health: HarnessHealth,
    pub readiness: HarnessReadiness,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub enum HarnessHealth {
    Running,
    Degraded,
    Stopped,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub enum HarnessReadiness {
    Ready,
    Starting,
    Unavailable,
}

// ─── Lifecycle observations (harness → router) ────────────

/// Harness started; ready to receive deliveries.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct HarnessStarted {
    pub harness: HarnessName,
}

/// Harness shut down cleanly. The router stops sending
/// deliveries to this harness.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct HarnessStopped {
    pub harness: HarnessName,
}

/// Harness crashed / died unexpectedly. The router needs
/// to retry or escalate.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct HarnessCrashed {
    pub harness: HarnessName,
    pub detail: String,
}

// ─── Adapter observations (harness → router) ──────────────

/// Per-adapter observation sequence pointer. Monotonic per harness
/// adapter session. Transcript observation has its own sequence because
/// transcript lines and adapter-state events are separate streams.
#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub struct AdapterEventSequence(u64);

impl AdapterEventSequence {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn into_u64(self) -> u64 {
        self.0
    }
}

/// The adapter has observed enough provider/runtime state to accept
/// routed input. This is distinct from process launch success.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct AdapterReady {
    pub harness: HarnessName,
    pub sequence: AdapterEventSequence,
}

/// The adapter accepted one routed input into its provider-specific
/// surface. The input may still produce later output, progress,
/// confirmation, completion, stalled, or exit events.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct AdapterInputAccepted {
    pub harness: HarnessName,
    pub sequence: AdapterEventSequence,
    pub message_slot: MessageSlot,
}

/// Provider-visible output observed by the adapter. Transcript storage
/// may also publish `TranscriptObservation`; this event reports the
/// adapter-level interpretation that output happened.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct AdapterOutput {
    pub harness: HarnessName,
    pub sequence: AdapterEventSequence,
    pub text: String,
}

/// Provider-neutral progress while a prompt turn is still in flight.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct AdapterProgress {
    pub harness: HarnessName,
    pub sequence: AdapterEventSequence,
    pub status: String,
}

/// The adapter observed that one prompt turn completed. This is not a
/// request to close the harness session; long-lived TUI sessions remain
/// open until an explicit close path asks for shutdown.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct AdapterCompletion {
    pub harness: HarnessName,
    pub sequence: AdapterEventSequence,
    pub message_slot: MessageSlot,
}

/// The adapter observed a provider-neutral confirmation prompt.
/// Policy decides whether an operator, automation rule, or later
/// escalation path answers it.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct AdapterConfirmationNeeded {
    pub harness: HarnessName,
    pub sequence: AdapterEventSequence,
    pub interaction_id: String,
    pub prompt: String,
    pub options: Vec<String>,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub enum AdapterStallReason {
    NoOutput,
    ReadinessTimeout,
    CompletionTimeout,
    TransportBackpressure,
}

/// The adapter did not observe the next expected provider-neutral state
/// transition within its local policy window.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct AdapterStalled {
    pub harness: HarnessName,
    pub sequence: AdapterEventSequence,
    pub reason: AdapterStallReason,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub enum AdapterExitStatus {
    Success,
    Failure,
}

/// The adapter observed that the provider process or session exited.
/// Runtime transport failures are still reported through typed delivery
/// failures when they affect a specific routed input.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct AdapterExited {
    pub harness: HarnessName,
    pub sequence: AdapterEventSequence,
    pub status: AdapterExitStatus,
}

// ─── Transcript observation stream (harness → router) ─────

/// Per-observation sequence pointer. Monotonic per harness, starting at
/// `1` for the first transcript line published after subscription. The
/// sequence pointer is the typed witness an observer uses to detect gaps,
/// re-anchor after reconnection, and order events causally — replacing
/// the implicit `transcript_event_count` field formerly carried only on
/// the harness actor's local state.
#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub struct HarnessTranscriptSequence(u64);

impl HarnessTranscriptSequence {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn into_u64(self) -> u64 {
        self.0
    }
}

/// Per-open transcript-observation subscription sequence. This is
/// daemon-minted and unique among the currently open subscriptions for a
/// harness daemon process.
#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub struct HarnessTranscriptSubscriptionIdentifier(u64);

impl HarnessTranscriptSubscriptionIdentifier {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn into_u64(self) -> u64 {
        self.0
    }
}

/// Per-subscription identity for the harness transcript-observation
/// stream. Multiple observers may watch the same harness at the same
/// time; the token names both the harness and the daemon-minted open
/// subscription.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct HarnessTranscriptToken {
    pub harness: HarnessName,
    pub subscription: HarnessTranscriptSubscriptionIdentifier,
}

/// Watch the harness's transcript-observation stream. The reply is a
/// `HarnessTranscriptSnapshot` carrying the daemon-minted subscription
/// token and current sequence pointer; subsequent `TranscriptObservation`
/// events arrive on the same connection as the stream pushes them.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct WatchHarnessTranscript {
    pub harness: HarnessName,
}

/// Acknowledgement that a transcript-observation subscription opened.
/// Carries the token needed to unwatch this exact open subscription and
/// the current sequence pointer so the subscriber knows the starting
/// position; the next `TranscriptObservation` carries sequence
/// `current_sequence + 1`.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct HarnessTranscriptSnapshot {
    pub token: HarnessTranscriptToken,
    pub current_sequence: HarnessTranscriptSequence,
}

/// Typed acknowledgement that a transcript-observation subscription has
/// been closed. Returned in reply to `UnwatchHarnessTranscript`.
/// Carries the retracted token so callers can match the ack to the
/// request they sent.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct HarnessSubscriptionRetracted {
    pub token: HarnessTranscriptToken,
}

/// One transcript line, pushed as it becomes visible to the harness.
/// Carries the sequence pointer so the subscriber can detect gaps and
/// order events causally. Bytes are typed as `String` for the prototype;
/// the eventual shape carries typed Nexus records, not raw text.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct TranscriptObservation {
    pub harness: HarnessName,
    pub sequence: HarnessTranscriptSequence,
    pub line: String,
}

// ─── Channel declaration ───────────────────────────────────

signal_channel! {
    channel Harness {
        operation MessageDelivery(MessageDelivery),
        operation InteractionPrompt(InteractionPrompt),
        operation DeliveryCancellation(DeliveryCancellation),
        operation HarnessStatusQuery(HarnessStatusQuery),
        operation WatchHarnessTranscript(WatchHarnessTranscript) opens HarnessTranscriptStream,
        operation UnwatchHarnessTranscript(HarnessTranscriptToken),
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
        AdapterReady(AdapterReady),
        AdapterInputAccepted(AdapterInputAccepted),
        AdapterOutput(AdapterOutput),
        AdapterProgress(AdapterProgress),
        AdapterCompletion(AdapterCompletion),
        AdapterConfirmationNeeded(AdapterConfirmationNeeded),
        AdapterStalled(AdapterStalled),
        AdapterExited(AdapterExited),
        HarnessTranscriptSnapshot(HarnessTranscriptSnapshot),
        HarnessSubscriptionRetracted(HarnessSubscriptionRetracted),
    }
    event HarnessStreamEvent {
        TranscriptObservation(TranscriptObservation) belongs HarnessTranscriptStream,
    }
    stream HarnessTranscriptStream {
        token HarnessTranscriptToken;
        opened HarnessTranscriptSnapshot;
        event TranscriptObservation;
        close UnwatchHarnessTranscript;
    }
}

pub type HarnessRequest = Operation;
pub type HarnessFrame = Frame;
pub type HarnessFrameBody = FrameBody;
pub type HarnessReplyEnvelope = ReplyEnvelope;
pub type HarnessRequestBuilder = RequestBuilder;
pub type HarnessOperationKind = OperationKind;
pub type HarnessStreamKind = StreamKind;

impl HarnessRequest {
    pub fn operation_kind(&self) -> HarnessOperationKind {
        self.kind()
    }
}

impl From<MessageDelivery> for HarnessRequest {
    fn from(p: MessageDelivery) -> Self {
        Self::MessageDelivery(p)
    }
}
impl From<InteractionPrompt> for HarnessRequest {
    fn from(p: InteractionPrompt) -> Self {
        Self::InteractionPrompt(p)
    }
}
impl From<DeliveryCancellation> for HarnessRequest {
    fn from(p: DeliveryCancellation) -> Self {
        Self::DeliveryCancellation(p)
    }
}
impl From<HarnessStatusQuery> for HarnessRequest {
    fn from(p: HarnessStatusQuery) -> Self {
        Self::HarnessStatusQuery(p)
    }
}
impl From<WatchHarnessTranscript> for HarnessRequest {
    fn from(p: WatchHarnessTranscript) -> Self {
        Self::WatchHarnessTranscript(p)
    }
}
impl From<HarnessTranscriptToken> for HarnessRequest {
    fn from(p: HarnessTranscriptToken) -> Self {
        Self::UnwatchHarnessTranscript(p)
    }
}

// And for event variants on the stream.
impl From<TranscriptObservation> for HarnessStreamEvent {
    fn from(p: TranscriptObservation) -> Self {
        Self::TranscriptObservation(p)
    }
}

// ─── Daemon configuration ──────────────────────────────────
//
// Typed startup configuration for `harness-daemon`. Deploy/bootstrap tooling
// may author or validate it through the NOTA projection, but the live daemon
// accepts only the rkyv/signal-encoded file path on argv and never decodes
// NOTA startup text.

/// Terminal socket endpoint delegated to a harness instance.
#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
)]
pub struct TerminalSocketPath(String);

impl TerminalSocketPath {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Executable path for the external Pi RPC/JSONL adapter.
#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
)]
pub struct PiRpcCommandPath(String);

impl PiRpcCommandPath {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Session directory owned by the external Pi RPC/JSONL adapter.
#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
)]
pub struct PiRpcSessionDirectoryPath(String);

impl PiRpcSessionDirectoryPath {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// The supervised harness runtime variant. Closed enum — every
/// production harness ships with one of these kinds.
#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum HarnessKind {
    Codex,
    Claude,
    Pi,
    Fixture,
}

/// Command shape the Pi RPC/JSONL adapter uses when delivering a message.
#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub enum PiRpcDeliveryMode {
    Prompt,
    Steer,
    FollowUp,
}

/// Optional model selector passed to the Pi RPC/JSONL adapter.
#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
)]
pub struct PiRpcModelPattern(String);

impl PiRpcModelPattern {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Typed boundary for the external Pi RPC/JSONL adapter process.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct PiRpcJsonlAdapterConfiguration {
    /// Executable path for the adapter command.
    pub command_path: PiRpcCommandPath,
    /// Directory where the adapter stores Pi session state.
    pub session_directory_path: PiRpcSessionDirectoryPath,
    /// Optional model selector understood by the adapter.
    pub model_pattern: Option<PiRpcModelPattern>,
    /// Delivery mode used when sending a message into Pi.
    pub delivery_mode: PiRpcDeliveryMode,
}

/// Startup configuration for one harness instance owned by
/// `harness-daemon`.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct HarnessInstanceConfiguration {
    /// The harness instance name this daemon serves.
    pub harness_name: HarnessName,
    /// The supervised harness runtime variant.
    pub harness_kind: HarnessKind,
    /// Optional terminal endpoint the daemon delegates to for this instance.
    pub terminal_socket_path: Option<TerminalSocketPath>,
    /// Optional Pi RPC/JSONL adapter boundary for `HarnessKind::Pi`.
    pub pi_rpc_adapter: Option<PiRpcJsonlAdapterConfiguration>,
}

/// Startup configuration for `harness-daemon`.
///
/// Replaces the previous `--socket`, `--harness`, `--kind`,
/// `--terminal-socket`, `PERSONA_HARNESS_TERMINAL_SOCKET`,
/// `PERSONA_SOCKET_MODE`, `PERSONA_SUPERVISION_SOCKET_PATH`, and
/// `PERSONA_SUPERVISION_SOCKET_MODE` argv/environment-variable
/// surface.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct HarnessDaemonConfiguration {
    /// Where the daemon binds its harness Unix socket.
    pub domain_socket_path: DomainSocketPath,
    /// chmod applied to the harness socket after bind.
    pub domain_socket_mode: DomainSocketMode,
    /// Where the daemon binds its engine-management Unix socket.
    pub engine_management_socket_path: EngineManagementSocketPath,
    /// chmod applied to the engine-management socket after bind.
    pub engine_management_socket_mode: EngineManagementSocketMode,
    /// The engine owner identity passed to the harness daemon.
    pub owner_identity: OwnerIdentity,
    /// The harness instances owned by this component daemon.
    pub harnesses: Vec<HarnessInstanceConfiguration>,
}

impl HarnessDaemonConfiguration {
    pub fn from_rkyv_bytes(bytes: &[u8]) -> Result<Self, HarnessDaemonConfigurationArchiveError> {
        rkyv::from_bytes::<Self, rkyv::rancor::Error>(bytes)
            .map_err(|_| HarnessDaemonConfigurationArchiveError::Decode)
    }

    pub fn to_rkyv_bytes(&self) -> Result<Vec<u8>, HarnessDaemonConfigurationArchiveError> {
        rkyv::to_bytes::<rkyv::rancor::Error>(self)
            .map(|bytes| bytes.to_vec())
            .map_err(|_| HarnessDaemonConfigurationArchiveError::Encode)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum HarnessDaemonConfigurationArchiveError {
    #[error("failed to encode harness daemon configuration archive")]
    Encode,

    #[error("failed to decode harness daemon configuration archive")]
    Decode,
}
