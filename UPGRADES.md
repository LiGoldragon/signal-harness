# Upgrades

## 8.0.0 → 8.1.0: Datom 0.33 readers required for new text

The authored schema and binary archive configuration are unchanged. Generated
scalar aliases become tuple structs carrying Datom derives; Rust callers must
construct the wrappers. This is the first Datom/Protos 0.33 adoption.

New printed text requires a Datom 0.33 / Protos 0.33 reader. In particular,
AccountHome strings beginning with a period now print bare: the
Option<AccountHome> value formerly printed Some.«.codex» prints Some..codex.
The same UsageSnapshot sample contains .codex-next and
.codex-next-8mkkxq293hk2. Four guillemet pairs disappear in that sample.
This is an intended quoting/head-recognition change, separate from vertical
layout changes. No quote shim or old-peer text compatibility is supplied.

The historical producer/parser were datom-codec
6dccc76b75918a91d3370a9d4fe88aa7dd567876 (0.27.0) and protos
171b21f65337983ab624b7b906397a4f1f92c5a3 (0.30.1). The new producer/parser are
eabead48b31e8d4f429d7fb156dd18548bbd79c1 and
3c7ade34010cec6cec153ae694ee335f7fea0193 (both 0.33.0).
Current Protos explicitly tests Some..a as Some with bare body .a; current
Datom explicitly tests leading-dot paths as bare strings. The historical
parser treats Some..codex as one bare run, then its Option composer rejects
that form.

Executed samples: four Query, four Response, two HarnessStreamEvent and one
daemon configuration. Old text → current reader succeeds 11/11. Current text
→ old reader succeeds 10/11; UsageSnapshot is refused at composition path
[1,2,2,1,2], expected Variant, found Bare. Each producer's own text roundtrip
succeeds 11/11. Archives and length-prefixed frames are byte-equal 11/11;
both-direction binary reads succeed 11/11 independently of text. Raw text is
equal 7/11; token equality holds 10/11. These are sampled results, not
exhaustive binary, nested-value, storage or runtime guarantees.

Known selected direct consumers at investigation time:
- harness explicitly enables datom on signal-harness 25a2d18b (8.0.0),
  with direct reader pins Datom 6dccc76b / Protos 171b21f6. Its lock also
  retains Datom 35d26d82 / Protos aac95b0d for other edges.
- meta-signal-harness forwards datom optionally; signal-harness 25a2d18b,
  Datom 6dccc76b and Protos 171b21f6 are locked.
- router declares default-features=false at its signal-harness 25a2d18b
  edge; its lock includes Datom 6dccc76b and Protos 171b21f6, plus legacy
  Protos 65e7c6d4.
- mentci declares a default-features=false branch-main edge, but its actual
  lock still selects signal-harness c05eacf1 (0.5.1), with legacy Protos
  9bce5ed8 / 65e7c6d4 / cdc74bd2 and no Datom entry.
- indexed Persona/persona manifests declare branch-main edges; both inspected
  locks select signal-harness 4d9f5879 (0.3.0), without a Datom/Protos entry.
  Clone/case identity is not reconciled here.

Those consumer sources and locks remain untouched. Feature unification and
reader identity require a consumer-specific check: a direct disabled datom
edge alone does not certify the whole graph. Integration and deployment of
newly emitted text are gated on current 0.33 readers. This source release
does not authorize consumer repins or runtime rollout.

Checks and witnesses cover x86_64-linux only. The remaining three declared
platforms were not executed.
