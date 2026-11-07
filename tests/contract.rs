use signal_harness::{
    AbsoluteLimit, AdapterExitStatus, AdapterExited, ByteViewable, ClaudeSessionLifecycle,
    ClaudeSessionObservation, ContextBasis, ContextFreshness, ContextSourceFailureReason,
    ContextSourceUnavailable, ContextUnavailableReason, DeliveryFailed, DeliveryFailureReason,
    ElapsedUnknownReason, ElapsedWindowDerivation, ElapsedWindowPosition,
    HarnessDaemonConfiguration, HarnessInstanceConfiguration, HarnessKind, HarnessStreamEvent,
    HarnessTranscriptEvent, HarnessTranscriptToken, LocalReset, LocalResetTime,
    LocalResetUnknownReason, MessageDelivery, PeriodSemantics, PiRpcDeliveryMode,
    PiRpcJsonlAdapterConfiguration, PlanningProjection, Query, QuotaLimit, QuotaShare, QuotaWindow,
    RateBasis, RateRounding, RemainderByResetRate, RemainderRateDerivation,
    RemainderRateUnknownReason, ResetBasis, ResetCountdown, Response, Restorable, SessionContext,
    SessionContextObservation, SessionContextUnavailable, ShareConversion, Signal, Signalizable,
    SubscriptionObservation, SubscriptionUsage, TranscriptObservation, TurnLaunch,
    UniformRateDerivation, UniformRateUnknownReason, UniformWindowRate, UsageProvider,
    UsageSnapshot, UsageSource, UsageUnavailable, UsageUnavailableReason, UsageUnreadableReason,
    WatchHarnessTranscript, WindowDurationBasis, WindowUsage,
};
use signal_persona::OwnerIdentity;

fn queries() -> Vec<Query> {
    vec![
        Query::MessageDelivery(MessageDelivery {
            harness_name: signal_harness::HarnessName("claude".into()),
            message_sender: signal_harness::MessageSender("operator".into()),
            message_body: signal_harness::MessageBody("rebase onto main, please".into()),
            message_slot: signal_harness::MessageSlot(7),
        }),
        Query::WatchHarnessTranscript(WatchHarnessTranscript {
            harness_name: signal_harness::HarnessName("claude".into()),
        }),
        Query::UnwatchHarnessTranscript(HarnessTranscriptToken {
            harness_name: signal_harness::HarnessName("claude".into()),
            harness_transcript_subscription_identifier:
                signal_harness::HarnessTranscriptSubscriptionIdentifier(3),
        }),
        Query::UsageSnapshotQuery,
    ]
}

/// One snapshot carrying every value state the reply distinguishes: a current
/// fixed-period window with its remainder-by-reset rate, uniform rate and
/// elapsed position, a pending reset with a local rendering and a
/// remainder-by-reset rate whose window duration is unknown, a passed reset whose old values are stale, an
/// unreadable percentage, retained unrecognized windows and unmodeled source
/// facts, a deduplicated Codex account over two homes, an unavailable provider
/// home, a failed context source, a proxy context, a superseded context, and an
/// unbound thread.
fn share(used: i64) -> QuotaShare {
    QuotaShare {
        used_basis_points: signal_harness::UsedBasisPoints(used),
        remaining_basis_points: signal_harness::RemainingBasisPoints(10_000 - used),
        share_conversion: ShareConversion::ProviderPercentRoundedToBasisPoint,
    }
}

fn local(date_time: &str) -> LocalReset {
    LocalReset::Rendered(LocalResetTime {
        local_date_time: signal_harness::LocalDateTime(date_time.into()),
        timezone_name: signal_harness::TimezoneName("America/Mexico_City".into()),
        utc_offset_seconds: signal_harness::UtcOffsetSeconds(-21_600),
    })
}

