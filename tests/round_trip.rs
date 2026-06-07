//! Architectural-truth round-trip tests for the
//! `signal-harness` channel.

use nota_next::{NotaDecode, NotaEncode, NotaSource};
use signal_engine_management::{SocketMode, WirePath};
use signal_frame::{
    ExchangeIdentifier, ExchangeLane, LaneSequence, NonEmpty, Reply, RequestPayload, SessionEpoch,
    SignalOperationHeads, SubReply,
};
use signal_harness::{
    DeliveryCancellation, DeliveryCompleted, DeliveryFailed, DeliveryFailureReason, HarnessCrashed,
    HarnessEvent, HarnessFrame, HarnessFrameBody, HarnessHealth, HarnessName, HarnessOperationKind,
    HarnessReadiness, HarnessRequest, HarnessRequestUnimplemented, HarnessStarted, HarnessStatus,
    HarnessStatusQuery, HarnessStopped, HarnessSubscriptionRetracted, HarnessTranscriptSequence,
    HarnessTranscriptSnapshot, HarnessTranscriptToken, HarnessUnimplementedReason,
    InteractionPrompt, InteractionResolved, MessageBody, MessageDelivery, MessageSender,
    MessageSlot, PiRpcDeliveryMode, PiRpcJsonlAdapterConfiguration, PiRpcModelPattern,
    TranscriptObservation, WatchHarnessTranscript,
};

fn harness() -> HarnessName {
    HarnessName::new("designer")
}

fn synthetic_exchange() -> ExchangeIdentifier {
    ExchangeIdentifier::new(
        SessionEpoch::new(0),
        ExchangeLane::Connector,
        LaneSequence::first(),
    )
}

fn round_trip_request(request: HarnessRequest) -> HarnessRequest {
    let signal_request = request.into_request();
    let frame = HarnessFrame::new(HarnessFrameBody::Request {
        exchange: synthetic_exchange(),
        request: signal_request,
    });
    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = HarnessFrame::decode_length_prefixed(&bytes).expect("decode");
    match decoded.into_body() {
        HarnessFrameBody::Request { request, .. } => request.payloads().head().clone(),
        other => panic!("expected request operation, got {other:?}"),
    }
}

fn round_trip_event(event: HarnessEvent) -> HarnessEvent {
    let reply = Reply::committed(NonEmpty::single(SubReply::Ok(event)));
    let frame = HarnessFrame::new(HarnessFrameBody::Reply {
        exchange: synthetic_exchange(),
        reply,
    });
    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = HarnessFrame::decode_length_prefixed(&bytes).expect("decode");
    match decoded.into_body() {
        HarnessFrameBody::Reply { reply, .. } => match reply {
            Reply::Accepted { per_operation, .. } => match per_operation.into_head() {
                SubReply::Ok(payload) => payload,
                other => panic!("expected Ok sub-reply, got {other:?}"),
            },
            Reply::Rejected { reason } => panic!("unexpected rejected reply: {reason:?}"),
        },
        other => panic!("expected reply operation, got {other:?}"),
    }
}

fn round_trip_nota<Value>(value: Value, expected: &str)
where
    Value: NotaEncode + NotaDecode + PartialEq + std::fmt::Debug,
{
    let text = value.to_nota();
    assert_eq!(text, expected);
    let recovered = NotaSource::new(&text).parse::<Value>().expect("decode");
    assert_eq!(recovered, value);
}

#[test]
fn message_delivery_round_trips() {
    let request = HarnessRequest::MessageDelivery(MessageDelivery {
        harness: harness(),
        sender: MessageSender::new("operator"),
        body: MessageBody::new("harness delivery test"),
        message_slot: MessageSlot::new(1024),
    });
    assert_eq!(round_trip_request(request.clone()), request);
}

#[test]
fn interaction_prompt_round_trips() {
    let request = HarnessRequest::InteractionPrompt(InteractionPrompt {
        harness: harness(),
        interaction_id: "i-abc".into(),
        prompt: "Approve commit?".into(),
        options: vec!["yes".into(), "no".into()],
    });
    assert_eq!(round_trip_request(request.clone()), request);
}

#[test]
fn delivery_cancellation_round_trips() {
    let request = HarnessRequest::DeliveryCancellation(DeliveryCancellation {
        harness: harness(),
        message_slot: MessageSlot::new(7),
    });
    assert_eq!(round_trip_request(request.clone()), request);
}

