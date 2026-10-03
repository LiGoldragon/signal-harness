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
pub type MetaSocketPath = String;
#[rustfmt::skip]
pub type MetaSocketMode = i64;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct HarnessDaemonConfiguration {
    pub domain_socket_path: signal_persona::DomainSocketPath,
    pub domain_socket_mode: signal_persona::DomainSocketMode,
    pub meta_socket_path: MetaSocketPath,
    pub meta_socket_mode: MetaSocketMode,
    pub engine_management_socket_path: signal_persona::EngineManagementSocketPath,
    pub engine_management_socket_mode: signal_persona::EngineManagementSocketMode,
    pub owner_identity: signal_persona::OwnerIdentity,
    pub harness_instance_configurations: HarnessInstanceConfigurations,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum UsageProvider {
    Claude,
    Codex,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum UsageSource {
    ClaudeOauthUsageEndpoint,
    CodexAppServerRateLimits,
}
#[rustfmt::skip]
pub type SnapshotTime = i64;
#[rustfmt::skip]
pub type ObservationTime = i64;
#[rustfmt::skip]
pub type EventTime = i64;
#[rustfmt::skip]
pub type PlanName = String;
#[rustfmt::skip]
pub type AccountHome = String;
#[rustfmt::skip]
pub type AccountHomes = std::vec::Vec<AccountHome>;
#[rustfmt::skip]
pub type QuotaLimitIdentifier = String;
#[rustfmt::skip]
pub type QuotaLimitName = String;
#[rustfmt::skip]
pub type ProviderWindowName = String;
#[rustfmt::skip]
pub type ProviderScopeName = String;
#[rustfmt::skip]
pub type UnrecognizedWindowNames = std::vec::Vec<ProviderWindowName>;
#[rustfmt::skip]
pub type SourceFactName = String;
#[rustfmt::skip]
pub type UnmodeledSourceFacts = std::vec::Vec<SourceFactName>;
#[rustfmt::skip]
pub type UsedBasisPoints = i64;
#[rustfmt::skip]
pub type RemainingBasisPoints = i64;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ShareConversion {
    ProviderPercentRoundedToBasisPoint,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct QuotaShare {
    pub used_basis_points: UsedBasisPoints,
    pub remaining_basis_points: RemainingBasisPoints,
    pub share_conversion: ShareConversion,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum UsageUnreadableReason {
    PercentageAbsent,
    PercentageNotFinite,
    PercentageNegative,
    PercentageAboveFull,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum WindowUsage {
    Current(QuotaShare),
    StaleAfterReset(QuotaShare),
    Unreadable(UsageUnreadableReason),
}
#[rustfmt::skip]
pub type ResetEpochSecond = i64;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ResetBasis {
    ProviderResetTime(ResetEpochSecond),
    Unknown,
}
#[rustfmt::skip]
pub type SecondsUntilReset = i64;
#[rustfmt::skip]
pub type SecondsSinceReset = i64;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ResetCountdown {
    Pending(SecondsUntilReset),
    Passed(SecondsSinceReset),
    Unknown,
}
#[rustfmt::skip]
pub type LocalDateTime = String;
#[rustfmt::skip]
pub type TimezoneName = String;
#[rustfmt::skip]
pub type UtcOffsetSeconds = i64;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct LocalResetTime {
    pub local_date_time: LocalDateTime,
    pub timezone_name: TimezoneName,
    pub utc_offset_seconds: UtcOffsetSeconds,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum LocalResetUnknownReason {
    ResetUnknown,
    TimezoneUnavailable,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum LocalReset {
    Rendered(LocalResetTime),
    Unknown(LocalResetUnknownReason),
}
#[rustfmt::skip]
pub type WindowDurationMinutes = i64;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum WindowDurationBasis {
    ProviderDeclared(WindowDurationMinutes),
    ProviderWindowNamed(WindowDurationMinutes),
    Unknown,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum PeriodSemantics {
    FixedPeriod,
    NotEstablished,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum AbsoluteLimit {
    NotExposedByProvider,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum RateBasis {
    OneSnapshotClockAllowance,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum RateRounding {
    TowardZero,
}
#[rustfmt::skip]
pub type RemainingBasisPointsPerClockHour = i64;
#[rustfmt::skip]
pub type RemainingBasisPointsPerClockDay = i64;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RemainderByResetRate {
    pub rate_basis: RateBasis,
    pub remaining_basis_points: RemainingBasisPoints,
    pub seconds_until_reset: SecondsUntilReset,
    pub remaining_basis_points_per_clock_hour: RemainingBasisPointsPerClockHour,
    pub remaining_basis_points_per_clock_day: RemainingBasisPointsPerClockDay,
    pub rate_rounding: RateRounding,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum RemainderRateUnknownReason {
    UsageUnreadable,
    UsageStale,
    ResetUnknown,
    ResetPassed,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum RemainderRateDerivation {
    Derived(RemainderByResetRate),
    Unknown(RemainderRateUnknownReason),
}
#[rustfmt::skip]
pub type UniformBasisPointsPerClockDay = i64;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct UniformWindowRate {
    pub window_duration_minutes: WindowDurationMinutes,
    pub uniform_basis_points_per_clock_day: UniformBasisPointsPerClockDay,
    pub rate_rounding: RateRounding,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum UniformRateUnknownReason {
    WindowDurationUnknown,
    WindowDurationNotPositive,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum UniformRateDerivation {
    Derived(UniformWindowRate),
    Unknown(UniformRateUnknownReason),
}
#[rustfmt::skip]
pub type ElapsedBasisPoints = i64;
#[rustfmt::skip]
pub type UsedMinusElapsedBasisPoints = i64;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ElapsedWindowPosition {
    pub window_duration_minutes: WindowDurationMinutes,
    pub seconds_until_reset: SecondsUntilReset,
    pub elapsed_basis_points: ElapsedBasisPoints,
    pub used_basis_points: UsedBasisPoints,
    pub used_minus_elapsed_basis_points: UsedMinusElapsedBasisPoints,
    pub rate_rounding: RateRounding,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ElapsedUnknownReason {
    PeriodSemanticsNotEstablished,
    UsageUnreadable,
    UsageStale,
    ResetUnknown,
    ResetPassed,
    WindowDurationUnknown,
    WindowDurationNotPositive,
    ResetBeyondWindow,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ElapsedWindowDerivation {
    Derived(ElapsedWindowPosition),
    Unknown(ElapsedUnknownReason),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct QuotaWindow {
    pub provider_window_name: ProviderWindowName,
    pub provider_scope_name_option: Option<ProviderScopeName>,
    pub window_usage: WindowUsage,
    pub reset_basis: ResetBasis,
    pub reset_countdown: ResetCountdown,
    pub local_reset: LocalReset,
    pub window_duration_basis: WindowDurationBasis,
    pub period_semantics: PeriodSemantics,
    pub absolute_limit: AbsoluteLimit,
    pub remainder_rate_derivation: RemainderRateDerivation,
    pub uniform_rate_derivation: UniformRateDerivation,
    pub elapsed_window_derivation: ElapsedWindowDerivation,
}
#[rustfmt::skip]
pub type QuotaWindows = std::vec::Vec<QuotaWindow>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct QuotaLimit {
    pub quota_limit_identifier: QuotaLimitIdentifier,
    pub quota_limit_name_option: Option<QuotaLimitName>,
    pub quota_windows: QuotaWindows,
}
#[rustfmt::skip]
pub type QuotaLimits = std::vec::Vec<QuotaLimit>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SubscriptionUsage {
    pub usage_provider: UsageProvider,
    pub usage_source: UsageSource,
    pub observation_time: ObservationTime,
    pub plan_name_option: Option<PlanName>,
    pub account_homes: AccountHomes,
    pub quota_limits: QuotaLimits,
    pub unrecognized_window_names: UnrecognizedWindowNames,
    pub unmodeled_source_facts: UnmodeledSourceFacts,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum UsageUnavailableReason {
    CredentialsAbsent,
    CredentialsUnreadable,
    AccessTokenExpired,
    NoLiveControlSocket,
    TransportFailed,
    TransportTimedOut,
    ProviderRejected,
    ProviderResponseUnreadable,
    CollectorFailed,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct UsageUnavailable {
    pub usage_provider: UsageProvider,
    pub observation_time: ObservationTime,
    pub account_home_option: Option<AccountHome>,
    pub usage_unavailable_reason: UsageUnavailableReason,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum SubscriptionObservation {
    Observed(SubscriptionUsage),
    Unavailable(UsageUnavailable),
}
#[rustfmt::skip]
pub type SubscriptionObservations = std::vec::Vec<SubscriptionObservation>;
#[rustfmt::skip]
pub type SessionIdentifier = String;
#[rustfmt::skip]
pub type FlowIdentifier = String;
#[rustfmt::skip]
pub type SessionName = String;
#[rustfmt::skip]
pub type ModelIdentifier = String;
#[rustfmt::skip]
pub type ContextWindowTokens = i64;
#[rustfmt::skip]
pub type ContextUsedBasisPoints = i64;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ContextBasis {
    ClaudeTranscriptLastRequest,
    CodexRolloutLastTokenCount,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ContextFreshness {
    Exact,
    Proxy,
    Superseded,
    Unknown,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SessionContext {
    pub usage_provider: UsageProvider,
    pub session_identifier: SessionIdentifier,
    pub flow_identifier_option: Option<FlowIdentifier>,
    pub session_name_option: Option<SessionName>,
    pub model_identifier_option: Option<ModelIdentifier>,
    pub observation_time: ObservationTime,
    pub event_time_option: Option<EventTime>,
    pub context_basis: ContextBasis,
    pub context_freshness: ContextFreshness,
    pub context_tokens_option: Option<ContextTokens>,
    pub context_window_tokens_option: Option<ContextWindowTokens>,
    pub context_used_basis_points_option: Option<ContextUsedBasisPoints>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ContextUnavailableReason {
    ThreadUnbound,
    TranscriptAbsent,
    TranscriptUnreadable,
    TransportFailed,
    TransportTimedOut,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SessionContextUnavailable {
    pub usage_provider: UsageProvider,
    pub session_identifier: SessionIdentifier,
    pub observation_time: ObservationTime,
    pub context_unavailable_reason: ContextUnavailableReason,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ContextSourceFailureReason {
    RegistryAbsent,
    RegistryUnreadable,
    RegistryEntryUnreadable,
    NoLiveControlSocket,
    TransportFailed,
    TransportTimedOut,
    ProviderRejected,
    ProviderResponseUnreadable,
    CollectorFailed,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ContextSourceUnavailable {
    pub usage_provider: UsageProvider,
    pub account_home_option: Option<AccountHome>,
    pub observation_time: ObservationTime,
    pub context_source_failure_reason: ContextSourceFailureReason,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum SessionContextObservation {
    Observed(SessionContext),
    Unavailable(SessionContextUnavailable),
    SourceUnavailable(ContextSourceUnavailable),
}
#[rustfmt::skip]
pub type SessionContextObservations = std::vec::Vec<SessionContextObservation>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum PlanningProjection {
    NotConfigured,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct UsageSnapshot {
    pub snapshot_time: SnapshotTime,
    pub planning_projection: PlanningProjection,
    pub subscription_observations: SubscriptionObservations,
    pub session_context_observations: SessionContextObservations,
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
    UsageSnapshotQuery,
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
    UsageSnapshot(UsageSnapshot),
}
