//! Architectural-truth round-trip tests for the
//! `signal-persona-harness` channel.

use nota_codec::{Decoder, Encoder, NotaDecode, NotaEncode};
use signal_core::{
    ExchangeIdentifier, ExchangeLane, LaneSequence, NonEmpty, Reply, RequestPayload, SessionEpoch,
    SignalVerb, SubReply,
};
use signal_persona_harness::{
    DeliveryCancellation, DeliveryCompleted, DeliveryFailed, DeliveryFailureReason, HarnessCrashed,
    HarnessEvent, HarnessFrame, HarnessFrameBody, HarnessHealth, HarnessName, HarnessOperationKind,
    HarnessReadiness, HarnessRequest, HarnessRequestUnimplemented, HarnessStarted, HarnessStatus,
    HarnessStatusQuery, HarnessStopped, HarnessSubscriptionRetracted, HarnessTranscriptSequence,
    HarnessTranscriptSnapshot, HarnessTranscriptToken, HarnessUnimplementedReason,
    InteractionPrompt, InteractionResolved, MessageBody, MessageDelivery, MessageSender,
    MessageSlot, SubscribeHarnessTranscript, TranscriptObservation,
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
    let expected_verb = request.signal_verb();
    let signal_request = request.into_request();
    let frame = HarnessFrame::new(HarnessFrameBody::Request {
        exchange: synthetic_exchange(),
        request: signal_request,
    });
    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = HarnessFrame::decode_length_prefixed(&bytes).expect("decode");
    match decoded.into_body() {
        HarnessFrameBody::Request { request, .. } => {
            let head = request.operations().head().clone();
            assert_eq!(head.verb, expected_verb);
            head.payload
        }
        other => panic!("expected request operation, got {other:?}"),
    }
}

fn round_trip_event(event: HarnessEvent) -> HarnessEvent {
    let reply = Reply::completed(NonEmpty::single(SubReply::Ok {
        verb: SignalVerb::Assert,
        payload: event,
    }));
    let frame = HarnessFrame::new(HarnessFrameBody::Reply {
        exchange: synthetic_exchange(),
        reply,
    });
    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = HarnessFrame::decode_length_prefixed(&bytes).expect("decode");
    match decoded.into_body() {
        HarnessFrameBody::Reply { reply, .. } => match reply {
            Reply::Accepted { per_operation, .. } => match per_operation.into_head() {
                SubReply::Ok { payload, .. } => payload,
                other => panic!("expected Ok sub-reply, got {other:?}"),
            },
            Reply::Rejected { reason } => panic!("unexpected rejected reply: {reason:?}"),
        },
        other => panic!("expected reply operation, got {other:?}"),
    }
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
fn subscribe_harness_transcript_round_trips() {
    let request = HarnessRequest::SubscribeHarnessTranscript(SubscribeHarnessTranscript {
        harness: harness(),
    });

    assert_eq!(round_trip_request(request.clone()), request);
}

#[test]
fn harness_transcript_retraction_round_trips() {
    let request =
        HarnessRequest::HarnessTranscriptRetraction(HarnessTranscriptToken { harness: harness() });

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
            HarnessRequest::SubscribeHarnessTranscript(SubscribeHarnessTranscript {
                harness: harness(),
            }),
            HarnessOperationKind::SubscribeHarnessTranscript,
        ),
        (
            HarnessRequest::HarnessTranscriptRetraction(HarnessTranscriptToken {
                harness: harness(),
            }),
            HarnessOperationKind::HarnessTranscriptRetraction,
        ),
    ];

    for (request, operation) in cases {
        assert_eq!(request.operation_kind(), operation);
    }
}

