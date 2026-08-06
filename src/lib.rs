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

#[cfg(feature = "dotos-text")]
use dotos::{DotosDecode, DotosEncode};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_frame::signal_channel;
use signal_persona::schema::lib::{z2VNyf, z2VRBs, z2VSSX, z2VUtF, z2VckR, z2Veez};

/// The ordinary Harness contract occupies the first wire seat in its family.
pub enum HarnessWire {}

impl signal_frame::WireContract for HarnessWire {
    const BINDING: signal_frame::ContractBinding = signal_frame::ContractBinding::new(
        signal_frame::ContractId::new(core::num::NonZeroU32::MIN),
        signal_frame::WireRevision::new(core::num::NonZeroU16::MIN),
    );
}

// ─── Harness identity ─────────────────────────────────────

/// A typed name for one harness instance. Multiple
/// harnesses on one machine each have their own
/// `HarnessName`; the router routes by name.
#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
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

#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MessageSender(String);

impl MessageSender {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MessageBody(String);

impl MessageBody {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
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
#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct InteractionPrompt {
    pub harness: HarnessName,
    pub interaction_id: String,
    pub prompt: String,
    pub options: Vec<String>,
}

/// Cancel a pending delivery (e.g. the recipient went
/// offline before delivery completed, or the router is
/// shutting down).
#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct DeliveryCancellation {
    pub harness: HarnessName,
    pub message_slot: MessageSlot,
}

/// Ask the harness daemon for its current minimal readiness facts.
///
/// This is intentionally small. Detailed lifecycle and transcript history are
/// harness-owned state, but a supervised engine needs one cheap typed probe
/// before it treats the daemon as started.
#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct HarnessStatusQuery {
    pub harness: HarnessName,
}

// ─── Delivery acknowledgements (harness → router) ─────────

/// The harness successfully delivered the message — the
/// bytes hit the input surface. The router can mark the
/// message as delivered in its store.
#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct DeliveryCompleted {
    pub harness: HarnessName,
    pub message_slot: MessageSlot,
}

/// Delivery failed — typed reason carried.
#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct DeliveryFailed {
    pub harness: HarnessName,
    pub message_slot: MessageSlot,
    pub reason: DeliveryFailureReason,
}

#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
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
#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct InteractionResolved {
    pub harness: HarnessName,
    pub interaction_id: String,
    pub chosen: String,
}

/// A valid request reached a harness daemon, but the daemon's current runtime
/// does not implement the operation yet.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
pub struct HarnessRequestUnimplemented {
    pub harness: HarnessName,
    pub operation: HarnessOperationKind,
    pub reason: HarnessUnimplementedReason,
}

#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum HarnessUnimplementedReason {
    NotBuiltYet,
    DependencyTrackNotLanded,
}

/// Minimal health surface for the daemon skeleton and supervisor witness.
#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct HarnessStatus {
    pub harness: HarnessName,
    pub health: HarnessHealth,
    pub readiness: HarnessReadiness,
}

#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum HarnessHealth {
    Running,
    Degraded,
    Stopped,
}

#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum HarnessReadiness {
    Ready,
    Starting,
    Unavailable,
}

// ─── Model resolution vocabulary (orchestrator → harness) ─

/// A provider model name requested exactly as a caller knows it. The harness
/// resolves this literal to a configured Claude, Codex, or Pi adapter; callers
/// do not infer the provider from the string.
#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct NamedModel(String);

impl NamedModel {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// A named capability/profile bundle requested instead of one exact provider
/// model. The harness owns the mapping from profile to provider model.
#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct CapabilityProfile(String);

impl CapabilityProfile {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ModelSelector {
    Exact(NamedModel),
    CapabilityProfile(CapabilityProfile),
}

/// Requested effort tier. The highest tiers are explicit variants so callers
/// can ask for an extra-high or maximum effort without smuggling policy in a
/// string.
#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EffortRequest {
    Minimal,
    Low,
    Medium,
    High,
    ExtraHigh,
    Maximum,
}

#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct ModelRequest {
    pub selector: ModelSelector,
    pub effort: EffortRequest,
}