#[test]
fn harness_status_query_round_trips() {
    let request = HarnessRequest::HarnessStatusQuery(HarnessStatusQuery { harness: harness() });

    assert_eq!(round_trip_request(request.clone()), request);
}

#[test]
fn watch_harness_transcript_round_trips() {
    let request =
        HarnessRequest::WatchHarnessTranscript(WatchHarnessTranscript { harness: harness() });

    assert_eq!(round_trip_request(request.clone()), request);
}

#[test]
fn unwatch_harness_transcript_round_trips() {
    let request =
        HarnessRequest::UnwatchHarnessTranscript(HarnessTranscriptToken { harness: harness() });

    assert_eq!(round_trip_request(request.clone()), request);
}

#[test]
fn harness_request_exposes_contract_owned_operation_kind() {
    let cases = [
        (
            HarnessRequest::MessageDelivery(MessageDelivery {
                harness: harness(),
                sender: MessageSender::new("operator"),
                body: MessageBody::new("kind witness"),
                message_slot: MessageSlot::new(1),
            }),
            HarnessOperationKind::MessageDelivery,
        ),
        (
            HarnessRequest::InteractionPrompt(InteractionPrompt {
                harness: harness(),
                interaction_id: "i-kind".into(),
                prompt: "Approve?".into(),
                options: vec!["yes".into(), "no".into()],
            }),
            HarnessOperationKind::InteractionPrompt,
        ),
        (
            HarnessRequest::DeliveryCancellation(DeliveryCancellation {
                harness: harness(),
                message_slot: MessageSlot::new(1),
            }),
            HarnessOperationKind::DeliveryCancellation,
        ),
        (
            HarnessRequest::HarnessStatusQuery(HarnessStatusQuery { harness: harness() }),
            HarnessOperationKind::HarnessStatusQuery,
        ),
        (
            HarnessRequest::WatchHarnessTranscript(WatchHarnessTranscript { harness: harness() }),
            HarnessOperationKind::WatchHarnessTranscript,
        ),
        (
            HarnessRequest::UnwatchHarnessTranscript(HarnessTranscriptToken { harness: harness() }),
            HarnessOperationKind::UnwatchHarnessTranscript,
        ),
    ];

    for (request, operation) in cases {
        assert_eq!(request.operation_kind(), operation);
    }
}

#[test]
fn harness_request_variants_declare_contract_local_operation_heads() {
    assert_eq!(
        <HarnessRequest as SignalOperationHeads>::HEADS,
        &[
            "MessageDelivery",
            "InteractionPrompt",
            "DeliveryCancellation",
            "HarnessStatusQuery",
            "WatchHarnessTranscript",
            "UnwatchHarnessTranscript",
        ]
    );
}

#[test]
fn harness_operation_kind_round_trips_through_nota_text() {
    round_trip_nota(HarnessOperationKind::MessageDelivery, "MessageDelivery");
}

#[test]
fn delivery_completed_round_trips() {
    let event = HarnessEvent::DeliveryCompleted(DeliveryCompleted {
        harness: harness(),
        message_slot: MessageSlot::new(1024),
    });
    assert_eq!(round_trip_event(event.clone()), event);
}

#[test]
fn delivery_failed_round_trips_for_each_reason() {
    for reason in [
        DeliveryFailureReason::TransportRejected,
        DeliveryFailureReason::HumanInputIntervened,
        DeliveryFailureReason::HarnessStoppedBeforeDelivery,
        DeliveryFailureReason::HarnessUnavailable,
    ] {
        let event = HarnessEvent::DeliveryFailed(DeliveryFailed {
            harness: harness(),
            message_slot: MessageSlot::new(1024),
            reason: reason.clone(),
        });
        assert_eq!(round_trip_event(event.clone()), event);
    }
}

#[test]
fn interaction_resolved_round_trips() {
    let event = HarnessEvent::InteractionResolved(InteractionResolved {
        harness: harness(),
        interaction_id: "i-abc".into(),
        chosen: "yes".into(),
    });
    assert_eq!(round_trip_event(event.clone()), event);
}

#[test]
fn harness_unimplemented_event_round_trips() {
    let event = HarnessEvent::HarnessRequestUnimplemented(HarnessRequestUnimplemented {
        harness: harness(),
        operation: HarnessOperationKind::InteractionPrompt,
        reason: HarnessUnimplementedReason::NotBuiltYet,
    });

    assert_eq!(round_trip_event(event.clone()), event);
}

