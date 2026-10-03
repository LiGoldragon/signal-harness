use signal_harness::{
    AbsoluteLimit, AdapterExitStatus, AdapterExited, ByteViewable, ClaudeSessionLifecycle,
    ClaudeSessionObservation, ContextBasis, ContextFreshness, ContextUnavailableReason,
    DeliveryFailed, DeliveryFailureReason, HarnessDaemonConfiguration,
    HarnessInstanceConfiguration, HarnessKind, HarnessStreamEvent, HarnessTranscriptToken,
    MessageDelivery, PaceDerivation, PaceUnknownReason, PiRpcDeliveryMode,
    PiRpcJsonlAdapterConfiguration, Query, QuotaLimit, QuotaPace, QuotaWindow, ResetBasis,
    Response, Restorable, SessionContext, SessionContextObservation, SessionContextUnavailable,
    Signal, Signalizable, SubscriptionObservation, SubscriptionUsage, TranscriptObservation,
    TurnLaunch, UsageProvider, UsageSnapshot, UsageSource, UsageUnavailable,
    UsageUnavailableReason, WatchHarnessTranscript, WindowDurationBasis, WindowFreshness,
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
        Query::UsageSnapshotQuery,
    ]
}

/// One snapshot carrying every value state the reply distinguishes: a derived
/// pace with its operands, an unknown pace, retained unrecognized windows, a
/// deduplicated Codex account over two homes, an unavailable provider home, a
/// proxy context, a superseded context, and an unbound thread.
fn usage_snapshot() -> UsageSnapshot {
    let observed_at = 1_791_600_000_000_000_000;
    UsageSnapshot {
        snapshot_time: observed_at,
        subscription_observations: vec![
            SubscriptionObservation::Observed(SubscriptionUsage {
                usage_provider: UsageProvider::Claude,
                usage_source: UsageSource::ClaudeOauthUsageEndpoint,
                observation_time: observed_at,
                plan_name_option: Some("max".into()),
                account_homes: vec![],
                quota_limits: vec![
                    QuotaLimit {
                        quota_limit_identifier: "session".into(),
                        quota_limit_name_option: None,
                        quota_windows: vec![QuotaWindow {
                            provider_window_name: "session".into(),
                            provider_scope_name_option: None,
                            used_basis_points: 3300,
                            remaining_basis_points: 6700,
                            reset_basis: ResetBasis::ProviderResetTime(1_791_609_599),
                            window_duration_basis: WindowDurationBasis::ProviderWindowNamed(300),
                            absolute_limit: AbsoluteLimit::NotExposedByProvider,
                            window_freshness: WindowFreshness::Current,
                            pace_derivation: PaceDerivation::Derived(QuotaPace {
                                remaining_basis_points: 6700,
                                seconds_until_reset: 9599,
                                window_duration_minutes: 300,
                                remaining_basis_points_per_day: 60_306,
                                even_pace_used_basis_points: 4667,
                                pace_variance_basis_points: -1367,
                            }),
                        }],
                    },
                    QuotaLimit {
                        quota_limit_identifier: "weekly".into(),
                        quota_limit_name_option: None,
                        quota_windows: vec![QuotaWindow {
                            provider_window_name: "weekly_scoped".into(),
                            provider_scope_name_option: Some("Fable".into()),
                            used_basis_points: 1200,
                            remaining_basis_points: 8800,
                            reset_basis: ResetBasis::ProviderResetTime(1_792_069_199),
                            window_duration_basis: WindowDurationBasis::Unknown,
                            absolute_limit: AbsoluteLimit::NotExposedByProvider,
                            window_freshness: WindowFreshness::Current,
                            pace_derivation: PaceDerivation::Unknown(
                                PaceUnknownReason::WindowDurationUnknown,
                            ),
                        }],
                    },
                ],
                unrecognized_window_names: vec!["tangelo".into()],
            }),
            SubscriptionObservation::Observed(SubscriptionUsage {
                usage_provider: UsageProvider::Codex,
                usage_source: UsageSource::CodexAppServerRateLimits,
                observation_time: observed_at,
                plan_name_option: Some("pro".into()),
                account_homes: vec![".codex-next".into(), ".codex-next-8mkkxq293hk2".into()],
                quota_limits: vec![QuotaLimit {
                    quota_limit_identifier: "codex".into(),
                    quota_limit_name_option: None,
                    quota_windows: vec![QuotaWindow {
                        provider_window_name: "primary".into(),
                        provider_scope_name_option: None,
                        used_basis_points: 2200,
                        remaining_basis_points: 7800,
                        reset_basis: ResetBasis::ProviderResetTime(1_791_580_388),
                        window_duration_basis: WindowDurationBasis::ProviderDeclared(10_080),
                        absolute_limit: AbsoluteLimit::NotExposedByProvider,
                        window_freshness: WindowFreshness::ResetPassed,
                        pace_derivation: PaceDerivation::Unknown(PaceUnknownReason::ResetPassed),
                    }],
                }],
                unrecognized_window_names: vec![],
            }),
            SubscriptionObservation::Unavailable(UsageUnavailable {
                usage_provider: UsageProvider::Codex,
                observation_time: observed_at,
                account_home_option: Some(".codex".into()),
                usage_unavailable_reason: UsageUnavailableReason::TransportTimedOut,
            }),
        ],
        session_context_observations: vec![
            SessionContextObservation::Observed(SessionContext {
                usage_provider: UsageProvider::Claude,
                session_identifier: "28d847ee-f350-4a24-8cb0-0e88abf16cbe".into(),
                flow_identifier_option: Some("28d847".into()),
                session_name_option: Some("Psyche.{ Opus 28d847 }".into()),
                model_identifier_option: Some("claude-opus-5-5".into()),
                observation_time: observed_at,
                event_time_option: Some(1_791_599_990_000_000_000),
                context_basis: ContextBasis::ClaudeTranscriptLastRequest,
                context_freshness: ContextFreshness::Proxy,
                context_tokens_option: Some(84_000),
                context_window_tokens_option: None,
                context_used_basis_points_option: None,
            }),
            SessionContextObservation::Observed(SessionContext {
                usage_provider: UsageProvider::Codex,
                session_identifier: "01a10384-16ed-7dd2-93b3-b6bd66c26f85".into(),
                flow_identifier_option: None,
                session_name_option: None,
                model_identifier_option: Some("gpt-6-astra".into()),
                observation_time: observed_at,
                event_time_option: Some(1_791_599_900_000_000_000),
                context_basis: ContextBasis::CodexRolloutLastTokenCount,
                context_freshness: ContextFreshness::Superseded,
                context_tokens_option: None,
                context_window_tokens_option: Some(272_000),
                context_used_basis_points_option: None,
            }),
            SessionContextObservation::Unavailable(SessionContextUnavailable {
                usage_provider: UsageProvider::Codex,
                session_identifier: "01a0fdcb-9409-7ba1-a656-a647de94a94b".into(),
                observation_time: observed_at,
                context_unavailable_reason: ContextUnavailableReason::ThreadUnbound,
            }),
        ],
    }
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
        Response::UsageSnapshot(usage_snapshot()),
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
fn response_round_trips_as_datom_text() {
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Protosizable, ReaderBudget, Textualizable};

    for response in responses() {
        let text = response.clone().datomize(vec![]).protosize().textualize();
        let restored = Potential::<Response>::from(text)
            .actualize(&mut Budget {
                remaining: 16384,
                reader: ReaderBudget { remaining: 16384 },
                depth: 0,
                maximum_depth: 1024,
            })
            .expect("Datom restores");
        assert_eq!(restored, response);
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
        lines, 28,
        "canonical file should carry twenty-eight contract heads"
    );
}