/// Codex continuation identity. The provider-specific schema stays inside the
/// Codex handle variant; consumers store and pass it without inspection.
#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct CodexContinuationIdentifier(String);

impl CodexContinuationIdentifier {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Pi continuation identity. The Pi adapter owns the field meaning; the shared
/// contract only types the handle boundary.
#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PiContinuationIdentifier(String);

impl PiContinuationIdentifier {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ContinuationHandle {
    Claude(ClaudeSessionIdentifier),
    Codex(CodexContinuationIdentifier),
    Pi(PiContinuationIdentifier),
}

#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ContinuationRequest {
    Fresh,
    Prefer(ContinuationHandle),
    Require(ContinuationHandle),
}

/// One privileged model-resolution attempt: choose a model by exact name or
/// capability/profile, ask for an effort tier, and state whether an existing
/// provider continuation is fresh, preferred, or required.
#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct ModelResolutionRequest {
    pub model: ModelRequest,
    pub continuation: ContinuationRequest,
}

/// Harness-side resolution result. The selected harness kind and model are the
/// provider-neutral facts a caller may act on; provider continuation internals
/// stay wrapped in `ContinuationHandle`.
#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct ModelResolved {
    pub harness: HarnessName,
    pub harness_kind: HarnessKind,
    pub model: NamedModel,
    pub effort: EffortRequest,
    pub continuation: ContinuationHandle,
}

#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct ModelUnavailable {
    pub request: ModelResolutionRequest,
    pub reason: ModelUnavailableReason,
}

#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModelUnavailableReason {
    NoConfiguredHarness,
    ModelNotKnown,
    EffortUnsupported,
    CapabilityUnsupported,
    ProviderUnavailable,
    ContinuationUnavailable,
    AdapterConfigurationMissing,
}

// ─── Session launch (orchestrator → harness, meta plane) ──

/// Orchestrator-minted agent identity delivered to the launched process in
/// its initial prompt. The orchestrator owns the mint; this contract only
/// types the token boundary.
#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct AgentIdentityToken(String);

impl AgentIdentityToken {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// The initial prompt text handed to the launched harness process at spawn.
/// It carries the agent identity announcement and the mission; the harness
/// passes it through verbatim.
#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct InitialPrompt(String);

impl InitialPrompt {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Filesystem location of the terminal session directory a launch produced,
/// carried as path text. Present only when the launch went through a
/// PTY-owning terminal cell.
#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct SessionDirectory(String);

impl SessionDirectory {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// One privileged session-launch request: spawn a harness process of the
/// named kind carrying an orchestrator-minted agent identity in its initial
/// prompt, either fresh or continuing an existing provider session.
#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct SessionLaunchRequest {
    pub harness_kind: HarnessKind,
    pub agent_identity: AgentIdentityToken,
    pub initial_prompt: InitialPrompt,
    pub continuation: ContinuationRequest,
}

/// Harness-side launch result: the spawned process facts a caller may act
/// on — the child process id, and the terminal session directory when the
/// launch went through a PTY-owning terminal cell. Provider continuation
/// internals stay wrapped in `ContinuationHandle`.
#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct SessionLaunched {
    pub agent_identity: AgentIdentityToken,
    pub child_process_id: u32,
    pub session_directory: Option<SessionDirectory>,
    pub continuation: Option<ContinuationHandle>,
}

#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SessionLaunchRefusalReason {
    HarnessKindUnsupported,
    ContinuationUnsupported,
    LauncherUnavailable,
    SpawnFailed,
}

/// Typed launch refusal. `detail` is diagnostic text for the operator; the
/// reason is the actionable fact.
#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct SessionLaunchRefused {
    pub request: SessionLaunchRequest,
    pub reason: SessionLaunchRefusalReason,
    pub detail: String,
}