#[test]
fn harness_request_variants_declare_expected_signal_root_verbs() {
    let cases = [
        (
            HarnessRequest::MessageDelivery(MessageDelivery {
                harness: harness(),
                sender: MessageSender::new("operator"),
                body: MessageBody::new("verb witness"),
                message_slot: MessageSlot::new(1),
            }),
            SignalVerb::Assert,
        ),
        (
            HarnessRequest::InteractionPrompt(InteractionPrompt {
                harness: harness(),
                interaction_id: "i-verb".into(),
                prompt: "Approve?".into(),
                options: vec!["yes".into(), "no".into()],
            }),
            SignalVerb::Assert,
        ),
        (
            HarnessRequest::DeliveryCancellation(DeliveryCancellation {
                harness: harness(),
                message_slot: MessageSlot::new(1),
            }),
            SignalVerb::Retract,
        ),
        (
            HarnessRequest::HarnessStatusQuery(HarnessStatusQuery { harness: harness() }),
            SignalVerb::Match,
        ),
        (
            HarnessRequest::SubscribeHarnessTranscript(SubscribeHarnessTranscript {
                harness: harness(),
            }),
            SignalVerb::Subscribe,
        ),
        (
            HarnessRequest::HarnessTranscriptRetraction(HarnessTranscriptToken {
                harness: harness(),
            }),
            SignalVerb::Retract,
        ),
    ];

    for (request, verb) in cases {
        assert_eq!(request.signal_verb(), verb);
    }
}

#[test]
fn harness_operation_kind_round_trips_through_nota_text() {
    let mut encoder = Encoder::new();
    HarnessOperationKind::MessageDelivery
        .encode(&mut encoder)
        .expect("encode operation kind");
    let text = encoder.into_string();
    let mut decoder = Decoder::new(&text);
    let recovered = HarnessOperationKind::decode(&mut decoder).expect("decode operation kind");

    assert_eq!(recovered, HarnessOperationKind::MessageDelivery);
    assert_eq!(text, "MessageDelivery");
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

    let mut encoder = Encoder::new();
    observation
        .encode(&mut encoder)
        .expect("encode observation");
    let text = encoder.into_string();
    let mut decoder = Decoder::new(&text);
    let recovered = TranscriptObservation::decode(&mut decoder).expect("decode observation");

    assert_eq!(recovered, observation);
    assert_eq!(
        text,
        "(TranscriptObservation designer 42 \"ready for prompt\")"
    );
}

#[test]
fn message_delivery_request_round_trips_through_nota_text() {
    let request = HarnessRequest::MessageDelivery(MessageDelivery {
        harness: harness(),
        sender: MessageSender::new("operator"),
        body: MessageBody::new("via nota"),
        message_slot: MessageSlot::new(42),
    });

    let mut encoder = Encoder::new();
    request.encode(&mut encoder).expect("encode request");
    let text = encoder.into_string();
    let mut decoder = Decoder::new(&text);
    let recovered = HarnessRequest::decode(&mut decoder).expect("decode request");

    assert_eq!(recovered, request);
    assert_eq!(text, "(MessageDelivery designer operator \"via nota\" 42)");
}

#[test]
fn delivery_failed_event_round_trips_through_nota_text() {
    let event = HarnessEvent::DeliveryFailed(DeliveryFailed {
        harness: harness(),
        message_slot: MessageSlot::new(42),
        reason: DeliveryFailureReason::TransportRejected,
    });

    let mut encoder = Encoder::new();
    event.encode(&mut encoder).expect("encode event");
    let text = encoder.into_string();
    let mut decoder = Decoder::new(&text);
    let recovered = HarnessEvent::decode(&mut decoder).expect("decode event");

    assert_eq!(recovered, event);
    assert_eq!(text, "(DeliveryFailed designer 42 TransportRejected)");
}

#[test]
fn harness_unimplemented_event_round_trips_through_nota_text() {
    let event = HarnessEvent::HarnessRequestUnimplemented(HarnessRequestUnimplemented {
        harness: harness(),
        operation: HarnessOperationKind::MessageDelivery,
        reason: HarnessUnimplementedReason::NotBuiltYet,
    });

    let mut encoder = Encoder::new();
    event.encode(&mut encoder).expect("encode event");
    let text = encoder.into_string();
    let mut decoder = Decoder::new(&text);
    let recovered = HarnessEvent::decode(&mut decoder).expect("decode event");

    assert_eq!(recovered, event);
    assert_eq!(
        text,
        "(HarnessRequestUnimplemented designer MessageDelivery NotBuiltYet)"
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
