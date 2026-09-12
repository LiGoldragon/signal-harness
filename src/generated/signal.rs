#![allow(dead_code, non_camel_case_types, non_snake_case)]
#[rustfmt::skip]
pub type HarnessName = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum HarnessKind {
    Codex,
    Claude,
    Pi,
    Fixture,
}
#[rustfmt::skip]
pub type MessageSender = String;
#[rustfmt::skip]
pub type MessageBody = String;
#[rustfmt::skip]
pub type MessageSlot = i64;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct MessageDelivery {
    pub harness_name: HarnessName,
    pub message_sender: MessageSender,
    pub message_body: MessageBody,
    pub message_slot: MessageSlot,
}
#[rustfmt::skip]
pub type InteractionIdentifier = String;
#[rustfmt::skip]
pub type InteractionPromptText = String;
#[rustfmt::skip]
pub type InteractionOption = String;
#[rustfmt::skip]
pub type InteractionOptions = std::vec::Vec<InteractionOption>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct InteractionPrompt {
    pub harness_name: HarnessName,
    pub interaction_identifier: InteractionIdentifier,
    pub interaction_prompt_text: InteractionPromptText,
    pub interaction_options: InteractionOptions,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct DeliveryCancellation {
    pub harness_name: HarnessName,
    pub message_slot: MessageSlot,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct HarnessStatusQuery {
    pub harness_name: HarnessName,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct DeliveryCompleted {
    pub harness_name: HarnessName,
    pub message_slot: MessageSlot,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum DeliveryFailureReason {
    TransportRejected,
    HumanInputIntervened,
    HarnessStoppedBeforeDelivery,
    HarnessUnavailable,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct DeliveryFailed {
    pub harness_name: HarnessName,
    pub message_slot: MessageSlot,
    pub delivery_failure_reason: DeliveryFailureReason,
}
#[rustfmt::skip]
pub type InteractionChoice = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct InteractionResolved {
    pub harness_name: HarnessName,
    pub interaction_identifier: InteractionIdentifier,
    pub interaction_choice: InteractionChoice,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum HarnessOperationKind {
    DeliverMessage,
    PromptInteraction,
    CancelDelivery,
    QueryHarnessStatus,
    WatchTranscript,
    UnwatchTranscript,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum HarnessUnimplementedReason {
    NotBuiltYet,
    DependencyTrackNotLanded,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct HarnessRequestUnimplemented {
    pub harness_name: HarnessName,
    pub harness_operation_kind: HarnessOperationKind,
    pub harness_unimplemented_reason: HarnessUnimplementedReason,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum HarnessHealth {
    Running,
    Degraded,
    Stopped,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum HarnessReadiness {
    Ready,
    Starting,
    Unavailable,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct HarnessStatus {
    pub harness_name: HarnessName,
    pub harness_health: HarnessHealth,
    pub harness_readiness: HarnessReadiness,
}
#[rustfmt::skip]
pub type NamedModel = String;
#[rustfmt::skip]
pub type CapabilityProfile = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ModelSelector {
    Exact(NamedModel),
    CapabilityProfile(CapabilityProfile),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum EffortRequest {
    Minimal,
    Low,
    Medium,
    High,
    ExtraHigh,
    Maximum,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ModelRequest {
    pub model_selector: ModelSelector,
    pub effort_request: EffortRequest,
}
#[rustfmt::skip]
pub type ClaudeSessionIdentifier = String;
#[rustfmt::skip]
pub type CodexContinuationIdentifier = String;
#[rustfmt::skip]
pub type PiContinuationIdentifier = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ContinuationHandle {
    Claude(ClaudeSessionIdentifier),
    Codex(CodexContinuationIdentifier),
    Pi(PiContinuationIdentifier),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ContinuationRequest {
    Fresh,
    Prefer(ContinuationHandle),
    Require(ContinuationHandle),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ModelResolutionRequest {
    pub model_request: ModelRequest,
    pub continuation_request: ContinuationRequest,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ModelResolved {
    pub harness_name: HarnessName,
    pub harness_kind: HarnessKind,
    pub named_model: NamedModel,
    pub effort_request: EffortRequest,
    pub continuation_handle: ContinuationHandle,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ModelUnavailableReason {
    NoConfiguredHarness,
    ModelNotKnown,
    EffortUnsupported,
    CapabilityUnsupported,
    ProviderUnavailable,
    ContinuationUnavailable,
    AdapterConfigurationMissing,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ModelUnavailable {
    pub model_resolution_request: ModelResolutionRequest,
    pub model_unavailable_reason: ModelUnavailableReason,
}
#[rustfmt::skip]
pub type AgentIdentityToken = String;
#[rustfmt::skip]
pub type InitialPrompt = String;
#[rustfmt::skip]
pub type SessionDirectory = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SessionLaunchRequest {
    pub harness_kind: HarnessKind,
    pub agent_identity_token: AgentIdentityToken,
    pub initial_prompt: InitialPrompt,
    pub continuation_request: ContinuationRequest,
}
#[rustfmt::skip]
pub type ChildProcessIdentifier = i64;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SessionLaunched {
    pub agent_identity_token: AgentIdentityToken,
    pub child_process_identifier: ChildProcessIdentifier,
    pub session_directory_option: Option<SessionDirectory>,
    pub continuation_handle_option: Option<ContinuationHandle>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum SessionLaunchRefusalReason {
    HarnessKindUnsupported,
    ContinuationUnsupported,
    LauncherUnavailable,
    SpawnFailed,
}
#[rustfmt::skip]
pub type SessionLaunchRefusalDetail = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SessionLaunchRefused {
    pub session_launch_request: SessionLaunchRequest,
    pub session_launch_refusal_reason: SessionLaunchRefusalReason,
    pub session_launch_refusal_detail: SessionLaunchRefusalDetail,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct HarnessStarted {
    pub harness_name: HarnessName,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct HarnessStopped {
    pub harness_name: HarnessName,
}
#[rustfmt::skip]
pub type HarnessCrashDetail = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct HarnessCrashed {
    pub harness_name: HarnessName,
    pub harness_crash_detail: HarnessCrashDetail,
}
#[rustfmt::skip]
pub type AdapterEventSequence = i64;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct AdapterReady {
    pub harness_name: HarnessName,
    pub adapter_event_sequence: AdapterEventSequence,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct AdapterInputAccepted {
    pub harness_name: HarnessName,
    pub adapter_event_sequence: AdapterEventSequence,
    pub message_slot: MessageSlot,
}
#[rustfmt::skip]
pub type AdapterOutputText = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct AdapterOutput {
    pub harness_name: HarnessName,
    pub adapter_event_sequence: AdapterEventSequence,
    pub adapter_output_text: AdapterOutputText,
}
#[rustfmt::skip]
pub type AdapterProgressStatus = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct AdapterProgress {
    pub harness_name: HarnessName,
    pub adapter_event_sequence: AdapterEventSequence,
    pub adapter_progress_status: AdapterProgressStatus,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct AdapterCompletion {
    pub harness_name: HarnessName,
    pub adapter_event_sequence: AdapterEventSequence,
    pub message_slot: MessageSlot,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct AdapterConfirmationNeeded {
    pub harness_name: HarnessName,
    pub adapter_event_sequence: AdapterEventSequence,
    pub interaction_identifier: InteractionIdentifier,
    pub interaction_prompt_text: InteractionPromptText,
    pub interaction_options: InteractionOptions,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum AdapterStallReason {
    NoOutput,
    ReadinessTimeout,
    CompletionTimeout,
    TransportBackpressure,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct AdapterStalled {
    pub harness_name: HarnessName,
    pub adapter_event_sequence: AdapterEventSequence,
    pub adapter_stall_reason: AdapterStallReason,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum AdapterExitStatus {
    Success,
    Failure,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct AdapterExited {
    pub harness_name: HarnessName,
    pub adapter_event_sequence: AdapterEventSequence,
    pub adapter_exit_status: AdapterExitStatus,
}
#[rustfmt::skip]
pub type HarnessTranscriptSequence = i64;
#[rustfmt::skip]
pub type HarnessTranscriptSubscriptionIdentifier = i64;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct HarnessTranscriptToken {
    pub harness_name: HarnessName,
    pub harness_transcript_subscription_identifier: HarnessTranscriptSubscriptionIdentifier,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct WatchHarnessTranscript {
    pub harness_name: HarnessName,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct HarnessTranscriptSnapshot {
    pub harness_transcript_token: HarnessTranscriptToken,
    pub harness_transcript_sequence: HarnessTranscriptSequence,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct HarnessSubscriptionRetracted {
    pub harness_transcript_token: HarnessTranscriptToken,
}
#[rustfmt::skip]
pub type TranscriptLine = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct TranscriptObservation {
    pub harness_name: HarnessName,
    pub harness_transcript_sequence: HarnessTranscriptSequence,
    pub transcript_line: TranscriptLine,
}
#[rustfmt::skip]
pub type ClaudeModel = String;
#[rustfmt::skip]
pub type TranscriptPath = String;
#[rustfmt::skip]
pub type AssistantResponseText = String;
#[rustfmt::skip]
pub type ContextTokens = i64;
#[rustfmt::skip]
pub type StreamedEventCount = i64;
#[rustfmt::skip]
pub type ToolCallCount = i64;
#[rustfmt::skip]
pub type StatusTransitionCount = i64;
#[rustfmt::skip]
pub type ReachedEndOfTurn = bool;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum TurnLaunch {
    Fresh,
    Resumed,
    SelfHealed,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ClaudeSessionLifecycle {
    Ready,
    Active,
    Completed,
    Exited(AdapterExitStatus),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ClaudeSessionObservation {
    pub harness_name: HarnessName,
    pub claude_session_identifier_option: Option<ClaudeSessionIdentifier>,
    pub claude_model_option: Option<ClaudeModel>,
    pub turn_launch: TurnLaunch,
    pub reached_end_of_turn: ReachedEndOfTurn,
    pub streamed_event_count: StreamedEventCount,
    pub tool_call_count: ToolCallCount,
    pub status_transition_count: StatusTransitionCount,
    pub transcript_path_option: Option<TranscriptPath>,
    pub assistant_response_text_option: Option<AssistantResponseText>,
    pub context_tokens_option: Option<ContextTokens>,
    pub timestamp_nanoseconds: signal_persona::TimestampNanoseconds,
    pub claude_session_lifecycle: ClaudeSessionLifecycle,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum HarnessStreamEvent {
    TranscriptObservation(TranscriptObservation),
    ClaudeSessionObservation(ClaudeSessionObservation),
}
#[rustfmt::skip]
pub type TerminalSocketPath = String;
#[rustfmt::skip]
pub type PiRpcCommandPath = String;
#[rustfmt::skip]
pub type PiRpcSessionDirectoryPath = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum PiRpcDeliveryMode {
    Prompt,
    Steer,
    FollowUp,
}
#[rustfmt::skip]
pub type PiRpcModelPattern = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct PiRpcJsonlAdapterConfiguration {
    pub pi_rpc_command_path: PiRpcCommandPath,
    pub pi_rpc_session_directory_path: PiRpcSessionDirectoryPath,
    pub pi_rpc_model_pattern_option: Option<PiRpcModelPattern>,
    pub pi_rpc_delivery_mode: PiRpcDeliveryMode,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct HarnessInstanceConfiguration {
    pub harness_name: HarnessName,
    pub harness_kind: HarnessKind,
    pub terminal_socket_path_option: Option<TerminalSocketPath>,
    pub pi_rpc_jsonl_adapter_configuration_option: Option<
        PiRpcJsonlAdapterConfiguration,
    >,
}
#[rustfmt::skip]
pub type HarnessInstanceConfigurations = std::vec::Vec<HarnessInstanceConfiguration>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct HarnessDaemonConfiguration {
    pub domain_socket_path: signal_persona::DomainSocketPath,
    pub domain_socket_mode: signal_persona::DomainSocketMode,
    pub engine_management_socket_path: signal_persona::EngineManagementSocketPath,
    pub engine_management_socket_mode: signal_persona::EngineManagementSocketMode,
    pub owner_identity: signal_persona::OwnerIdentity,
    pub harness_instance_configurations: HarnessInstanceConfigurations,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Query {
    MessageDelivery(MessageDelivery),
    InteractionPrompt(InteractionPrompt),
    DeliveryCancellation(DeliveryCancellation),
    HarnessStatusQuery(HarnessStatusQuery),
    WatchHarnessTranscript(WatchHarnessTranscript),
    UnwatchHarnessTranscript(HarnessTranscriptToken),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Response {
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