// ─── Lifecycle observations (harness → router) ────────────

/// Harness started; ready to receive deliveries.
#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct HarnessStarted {
    pub harness: HarnessName,
}

/// Harness shut down cleanly. The router stops sending
/// deliveries to this harness.
#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct HarnessStopped {
    pub harness: HarnessName,
}

/// Harness crashed / died unexpectedly. The router needs
/// to retry or escalate.
#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct HarnessCrashed {
    pub harness: HarnessName,
    pub detail: String,
}

// ─── Adapter observations (harness → router) ──────────────

/// Per-adapter observation sequence pointer. Monotonic per harness
/// adapter session. Transcript observation has its own sequence because
/// transcript lines and adapter-state events are separate streams.
#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct AdapterReady {
    pub harness: HarnessName,
    pub sequence: AdapterEventSequence,
}

/// The adapter accepted one routed input into its provider-specific
/// surface. The input may still produce later output, progress,
/// confirmation, completion, stalled, or exit events.
#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct AdapterInputAccepted {
    pub harness: HarnessName,
    pub sequence: AdapterEventSequence,
    pub message_slot: MessageSlot,
}

/// Provider-visible output observed by the adapter. Transcript storage
/// may also publish `TranscriptObservation`; this event reports the
/// adapter-level interpretation that output happened.
#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct AdapterOutput {
    pub harness: HarnessName,
    pub sequence: AdapterEventSequence,
    pub text: String,
}

/// Provider-neutral progress while a prompt turn is still in flight.
#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct AdapterProgress {
    pub harness: HarnessName,
    pub sequence: AdapterEventSequence,
    pub status: String,
}

/// The adapter observed that one prompt turn completed. This is not a
/// request to close the harness session; long-lived TUI sessions remain
/// open until an explicit close path asks for shutdown.
#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct AdapterCompletion {
    pub harness: HarnessName,
    pub sequence: AdapterEventSequence,
    pub message_slot: MessageSlot,
}

/// The adapter observed a provider-neutral confirmation prompt.
/// Policy decides whether an operator, automation rule, or later
/// escalation path answers it.
#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct AdapterConfirmationNeeded {
    pub harness: HarnessName,
    pub sequence: AdapterEventSequence,
    pub interaction_id: String,
    pub prompt: String,
    pub options: Vec<String>,
}

#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdapterStallReason {
    NoOutput,
    ReadinessTimeout,
    CompletionTimeout,
    TransportBackpressure,
}

/// The adapter did not observe the next expected provider-neutral state
/// transition within its local policy window.
#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct AdapterStalled {
    pub harness: HarnessName,
    pub sequence: AdapterEventSequence,
    pub reason: AdapterStallReason,
}

#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdapterExitStatus {
    Success,
    Failure,
}

/// The adapter observed that the provider process or session exited.
/// Runtime transport failures are still reported through typed delivery
/// failures when they affect a specific routed input.
#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
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
#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct HarnessTranscriptToken {
    pub harness: HarnessName,
    pub subscription: HarnessTranscriptSubscriptionIdentifier,
}

/// Watch the harness's transcript-observation stream. The reply is a
/// `HarnessTranscriptSnapshot` carrying the daemon-minted subscription
/// token and current sequence pointer; subsequent `TranscriptObservation`
/// events arrive on the same connection as the stream pushes them.
#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct WatchHarnessTranscript {
    pub harness: HarnessName,
}

/// Acknowledgement that a transcript-observation subscription opened.
/// Carries the token needed to unwatch this exact open subscription and
/// the current sequence pointer so the subscriber knows the starting
/// position; the next `TranscriptObservation` carries sequence
/// `current_sequence + 1`.
#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct HarnessTranscriptSnapshot {
    pub token: HarnessTranscriptToken,
    pub current_sequence: HarnessTranscriptSequence,
}

