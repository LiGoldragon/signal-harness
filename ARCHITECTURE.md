# signal-harness — architecture

The ordinary Signal contract for the `router` ↔ `harness` channel.

## Direction

The channel is bidirectional and push-shaped. The router initiates delivery,
prompting, cancellation, status enquiry and transcript subscription. The
harness pushes lifecycle, adapter and resolution observations back without
being polled, and pushes transcript observations for as long as a subscription
is open.

## Surface

`ethos/signal.ethos` is the single source of the contract. Its request root
generates `Query` and its reply root generates `Response`. The declared
`HarnessStreamEvent` enum is the per-subscription stream's payload; it rides
the wire inside the `Response::HarnessTranscriptEvent` reply with the token of
the subscription it belongs to, so every frame the daemon writes on a
connection is one `Response` archive and two subscriptions on one connection
stay distinguishable.

| Request | Meaning |
|---|---|
| `MessageDelivery` | Deliver a typed message body from a sender into a harness's delivery path, at a message slot. |
| `InteractionPrompt` | Put a typed question with a fixed option set to the harness. |
| `DeliveryCancellation` | Withdraw a delivery still outstanding at a message slot. |
| `HarnessStatusQuery` | Ask one harness for its health and readiness. |
| `WatchHarnessTranscript` | Open a transcript subscription; answered with `HarnessTranscriptSnapshot`. |
| `UnwatchHarnessTranscript(HarnessTranscriptToken)` | Close the subscription the token names. |
| `UsageSnapshotQuery` | Read one fresh snapshot of every known Claude and Codex subscription's quota windows and every live session's context; answered with `UsageSnapshot`. Daemon-level: it names no harness instance. |

| Reply | Meaning |
|---|---|
| `DeliveryCompleted` / `DeliveryFailed` | The slot's delivery resolved; failure carries a typed `DeliveryFailureReason`. |
| `InteractionResolved` | The prompt was answered; carries the chosen option. |
| `HarnessRequestUnimplemented` | The request reached the surface but the runtime path is not built; carries `HarnessOperationKind` and `HarnessUnimplementedReason`. |
| `HarnessStatus` | The harness's `HarnessHealth` and `HarnessReadiness`. |
| `HarnessStarted` / `HarnessStopped` / `HarnessCrashed` | Harness lifecycle; the crash carries a detail string. |
| `AdapterReady` / `AdapterInputAccepted` / `AdapterOutput` / `AdapterProgress` / `AdapterCompletion` / `AdapterConfirmationNeeded` / `AdapterStalled` / `AdapterExited` | The adapter's own sequenced observations, each carrying an `AdapterEventSequence`. |
| `HarnessTranscriptSnapshot` | A subscription opened; carries its token and the current sequence. |
| `HarnessSubscriptionRetracted` | The subscription the token names is closed. |
| `UsageSnapshot` | One read-only usage snapshot: per-provider `SubscriptionObservation`s and per-session `SessionContextObservation`s. |
| `HarnessTranscriptEvent` | One `HarnessStreamEvent` on the open transcript subscription its `HarnessTranscriptToken` names. |

| Stream event | Meaning |
|---|---|
| `TranscriptObservation` | One transcript line at a transcript sequence. |
| `ClaudeSessionObservation` | One Claude session state observation: identity, model, launch shape, counts, transcript path, response, accumulated context, activity timestamp, lifecycle. |

Types that are declared but carried by no root — `ModelRequest`,
`ModelResolutionRequest`, `ModelResolved`, `ModelUnavailable`,
`SessionLaunchRequest`, `SessionLaunched`, `SessionLaunchRefused` — are the
model-resolution and session-launch vocabulary the daemon and its callers
share ahead of the operations that will carry them.

`HarnessDaemonConfiguration` is the harness daemon's typed startup record. It
imports the ordinary and engine-management socket paths, their modes and the
owner identity from `signal-persona`, declares the meta socket
(`MetaSocketPath`, `MetaSocketMode`) itself, and carries the per-instance
`HarnessInstanceConfiguration` set, including the Pi RPC JSONL adapter
configuration. The meta contract has its own socket because a Signal frame is
the bare rkyv archive of one contract's root, with no contract discriminator:
`meta-signal-harness` and the `signal-persona` engine-management lifecycle
cannot share one listener without guessing which contract a frame holds.