fn usage_snapshot() -> UsageSnapshot {
    let observed_at = 1_791_600_000_000_000_000;
    UsageSnapshot {
        snapshot_time: signal_harness::SnapshotTime(observed_at),
        planning_projection: PlanningProjection::NotConfigured,
        subscription_observations: signal_harness::SubscriptionObservations(vec![
            SubscriptionObservation::Observed(SubscriptionUsage {
                usage_provider: UsageProvider::Claude,
                usage_source: UsageSource::ClaudeOauthUsageEndpoint,
                observation_time: signal_harness::ObservationTime(observed_at),
                plan_name_option: Some(signal_harness::PlanName("max".into())),
                account_homes: signal_harness::AccountHomes(vec![]),
                quota_limits: signal_harness::QuotaLimits(vec![
                    QuotaLimit {
                        quota_limit_identifier: signal_harness::QuotaLimitIdentifier(
                            "five_hour".into(),
                        ),
                        quota_limit_name_option: None,
                        quota_windows: signal_harness::QuotaWindows(vec![QuotaWindow {
                            provider_window_name: signal_harness::ProviderWindowName(
                                "five_hour".into(),
                            ),
                            provider_scope_name_option: None,
                            window_usage: WindowUsage::Current(share(3300)),
                            reset_basis: ResetBasis::ProviderResetTime(
                                signal_harness::ResetEpochSecond(1_791_609_599),
                            ),
                            reset_countdown: ResetCountdown::Pending(
                                signal_harness::SecondsUntilReset(9599),
                            ),
                            local_reset: local("2026-10-09T23:19:59"),
                            window_duration_basis: WindowDurationBasis::ProviderWindowNamed(
                                signal_harness::WindowDurationMinutes(300),
                            ),
                            period_semantics: PeriodSemantics::FixedPeriod,
                            absolute_limit: AbsoluteLimit::NotExposedByProvider,
                            remainder_rate_derivation: RemainderRateDerivation::Derived(
                                RemainderByResetRate {
                                    rate_basis: RateBasis::OneSnapshotClockAllowance,
                                    remaining_basis_points: signal_harness::RemainingBasisPoints(
                                        6700,
                                    ),
                                    seconds_until_reset: signal_harness::SecondsUntilReset(9599),
                                    remaining_basis_points_per_clock_hour:
                                        signal_harness::RemainingBasisPointsPerClockHour(2512),
                                    remaining_basis_points_per_clock_day:
                                        signal_harness::RemainingBasisPointsPerClockDay(60_306),
                                    rate_rounding: RateRounding::TowardZero,
                                },
                            ),
                            uniform_rate_derivation: UniformRateDerivation::Derived(
                                UniformWindowRate {
                                    window_duration_minutes: signal_harness::WindowDurationMinutes(
                                        300,
                                    ),
                                    uniform_basis_points_per_clock_day:
                                        signal_harness::UniformBasisPointsPerClockDay(48_000),
                                    rate_rounding: RateRounding::TowardZero,
                                },
                            ),
                            elapsed_window_derivation: ElapsedWindowDerivation::Derived(
                                ElapsedWindowPosition {
                                    window_duration_minutes: signal_harness::WindowDurationMinutes(
                                        300,
                                    ),
                                    seconds_until_reset: signal_harness::SecondsUntilReset(9599),
                                    elapsed_basis_points: signal_harness::ElapsedBasisPoints(4667),
                                    used_basis_points: signal_harness::UsedBasisPoints(3300),
                                    used_minus_elapsed_basis_points:
                                        signal_harness::UsedMinusElapsedBasisPoints(-1367),
                                    rate_rounding: RateRounding::TowardZero,
                                },
                            ),
                        }]),
                    },
                    QuotaLimit {
                        quota_limit_identifier: signal_harness::QuotaLimitIdentifier(
                            "weekly".into(),
                        ),
                        quota_limit_name_option: None,
                        quota_windows: signal_harness::QuotaWindows(vec![
                            QuotaWindow {
                                provider_window_name: signal_harness::ProviderWindowName(
                                    "weekly_scoped".into(),
                                ),
                                provider_scope_name_option: Some(
                                    signal_harness::ProviderScopeName("Fable".into()),
                                ),
                                window_usage: WindowUsage::Current(share(1200)),
                                reset_basis: ResetBasis::ProviderResetTime(
                                    signal_harness::ResetEpochSecond(1_792_069_199),
                                ),
                                reset_countdown: ResetCountdown::Pending(
                                    signal_harness::SecondsUntilReset(469_199),
                                ),
                                local_reset: local("2026-10-15T06:59:59"),
                                window_duration_basis: WindowDurationBasis::Unknown,
                                period_semantics: PeriodSemantics::NotEstablished,
                                absolute_limit: AbsoluteLimit::NotExposedByProvider,
                                remainder_rate_derivation: RemainderRateDerivation::Derived(
                                    RemainderByResetRate {
                                        rate_basis: RateBasis::OneSnapshotClockAllowance,
                                        remaining_basis_points:
                                            signal_harness::RemainingBasisPoints(8800),
                                        seconds_until_reset: signal_harness::SecondsUntilReset(
                                            469_199,
                                        ),
                                        remaining_basis_points_per_clock_hour:
                                            signal_harness::RemainingBasisPointsPerClockHour(67),
                                        remaining_basis_points_per_clock_day:
                                            signal_harness::RemainingBasisPointsPerClockDay(1620),
                                        rate_rounding: RateRounding::TowardZero,
                                    },
                                ),
                                uniform_rate_derivation: UniformRateDerivation::Unknown(
                                    UniformRateUnknownReason::WindowDurationUnknown,
                                ),
                                elapsed_window_derivation: ElapsedWindowDerivation::Unknown(
                                    ElapsedUnknownReason::PeriodSemanticsNotEstablished,
                                ),
                            },
                            QuotaWindow {
                                provider_window_name: signal_harness::ProviderWindowName(
                                    "weekly_mystery".into(),
                                ),
                                provider_scope_name_option: None,
                                window_usage: WindowUsage::Unreadable(
                                    UsageUnreadableReason::PercentageAboveFull,
                                ),
                                reset_basis: ResetBasis::Unknown,
                                reset_countdown: ResetCountdown::Unknown,
                                local_reset: LocalReset::Unknown(
                                    LocalResetUnknownReason::ResetUnknown,
                                ),
                                window_duration_basis: WindowDurationBasis::Unknown,
                                period_semantics: PeriodSemantics::NotEstablished,
                                absolute_limit: AbsoluteLimit::NotExposedByProvider,
                                remainder_rate_derivation: RemainderRateDerivation::Unknown(
                                    RemainderRateUnknownReason::UsageUnreadable,
                                ),
                                uniform_rate_derivation: UniformRateDerivation::Unknown(
                                    UniformRateUnknownReason::WindowDurationUnknown,
                                ),
                                elapsed_window_derivation: ElapsedWindowDerivation::Unknown(
                                    ElapsedUnknownReason::PeriodSemanticsNotEstablished,
                                ),
                            },
                        ]),
                    },
                ]),
                unrecognized_window_names: signal_harness::UnrecognizedWindowNames(vec![
                    signal_harness::ProviderWindowName("tangelo".into()),
                ]),
                unmodeled_source_facts: signal_harness::UnmodeledSourceFacts(vec![
                    signal_harness::SourceFactName("extra_usage".into()),
                    signal_harness::SourceFactName("spend".into()),
                ]),
            }),
            SubscriptionObservation::Observed(SubscriptionUsage {
                usage_provider: UsageProvider::Codex,
                usage_source: UsageSource::CodexAppServerRateLimits,
                observation_time: signal_harness::ObservationTime(observed_at),
                plan_name_option: Some(signal_harness::PlanName("pro".into())),
                account_homes: signal_harness::AccountHomes(vec![
                    signal_harness::AccountHome(".codex-next".into()),
                    signal_harness::AccountHome(".codex-next-8mkkxq293hk2".into()),
                ]),
                quota_limits: signal_harness::QuotaLimits(vec![QuotaLimit {
                    quota_limit_identifier: signal_harness::QuotaLimitIdentifier("codex".into()),
                    quota_limit_name_option: None,
                    quota_windows: signal_harness::QuotaWindows(vec![QuotaWindow {
                        provider_window_name: signal_harness::ProviderWindowName("primary".into()),
                        provider_scope_name_option: None,
                        window_usage: WindowUsage::StaleAfterReset(share(2200)),
                        reset_basis: ResetBasis::ProviderResetTime(
                            signal_harness::ResetEpochSecond(1_791_580_388),
                        ),
                        reset_countdown: ResetCountdown::Passed(signal_harness::SecondsSinceReset(
                            19_612,
                        )),
                        local_reset: local("2026-10-09T15:13:08"),
                        window_duration_basis: WindowDurationBasis::ProviderDeclared(
                            signal_harness::WindowDurationMinutes(10_080),
                        ),
                        period_semantics: PeriodSemantics::NotEstablished,
                        absolute_limit: AbsoluteLimit::NotExposedByProvider,
                        remainder_rate_derivation: RemainderRateDerivation::Unknown(
                            RemainderRateUnknownReason::UsageStale,
                        ),
                        uniform_rate_derivation: UniformRateDerivation::Derived(
                            UniformWindowRate {
                                window_duration_minutes: signal_harness::WindowDurationMinutes(
                                    10_080,
                                ),
                                uniform_basis_points_per_clock_day:
                                    signal_harness::UniformBasisPointsPerClockDay(1428),
                                rate_rounding: RateRounding::TowardZero,
                            },
                        ),
                        elapsed_window_derivation: ElapsedWindowDerivation::Unknown(
                            ElapsedUnknownReason::PeriodSemanticsNotEstablished,
                        ),
                    }]),
                }]),
                unrecognized_window_names: signal_harness::UnrecognizedWindowNames(vec![]),
                unmodeled_source_facts: signal_harness::UnmodeledSourceFacts(vec![
                    signal_harness::SourceFactName("codex.credits".into()),
                ]),
            }),
            SubscriptionObservation::Unavailable(UsageUnavailable {
                usage_provider: UsageProvider::Codex,
                observation_time: signal_harness::ObservationTime(observed_at),
                account_home_option: Some(signal_harness::AccountHome(".codex".into())),
                usage_unavailable_reason: UsageUnavailableReason::TransportTimedOut,
            }),
        ]),
        session_context_observations: signal_harness::SessionContextObservations(vec![
            SessionContextObservation::Observed(SessionContext {
                usage_provider: UsageProvider::Claude,
                session_identifier: signal_harness::SessionIdentifier(
                    "28d847ee-f350-4a24-8cb0-0e88abf16cbe".into(),
                ),
                flow_identifier_option: None,
                session_name_option: Some(signal_harness::SessionName(
                    "Psyche.{ Opus 28d847 }".into(),
                )),
                model_identifier_option: Some(signal_harness::ModelIdentifier(
                    "claude-opus-5-5".into(),
                )),
                observation_time: signal_harness::ObservationTime(observed_at),
                event_time_option: Some(signal_harness::EventTime(1_791_599_990_000_000_000)),
                context_basis: ContextBasis::ClaudeTranscriptLastRequest,
                context_freshness: ContextFreshness::Proxy,
                context_tokens_option: Some(signal_harness::ContextTokens(84_000)),
                context_window_tokens_option: None,
                context_used_basis_points_option: None,
            }),
            SessionContextObservation::Observed(SessionContext {
                usage_provider: UsageProvider::Codex,
                session_identifier: signal_harness::SessionIdentifier(
                    "01a10384-16ed-7dd2-93b3-b6bd66c26f85".into(),
                ),
                flow_identifier_option: None,
                session_name_option: None,
                model_identifier_option: Some(signal_harness::ModelIdentifier(
                    "gpt-6-astra".into(),
                )),
                observation_time: signal_harness::ObservationTime(observed_at),
                event_time_option: Some(signal_harness::EventTime(1_791_599_900_000_000_000)),
                context_basis: ContextBasis::CodexRolloutLastTokenCount,
                context_freshness: ContextFreshness::Superseded,
                context_tokens_option: None,
                context_window_tokens_option: Some(signal_harness::ContextWindowTokens(272_000)),
                context_used_basis_points_option: None,
            }),
            SessionContextObservation::Unavailable(SessionContextUnavailable {
                usage_provider: UsageProvider::Codex,
                session_identifier: signal_harness::SessionIdentifier(
                    "01a0fdcb-9409-7ba1-a656-a647de94a94b".into(),
                ),
                observation_time: signal_harness::ObservationTime(observed_at),
                context_unavailable_reason: ContextUnavailableReason::ThreadUnbound,
            }),
            SessionContextObservation::SourceUnavailable(ContextSourceUnavailable {
                usage_provider: UsageProvider::Codex,
                account_home_option: Some(signal_harness::AccountHome(".codex".into())),
                observation_time: signal_harness::ObservationTime(observed_at),
                context_source_failure_reason: ContextSourceFailureReason::TransportTimedOut,
            }),
        ]),
    }
}

