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
generates `Query`, its reply root generates `Response`, and the declared
`HarnessStreamEvent` enum carries the per-subscription stream.

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
  `WindowUsage` holds the window's share in hundredths of a percentage point:
  `Current(QuotaShare)`; `StaleAfterReset(QuotaShare)`, the values the
  provider reported for a window whose reset has already passed, never a fresh
  window; or `Unreadable(UsageUnreadableReason)` when the provider's
  percentage is absent, not finite, negative or above 100. Neither provider
  documents an overrun, so a percentage above 100 is unreadable, never clamped.
  `RemainingBasisPoints` is 10000 minus `UsedBasisPoints`.
- `ResetBasis` says where the reset came from; `ResetCountdown` is the time
  until it (`Pending`), the time since it (`Passed`), or `Unknown`. The
  countdown is its own state: a known reset stays visible when the window's
  duration is unknown.
- `WindowDurationBasis` is `ProviderDeclared`, `ProviderWindowNamed` (only the
  provider's own named windows, such as Claude's `five_hour` and `seven_day`)
  or `Unknown`. A duration is never inferred from a shared reset time.
- `AbsoluteLimit` is `NotExposedByProvider`: neither provider reports tokens or
  money per window.
- `BudgetDerivation` is `Derived(WallClockBudget)` only when the window is
  current and its remaining share, its pending reset and its duration are all
  known; otherwise `Unknown(BudgetUnknownReason)`. A `WallClockBudget` is a
  one-snapshot wall-clock budget figure, labelled `OneSnapshotWallClock`, not
  an observed burn: it carries its operands (remaining, seconds until reset,
  window duration), the remaining share per wall-clock day until the reset, the
  share an even wall-clock spend would have used by now, and the variance
  (used minus that even share; positive means more used than an even spend).
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
`QueryHarnessStatus`, `WatchTranscript`, `UnwatchTranscript` — names that are
not themselves declared types.

## Frames

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