## Usage snapshot

`UsageSnapshot` is paced by invocation: one `UsageSnapshotQuery` yields one
snapshot. It carries no subscription, watch or history.

- Every provider and session result carries its own `ObservationTime`
  (epoch nanoseconds); the snapshot carries `SnapshotTime`.
- A provider result is `Observed(SubscriptionUsage)` or
  `Unavailable(UsageUnavailable)` with a stable `UsageUnavailableReason`
  (`CollectorFailed` when the collector itself failed); one provider's or
  home's failure never removes another's result.
- Codex homes that answered for the same account are one
  `SubscriptionUsage`, listing those `AccountHomes`. That record and its home
  set are the complete grouping claim: the provider's account identifier is
  read only to merge homes and is never emitted, and no cross-snapshot account
  reference is carried.
- `QuotaLimits` lists every provider-reported limit and each of its windows.
  `WindowUsage` holds the window's share in basis points (hundredths of a
  percentage point): `Current(QuotaShare)`; `StaleAfterReset(QuotaShare)`, the
  values the provider reported for a window whose reset has already passed,
  never a fresh window; or `Unreadable(UsageUnreadableReason)` when the
  provider's percentage is absent, not finite, negative or above 100. Neither
  provider documents an overrun, so a percentage above 100 is unreadable,
  never clamped. A `QuotaShare` carries used, remaining (10000 minus used) and
  its `ShareConversion`: the provider percentage rounded to the basis point.
- `ResetBasis` says where the reset came from. `ResetCountdown` is its own
  state: `Pending` (seconds until a reset still ahead), `Passed` (seconds since
  a reset at or before the observation; a passed reset is not zero time left)
  or `Unknown`. `LocalReset` renders the reset instant in the daemon host's
  configured time zone, to the source's second precision, or says why it
  cannot (`ResetUnknown`, `TimezoneUnavailable`); no zone is invented. A known
  reset and its local time stay visible when the window's duration is unknown.
- `WindowDurationBasis` is `ProviderDeclared`, `ProviderWindowNamed` (only the
  provider's own named windows, such as Claude's `five_hour` and `seven_day`)
  or `Unknown`. A duration is never inferred from a shared reset time.
  `PeriodSemantics` is `FixedPeriod` only when the source establishes a fixed
  period; a named window or a declared duration does not, so today it is
  `NotEstablished`.
- `AbsoluteLimit` is `NotExposedByProvider`: neither provider reports tokens or
  money per window.
- Three derivations stay separate, each `Derived` with its operands or
  `Unknown` with its reason, never dividing by zero:
  - `RemainderRateDerivation`: the rate to use the remainder by the reset,
    remaining divided by the positive time until it, in basis points per clock
    hour and per clock day, rounded toward zero. Labelled
    `OneSnapshotClockAllowance`: an allowance from one snapshot, not an
    observed burn or a forecast. It needs a current share and a pending reset,
    not a window duration.
  - `UniformRateDerivation`: the window's uniform rate, one full share over
    its known duration, in basis points per clock day (a week's is 100/7
    percent, 1428 basis points rounded toward zero, per day).
  - `ElapsedWindowDerivation`: the elapsed share of a fixed period,
    `e = (W - T) / W`, and used minus it. It needs `FixedPeriod` semantics,
    a current share, a pending reset and a positive known duration no shorter
    than the time left.
- `PlanningProjection` is `NotConfigured`: no usage plan is supplied, and none
  is inferred from activity.
- `UnrecognizedWindowNames` retains provider windows that are present but not
  understood. `UnmodeledSourceFacts` names the provider's present auxiliary
  allowance, credit and spend/control facts (Claude `extra_usage` and `spend`,
  Codex `credits`, `spendControlReached`, `rateLimitReachedType` and the like)
  without modelling them and without making a quota window of them.