#[test]
fn harness_status_event_round_trips() {
    let event = HarnessEvent::HarnessStatus(HarnessStatus {
        harness: harness(),
        health: HarnessHealth::Running,
        readiness: HarnessReadiness::Ready,
    });

    assert_eq!(round_trip_event(event.clone()), event);
}

#[test]
fn harness_started_round_trips() {
    let event = HarnessEvent::HarnessStarted(HarnessStarted { harness: harness() });
    assert_eq!(round_trip_event(event.clone()), event);
}

#[test]
fn harness_stopped_round_trips() {
    let event = HarnessEvent::HarnessStopped(HarnessStopped { harness: harness() });
    assert_eq!(round_trip_event(event.clone()), event);
}

#[test]
fn harness_crashed_carries_typed_detail() {
    let event = HarnessEvent::HarnessCrashed(HarnessCrashed {
        harness: harness(),
        detail: "PTY fd was closed".into(),
    });
    assert_eq!(round_trip_event(event.clone()), event);
}

#[test]
fn harness_transcript_snapshot_round_trips() {
    let event = HarnessEvent::HarnessTranscriptSnapshot(HarnessTranscriptSnapshot {
        harness: harness(),
        current_sequence: HarnessTranscriptSequence::new(0),
    });
    assert_eq!(round_trip_event(event.clone()), event);
}

#[test]
fn harness_subscription_retracted_round_trips() {
    let event = HarnessEvent::HarnessSubscriptionRetracted(HarnessSubscriptionRetracted {
        token: HarnessTranscriptToken { harness: harness() },
    });
    assert_eq!(round_trip_event(event.clone()), event);
}

#[test]
fn transcript_observation_event_round_trips_through_nota_text() {
    let observation = TranscriptObservation {
        harness: harness(),
        sequence: HarnessTranscriptSequence::new(42),
        line: "ready for prompt".into(),
    };

    round_trip_nota(observation, "([designer] 42 [ready for prompt])");
}

#[test]
fn message_delivery_request_round_trips_through_nota_text() {
    let request = HarnessRequest::MessageDelivery(MessageDelivery {
        harness: harness(),
        sender: MessageSender::new("operator"),
        body: MessageBody::new("via nota"),
        message_slot: MessageSlot::new(42),
    });

    round_trip_nota(
        request,
        "(MessageDelivery ([designer] [operator] [via nota] 42))",
    );
}

#[test]
fn delivery_failed_event_round_trips_through_nota_text() {
    let event = HarnessEvent::DeliveryFailed(DeliveryFailed {
        harness: harness(),
        message_slot: MessageSlot::new(42),
        reason: DeliveryFailureReason::TransportRejected,
    });

    round_trip_nota(event, "(DeliveryFailed ([designer] 42 TransportRejected))");
}

#[test]
fn harness_unimplemented_event_round_trips_through_nota_text() {
    let event = HarnessEvent::HarnessRequestUnimplemented(HarnessRequestUnimplemented {
        harness: harness(),
        operation: HarnessOperationKind::MessageDelivery,
        reason: HarnessUnimplementedReason::NotBuiltYet,
    });

    round_trip_nota(
        event,
        "(HarnessRequestUnimplemented ([designer] MessageDelivery NotBuiltYet))",
    );
}