fn responses() -> Vec<Response> {
    vec![
        Response::DeliveryFailed(DeliveryFailed {
            harness_name: signal_harness::HarnessName("claude".into()),
            message_slot: signal_harness::MessageSlot(7),
            delivery_failure_reason: DeliveryFailureReason::TransportRejected,
        }),
        Response::AdapterExited(AdapterExited {
            harness_name: signal_harness::HarnessName("claude".into()),
            adapter_event_sequence: signal_harness::AdapterEventSequence(8),
            adapter_exit_status: AdapterExitStatus::Success,
        }),
        Response::UsageSnapshot(usage_snapshot()),
        Response::HarnessTranscriptEvent(HarnessTranscriptEvent {
            harness_transcript_token: HarnessTranscriptToken {
                harness_name: signal_harness::HarnessName("claude".into()),
                harness_transcript_subscription_identifier:
                    signal_harness::HarnessTranscriptSubscriptionIdentifier(3),
            },
            harness_stream_event: HarnessStreamEvent::TranscriptObservation(
                TranscriptObservation {
                    harness_name: signal_harness::HarnessName("claude".into()),
                    harness_transcript_sequence: signal_harness::HarnessTranscriptSequence(44),
                    transcript_line: signal_harness::TranscriptLine("> jj st".into()),
                },
            ),
        }),
    ]
}