- A context result is `Observed(SessionContext)`,
  `Unavailable(SessionContextUnavailable)` for one session, or
  `SourceUnavailable(ContextSourceUnavailable)` for an attempted collector,
  home or control socket that could not be read, with its cause.
- `SessionContext` carries `ContextBasis` and `ContextFreshness`
  (`Exact`, `Proxy`, `Superseded`, `Unknown`); a value absent from its source
  is `None`, never zero. `FlowIdentifier` is present only with an exact,
  separately witnessed session-to-Flow binding; a session's display name or
  identifier prefix is not one.
- No field carries credentials, credential paths, raw provider bodies,
  process arguments or environment contents.

## Generation

```sh
ethos-zero 'Generate.{ <repo>/ethos/signal.ethos <repo>/src/generated }'
```

`src/generated/signal.rs` is committed. `build.rs` regenerates from the ethos
source at build time and asserts equality with the committed file, so a drifted
generation fails the build rather than the review. No `Datomic` implementation
is hand-written for a declared type.

A bare ethos enum variant whose name matches a declared type generates a
payload-carrying variant. `HarnessOperationKind` is a pure tag, so its variants
are named `DeliverMessage`, `PromptInteraction`, `CancelDelivery`,
`QueryHarnessStatus`, `WatchTranscript`, `UnwatchTranscript`, `ReadUsageSnapshot` — names that are
not themselves declared types.

## Frames

One request is one length-prefixed `signal` frame holding the rkyv archive of
`Query`; one answer, and each later event on a watched connection, is one
frame of `Response`. A frame carries no envelope, exchange identifier or
contract discriminator: the root heads discriminate, the connection
correlates, and on a watch connection the replies keep the order in which the
daemon wrote them.

`src/lib.rs` carries the portable frame surface only: `Signal<T>`,
`Signalizable`, `ByteViewable`, and `Restorable<T>`. Archiving is rkyv;
`Signal<T>` carries its target contract in its type so received bytes restore
into the contract they were framed from. `Query`, `Response`,
`HarnessStreamEvent` and `HarnessDaemonConfiguration` each bear the frame
surface.

## Boundaries

This crate carries only wire vocabulary and codecs. It does not own:

- the `harness` daemon runtime or its adapters;
- socket binding or transport framing policy;
- subscription bookkeeping;
- transcript storage;
- model resolution or session launching.

Terminal control traffic lives in `signal-terminal`; upstream message traffic
in `signal-message`; component lifecycle nouns in `signal-persona`.

## Constraints

| Constraint | Witness |
|---|---|
| The contract shape is declared, never hand-written. | `build.rs` asserts `src/generated/signal.rs` equals a fresh generation from `ethos/signal.ethos`. |
| Shared Persona nouns are imported, not copied. | `HarnessDaemonConfiguration` and `ClaudeSessionObservation` carry `signal_persona::` types. |
| Every root round-trips over the real wire. | `tests/contract.rs` archives and restores `Query`, `Response`, `HarnessStreamEvent` and `HarnessDaemonConfiguration` through received bytes. |
| Malformed bytes are rejected rather than misread. | `malformed_archive_is_rejected`. |
| Canonical Datom text stays true to the contract. | `every_canonical_datom_line_actualizes_into_a_contract_head` actualizes every line of `examples/canonical.datom`. |
| Contract crate dependencies name one exact published revision. | Every `LiGoldragon` dependency in `Cargo.toml` is pinned by `rev`. |
| Contract code contains no runtime. | Source contains no actors, tokio, storage, or socket implementation. |
| Behavior is homed in traits. | The `no-free-functions` and `no-inherent-methods` Nix checks. |

## Code Map

```text
ethos/signal.ethos       the contract source
build.rs                 freshness assertion over the committed generation
src/generated/signal.rs  generated contract types
src/lib.rs               portable rkyv Signal frame surface
examples/canonical.datom canonical Datom projection of every contract head
tests/contract.rs        rkyv frame and Datom witnesses
```