/// Typed acknowledgement that a transcript-observation subscription has
/// been closed. Returned in reply to `UnwatchHarnessTranscript`.
/// Carries the retracted token so callers can match the ack to the
/// request they sent.
#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct HarnessSubscriptionRetracted {
    pub token: HarnessTranscriptToken,
}

/// One transcript line, pushed as it becomes visible to the harness.
/// Carries the sequence pointer so the subscriber can detect gaps and
/// order events causally. Bytes are typed as `String` for the prototype;
/// the eventual shape carries typed Nexus records, not raw text.
#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct TranscriptObservation {
    pub harness: HarnessName,
    pub sequence: HarnessTranscriptSequence,
    pub line: String,
}

// ─── Claude session observation (harness → router) ────────
//
// The store-and-render-shaped observation the harness pushes for one
// Claude session turn. It rides the same multi-watch
// `HarnessTranscriptStream` as `TranscriptObservation` (per the accepted
// session-flow design §2d): the Mentci view renders it, and orchestrate's
// session store later consumes the same event. Under the
// one-session-per-instance addressing model, `HarnessName` is the whole
// per-session key — orchestrate correlates it to its durable
// `(lane, session-handle)` record via the hosting-harness binding.

/// The Claude session identifier recovered from the JSONL transcript. It
/// doubles as the `claude --resume` target; the type is named for its
/// identity role, resume being one use of it.
#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClaudeSessionIdentifier(String);