#[test]
fn harness_contract_cannot_claim_router_or_system_prompt_gate_precheck() {
    let scan = DriftScan::new(env!("CARGO_MANIFEST_DIR"));

    scan.assert_absent(&[
        "already verified",
        "focus not human-owned",
        "input buffer empty",
        "signal-persona-system",
        "focus/input-buffer",
        "focus + input-buffer",
        "safety gate cleared",
    ]);
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DriftScan {
    root: std::path::PathBuf,
}

impl DriftScan {
    fn new(root: impl Into<std::path::PathBuf>) -> Self {
        Self { root: root.into() }
    }

    fn assert_absent(&self, forbidden_fragments: &[&str]) {
        let mut violations = Vec::new();
        self.collect_violations("src/lib.rs", forbidden_fragments, &mut violations);
        assert!(
            violations.is_empty(),
            "prompt-gate precheck belongs to terminal control, not this harness contract:\n{}",
            violations.join("\n")
        );
    }

    fn collect_violations(
        &self,
        relative_path: &str,
        forbidden_fragments: &[&str],
        violations: &mut Vec<String>,
    ) {
        let path = self.root.join(relative_path);
        let content = std::fs::read_to_string(&path).expect("scan source file");
        for fragment in forbidden_fragments {
            if content.contains(fragment) {
                violations.push(format!("{relative_path} contains {fragment}"));
            }
        }
    }
}

#[test]
fn harness_daemon_configuration_round_trips_through_nota_text() {
    use signal_harness::{HarnessDaemonConfiguration, HarnessInstanceConfiguration, HarnessKind};
    use signal_persona_origin::{OwnerIdentity, UnixUserIdentifier};

    let configuration = HarnessDaemonConfiguration {
        harness_socket_path: WirePath::new("/run/persona/X/harness.sock"),
        harness_socket_mode: SocketMode::new(0o600),
        supervision_socket_path: WirePath::new("/run/persona/X/harness-supervision.sock"),
        supervision_socket_mode: SocketMode::new(0o600),
        owner_identity: OwnerIdentity::UnixUser(UnixUserIdentifier::new(1000)),
        harnesses: vec![
            HarnessInstanceConfiguration {
                harness_name: harness(),
                harness_kind: HarnessKind::Pi,
                terminal_socket_path: Some(WirePath::new("/run/persona/X/terminal.sock")),
                pi_rpc_adapter: Some(PiRpcJsonlAdapterConfiguration {
                    command_path: WirePath::new("/run/current-system/sw/bin/pi-rpc"),
                    session_directory_path: WirePath::new("/var/lib/persona/pi"),
                    model_pattern: Some(PiRpcModelPattern::new("pi-*")),
                    delivery_mode: PiRpcDeliveryMode::FollowUp,
                }),
            },
            HarnessInstanceConfiguration {
                harness_name: HarnessName::new("observer"),
                harness_kind: HarnessKind::Codex,
                terminal_socket_path: None,
                pi_rpc_adapter: None,
            },
        ],
    };

    let text = configuration.to_nota();
    let recovered = NotaSource::new(&text)
        .parse::<HarnessDaemonConfiguration>()
        .expect("decode configuration");

    assert_eq!(recovered, configuration);
}

#[test]
fn harness_daemon_configuration_round_trips_through_rkyv() {
    use signal_harness::{HarnessDaemonConfiguration, HarnessInstanceConfiguration, HarnessKind};
    use signal_persona_origin::{OwnerIdentity, UnixUserIdentifier};

    let configuration = HarnessDaemonConfiguration {
        harness_socket_path: WirePath::new("/run/persona/X/harness.sock"),
        harness_socket_mode: SocketMode::new(0o600),
        supervision_socket_path: WirePath::new("/run/persona/X/harness-supervision.sock"),
        supervision_socket_mode: SocketMode::new(0o600),
        owner_identity: OwnerIdentity::UnixUser(UnixUserIdentifier::new(1000)),
        harnesses: vec![HarnessInstanceConfiguration {
            harness_name: harness(),
            harness_kind: HarnessKind::Codex,
            terminal_socket_path: None,
            pi_rpc_adapter: None,
        }],
    };

    let bytes = configuration.to_rkyv_bytes().expect("archive");
    let recovered = HarnessDaemonConfiguration::from_rkyv_bytes(&bytes).expect("decode rkyv");
    assert_eq!(recovered, configuration);
}

#[test]
fn pi_rpc_jsonl_adapter_configuration_round_trips_through_nota_text() {
    let configuration = PiRpcJsonlAdapterConfiguration {
        command_path: WirePath::new("/run/current-system/sw/bin/pi-rpc"),
        session_directory_path: WirePath::new("/var/lib/persona/pi"),
        model_pattern: Some(PiRpcModelPattern::new("pi-*")),
        delivery_mode: PiRpcDeliveryMode::Prompt,
    };

    let text = configuration.to_nota();
    let recovered = NotaSource::new(&text)
        .parse::<PiRpcJsonlAdapterConfiguration>()
        .expect("decode Pi RPC adapter");
    assert_eq!(recovered, configuration);
}

#[test]
fn pi_rpc_jsonl_adapter_configuration_round_trips_through_rkyv() {
    let configuration = PiRpcJsonlAdapterConfiguration {
        command_path: WirePath::new("/run/current-system/sw/bin/pi-rpc"),
        session_directory_path: WirePath::new("/var/lib/persona/pi"),
        model_pattern: None,
        delivery_mode: PiRpcDeliveryMode::Steer,
    };

    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&configuration).expect("archive");
    let recovered = rkyv::from_bytes::<PiRpcJsonlAdapterConfiguration, rkyv::rancor::Error>(&bytes)
        .expect("decode rkyv");
    assert_eq!(recovered, configuration);
}