fn stream_events() -> Vec<HarnessStreamEvent> {
    vec![
        HarnessStreamEvent::TranscriptObservation(TranscriptObservation {
            harness_name: signal_harness::HarnessName("claude".into()),
            harness_transcript_sequence: signal_harness::HarnessTranscriptSequence(43),
            transcript_line: signal_harness::TranscriptLine("> jj log".into()),
        }),
        HarnessStreamEvent::ClaudeSessionObservation(ClaudeSessionObservation {
            harness_name: signal_harness::HarnessName("claude".into()),
            claude_session_identifier_option: Some(signal_harness::ClaudeSessionIdentifier(
                "session-abc".into(),
            )),
            claude_model_option: None,
            turn_launch: TurnLaunch::Resumed,
            reached_end_of_turn: signal_harness::ReachedEndOfTurn(true),
            streamed_event_count: signal_harness::StreamedEventCount(12),
            tool_call_count: signal_harness::ToolCallCount(3),
            status_transition_count: signal_harness::StatusTransitionCount(4),
            transcript_path_option: None,
            assistant_response_text_option: Some(signal_harness::AssistantResponseText(
                "rebase complete".into(),
            )),
            context_tokens_option: Some(signal_harness::ContextTokens(120_000)),
            timestamp_nanoseconds: signal_persona::TimestampNanoseconds(1_757_600_000_000_000_000),
            claude_session_lifecycle: ClaudeSessionLifecycle::Active,
        }),
    ]
}