impl ClaudeSessionIdentifier {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// The Claude model literal observed for the session (e.g. a `claude-*`
/// name or a short alias). Provider-scoped to this Claude observation
/// contract; the cross-crate model-vocabulary unification is deferred.
#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClaudeModel(String);

impl ClaudeModel {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Filesystem path to the session's JSONL transcript file, as the harness
/// observer discovered it.
#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct TranscriptPath(String);

impl TranscriptPath {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// The assistant's response text captured for the observed turn.
#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct AssistantResponseText(String);

impl AssistantResponseText {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Accumulated context size in tokens — the staleness signal a later
/// routing/handover decision reads. This contract only carries the figure;
/// where the number is sourced is deferred to the harness statusline wiring
/// and is intentionally not decided here.
#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ContextTokens(u64);

impl ContextTokens {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn into_u64(self) -> u64 {
        self.0
    }
}

/// Count of streamed provider events the harness observed for the turn.
#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StreamedEventCount(u64);

impl StreamedEventCount {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn into_u64(self) -> u64 {
        self.0
    }
}

/// Count of tool calls the harness observed for the turn.
#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ToolCallCount(u64);

impl ToolCallCount {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn into_u64(self) -> u64 {
        self.0
    }
}

/// Count of status transitions the harness observed for the turn.
#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StatusTransitionCount(u64);

impl StatusTransitionCount {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn into_u64(self) -> u64 {
        self.0
    }
}

/// How the observed turn opened its session. A closed typed record, not an
/// `is_resume` bool: the provenance is a fixed variant set, and `SelfHealed`
/// records the psyche-locked self-heal path where a resume target was gone
/// and a fresh session was minted in its place.
#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TurnLaunch {
    Fresh,
    Resumed,
    SelfHealed,
}

/// The Claude session's lifecycle at the moment of observation. Closed
/// enum; the `Exited` variant carries the process exit status, reusing the
/// adapter exit-status vocabulary rather than re-inventing it.
#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClaudeSessionLifecycle {
    Ready,
    Active,
    Completed,
    Exited(AdapterExitStatus),
}

/// A store-and-render-shaped observation of one Claude session turn, pushed
/// on `HarnessTranscriptStream`. It fuses the render facts the Mentci view
/// paints (launch provenance, end-of-turn, activity counts, transcript
/// path, assistant response) with the store-shaped session facts
/// orchestrate later persists (recovered session id, model, accumulated
/// context, last activity, lifecycle). `session_identifier`, `model`,
/// `transcript_path`, `response`, and `accumulated_context` are `Option`
/// because each is genuinely absent until the harness observes it — before
/// the first turn, while a turn is still `Active`, or before a context
/// figure has been reported.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
pub struct ClaudeSessionObservation {
    /// The harness instance hosting the session — the whole per-session key
    /// under one-session-per-instance addressing.
    pub harness: HarnessName,
    /// The recovered Claude session id; absent until the JSONL observer sees
    /// it.
    pub session_identifier: Option<ClaudeSessionIdentifier>,
    /// The observed model literal; absent until reported.
    pub model: Option<ClaudeModel>,
    /// Whether this turn opened fresh, resumed, or self-healed.
    pub launch: TurnLaunch,
    /// Whether the turn reached a clean `end_turn` stop. Payload-free
    /// yes/no: it mirrors the harness observer's own boolean signal.
    pub reached_end_of_turn: bool,
    /// Streamed provider events observed for the turn.
    pub streamed_event_count: StreamedEventCount,
    /// Tool calls observed for the turn.
    pub tool_call_count: ToolCallCount,
    /// Status transitions observed for the turn.
    pub status_transition_count: StatusTransitionCount,
    /// Path to the session's JSONL transcript; absent until discovered.
    pub transcript_path: Option<TranscriptPath>,
    /// The assistant's response text; absent while the turn is still in
    /// flight.
    pub response: Option<AssistantResponseText>,
    /// The accumulated context size; `Option` because the sourcing is
    /// deferred and no figure is synthesized to fill a gap.
    pub accumulated_context: Option<ContextTokens>,
    /// Infrastructure-minted last-activity timestamp — display/ordering
    /// only, never a resume gate.
    pub last_activity: z2VUtF,
    /// The session's lifecycle at observation time.
    pub lifecycle: ClaudeSessionLifecycle,
}

// ─── Channel declaration ───────────────────────────────────

signal_channel! {
    channel Harness contract HarnessWire {
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
        ClaudeSessionObservation(ClaudeSessionObservation) belongs HarnessTranscriptStream,
    }
    stream HarnessTranscriptStream {
        token HarnessTranscriptToken;
        opened HarnessTranscriptSnapshot;
        event TranscriptObservation;
        event ClaudeSessionObservation;
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
impl From<ClaudeSessionObservation> for HarnessStreamEvent {
    fn from(p: ClaudeSessionObservation) -> Self {
        Self::ClaudeSessionObservation(p)
    }
}

// ─── Daemon configuration ──────────────────────────────────
//
// Typed startup configuration for `harness-daemon`. Deploy/bootstrap tooling
// may author or validate it through the Dotos projection, but the live daemon
// accepts only the rkyv/signal-encoded file path on argv and never decodes
// Dotos startup text.

/// Terminal socket endpoint delegated to a harness instance.
#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq, Hash)]
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
#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq, Hash)]
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
#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq, Hash)]
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
#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HarnessKind {
    Codex,
    Claude,
    Pi,
    Fixture,
}

/// Command shape the Pi RPC/JSONL adapter uses when delivering a message.
#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum PiRpcDeliveryMode {
    Prompt,
    Steer,
    FollowUp,
}

/// Optional model selector passed to the Pi RPC/JSONL adapter.
#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq, Hash)]
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
#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
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
#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
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
#[cfg_attr(feature = "dotos-text", derive(DotosEncode, DotosDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct HarnessDaemonConfiguration {
    /// Where the daemon binds its harness Unix socket.
    pub domain_socket_path: z2Veez,
    /// chmod applied to the harness socket after bind.
    pub domain_socket_mode: z2VNyf,
    /// Where the daemon binds its engine-management Unix socket.
    pub engine_management_socket_path: z2VckR,
    /// chmod applied to the engine-management socket after bind.
    pub engine_management_socket_mode: z2VSSX,
    /// The engine owner identity passed to the harness daemon.
    pub owner_identity: z2VRBs,
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
