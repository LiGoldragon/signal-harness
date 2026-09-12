use signal_harness::{
    AdapterExitStatus, AdapterExited, ByteViewable, ClaudeSessionLifecycle,
    ClaudeSessionObservation, DeliveryFailed, DeliveryFailureReason, HarnessDaemonConfiguration,
    HarnessInstanceConfiguration, HarnessKind, HarnessStreamEvent, HarnessTranscriptToken,
    MessageDelivery, PiRpcDeliveryMode, PiRpcJsonlAdapterConfiguration, Query, Response,
    Restorable, Signal, Signalizable, TranscriptObservation, TurnLaunch, WatchHarnessTranscript,
};
use signal_persona::OwnerIdentity;

fn queries() -> Vec<Query> {
    vec![
        Query::MessageDelivery(MessageDelivery {
            harness_name: "claude".into(),
            message_sender: "operator".into(),
            message_body: "rebase onto main, please".into(),
            message_slot: 7,
        }),
        Query::WatchHarnessTranscript(WatchHarnessTranscript {
            harness_name: "claude".into(),
        }),
        Query::UnwatchHarnessTranscript(HarnessTranscriptToken {
            harness_name: "claude".into(),
            harness_transcript_subscription_identifier: 3,
        }),
    ]
}

fn responses() -> Vec<Response> {
    vec![
        Response::DeliveryFailed(DeliveryFailed {
            harness_name: "claude".into(),
            message_slot: 7,
            delivery_failure_reason: DeliveryFailureReason::TransportRejected,
        }),
        Response::AdapterExited(AdapterExited {
            harness_name: "claude".into(),
            adapter_event_sequence: 8,
            adapter_exit_status: AdapterExitStatus::Success,
        }),
    ]
}

fn stream_events() -> Vec<HarnessStreamEvent> {
    vec![
        HarnessStreamEvent::TranscriptObservation(TranscriptObservation {
            harness_name: "claude".into(),
            harness_transcript_sequence: 43,
            transcript_line: "> jj log".into(),
        }),
        HarnessStreamEvent::ClaudeSessionObservation(ClaudeSessionObservation {
            harness_name: "claude".into(),
            claude_session_identifier_option: Some("session-abc".into()),
            claude_model_option: None,
            turn_launch: TurnLaunch::Resumed,
            reached_end_of_turn: true,
            streamed_event_count: 12,
            tool_call_count: 3,
            status_transition_count: 4,
            transcript_path_option: None,
            assistant_response_text_option: Some("rebase complete".into()),
            context_tokens_option: Some(120_000),
            timestamp_nanoseconds: 1_757_600_000_000_000_000,
            claude_session_lifecycle: ClaudeSessionLifecycle::Active,
        }),
    ]
}

fn configuration() -> HarnessDaemonConfiguration {
    HarnessDaemonConfiguration {
        domain_socket_path: "/run/persona/harness.sock".into(),
        domain_socket_mode: 0o600,
        engine_management_socket_path: "/run/persona/harness-meta.sock".into(),
        engine_management_socket_mode: 0o600,
        owner_identity: OwnerIdentity::UnixUser(1000),
        harness_instance_configurations: vec![HarnessInstanceConfiguration {
            harness_name: "pi".into(),
            harness_kind: HarnessKind::Pi,
            terminal_socket_path_option: None,
            pi_rpc_jsonl_adapter_configuration_option: Some(PiRpcJsonlAdapterConfiguration {
                pi_rpc_command_path: "/run/current-system/sw/bin/pi".into(),
                pi_rpc_session_directory_path: "/var/lib/persona/pi".into(),
                pi_rpc_model_pattern_option: None,
                pi_rpc_delivery_mode: PiRpcDeliveryMode::Prompt,
            }),
        }],
    }
}

#[test]
fn queries_round_trip_through_received_bytes() {
    for query in queries() {
        let received =
            Signal::<Query>::from(query.signalize().expect("query archives").bytes().to_vec());
        assert_eq!(received.restore().expect("query restores"), query);
    }
}

#[test]
fn responses_round_trip_through_received_bytes() {
    for response in responses() {
        let received = Signal::<Response>::from(
            response
                .signalize()
                .expect("response archives")
                .bytes()
                .to_vec(),
        );
        assert_eq!(received.restore().expect("response restores"), response);
    }
}

#[test]
fn stream_events_round_trip_through_received_bytes() {
    for event in stream_events() {
        let received = Signal::<HarnessStreamEvent>::from(
            event.signalize().expect("event archives").bytes().to_vec(),
        );
        assert_eq!(received.restore().expect("event restores"), event);
    }
}

#[test]
fn daemon_configuration_round_trips_through_received_bytes() {
    let configuration = configuration();
    let received = Signal::<HarnessDaemonConfiguration>::from(
        configuration
            .signalize()
            .expect("configuration archives")
            .bytes()
            .to_vec(),
    );
    assert_eq!(
        received.restore().expect("configuration restores"),
        configuration
    );
}

#[test]
fn malformed_archive_is_rejected() {
    assert!(Signal::<Query>::from(vec![1, 2, 3]).restore().is_err());
}

#[cfg(feature = "datom")]
#[test]
fn query_round_trips_as_datom_text() {
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Protosizable, ReaderBudget, Textualizable};

    for query in queries() {
        let text = query.clone().datomize(vec![]).protosize().textualize();
        let restored = Potential::<Query>::from(text)
            .actualize(&mut Budget {
                remaining: 4096,
                reader: ReaderBudget { remaining: 4096 },
                depth: 0,
                maximum_depth: 1024,
            })
            .expect("Datom restores");
        assert_eq!(restored, query);
    }
}

#[cfg(feature = "datom")]
#[test]
fn every_canonical_datom_line_actualizes_into_a_contract_head() {
    use datom_codec::{Actualizing, Budget, Potential};
    use protos::ReaderBudget;

    fn budget() -> Budget {
        Budget {
            remaining: 8192,
            reader: ReaderBudget { remaining: 8192 },
            depth: 0,
            maximum_depth: 1024,
        }
    }

    let canonical = include_str!("../examples/canonical.datom");
    let mut lines = 0;
    for line in canonical.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with(';') {
            continue;
        }
        lines += 1;
        let as_query = Potential::<Query>::from(line.to_owned())
            .actualize(&mut budget())
            .is_ok();
        let as_response = Potential::<Response>::from(line.to_owned())
            .actualize(&mut budget())
            .is_ok();
        let as_event = Potential::<HarnessStreamEvent>::from(line.to_owned())
            .actualize(&mut budget())
            .is_ok();
        assert!(
            as_query || as_response || as_event,
            "canonical line is no contract head: {line}"
        );
    }
    assert_eq!(
        lines, 26,
        "canonical file should carry twenty-six contract heads"
    );
}