/// One generic transport, written against `signal`'s kinds alone, carries a
/// harness frame and a Persona frame. That is only possible because both
/// contracts speak `signal`'s frame rather than each vendoring its own; a
/// vendored copy is a distinct Rust type and this function would not accept
/// both.
#[test]
fn one_generic_transport_carries_harness_and_persona_frames() {
    fn ship<T>(value: &T) -> Vec<u8>
    where
        T: signal::Signalizable,
        signal::Signal<T>: signal::ByteViewable,
    {
        use signal::ByteViewable;
        value.signalize().expect("archive").bytes().to_vec()
    }
    fn land<T>(bytes: Vec<u8>) -> T
    where
        signal::Signal<T>: signal::Restorable<T>,
    {
        use signal::Restorable;
        signal::Signal::<T>::from(bytes).restore().expect("restore")
    }

    let harness_query = Query::WatchHarnessTranscript(WatchHarnessTranscript {
        harness_name: "claude".into(),
    });
    let persona_query = signal_persona::Query::Stop(String::from("router"));

    let landed_harness: Query = land(ship(&harness_query));
    let landed_persona: signal_persona::Query = land(ship(&persona_query));
    assert_eq!(landed_harness, harness_query);
    assert_eq!(landed_persona, persona_query);

    // The two contracts' re-exported frame names denote one type.
    let framed: Signal<Query> = signal::Signal::<Query>::from(ship(&harness_query));
    let persona_framed: signal_persona::Signal<signal_persona::Query> =
        signal::Signal::from(ship(&persona_query));
    assert_eq!(
        <Signal<Query> as Restorable<Query>>::restore(&framed).expect("restore"),
        harness_query
    );
    assert_eq!(
        signal::Restorable::restore(&persona_framed).expect("restore"),
        persona_query
    );
}