fn configuration() -> HarnessDaemonConfiguration {
    HarnessDaemonConfiguration {
        domain_socket_path: signal_persona::DomainSocketPath("/run/persona/harness.sock".into()),
        domain_socket_mode: signal_persona::DomainSocketMode(0o600),
        meta_socket_path: signal_harness::MetaSocketPath("/run/persona/meta-harness.sock".into()),
        meta_socket_mode: signal_harness::MetaSocketMode(0o600),
        engine_management_socket_path: signal_persona::EngineManagementSocketPath(
            "/run/persona/harness-meta.sock".into(),
        ),
        engine_management_socket_mode: signal_persona::EngineManagementSocketMode(0o600),
        owner_identity: OwnerIdentity::UnixUser(signal_persona::UnixUserIdentifier(1000)),
        harness_instance_configurations: signal_harness::HarnessInstanceConfigurations(vec![
            HarnessInstanceConfiguration {
                harness_name: signal_harness::HarnessName("pi".into()),
                harness_kind: HarnessKind::Pi,
                terminal_socket_path_option: None,
                pi_rpc_jsonl_adapter_configuration_option: Some(PiRpcJsonlAdapterConfiguration {
                    pi_rpc_command_path: signal_harness::PiRpcCommandPath(
                        "/run/current-system/sw/bin/pi".into(),
                    ),
                    pi_rpc_session_directory_path: signal_harness::PiRpcSessionDirectoryPath(
                        "/var/lib/persona/pi".into(),
                    ),
                    pi_rpc_model_pattern_option: None,
                    pi_rpc_delivery_mode: PiRpcDeliveryMode::Prompt,
                }),
            },
        ]),
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
                reader: ReaderBudget::try_from(4096).expect("bounded reader budget"),
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
                reader: ReaderBudget::try_from(16384).expect("bounded reader budget"),
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
            reader: ReaderBudget::try_from(8192).expect("bounded reader budget"),
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
        lines, 29,
        "canonical file should carry twenty-nine contract heads"
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
        harness_name: signal_harness::HarnessName("claude".into()),
    });
    let persona_query =
        signal_persona::Query::Stop(signal_persona::ComponentName(String::from("router")));

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
