# Scoring v2 Backtest — 2026-07-16

v1 → v2 comparison across every scored municipality, same financial year (2024)
for all 206 joined pairs — zero year drift, so every delta below is the formula,
not the data. Baselines: [snapshots/v1-scores-2026-07-08.json](snapshots/v1-scores-2026-07-08.json)
vs [snapshots/v2-scores-2026-07-16.json](snapshots/v2-scores-2026-07-16.json).

**Verdict: v2 ships as-is.** The distribution is stable, every designed behavior
fired on exactly the municipalities it was designed for, and the movers are
explainable in one sentence each. Two calibration observations logged for the
annual review (below) — neither blocks.

## Coverage

| | v1 | v2 |
|---|---|---|
| Municipalities scored | 208 | **206** |
| Without data | 5 | 7 |

The two "lost" municipalities are the point, not a bug:

- **Msinga (KZN244)** — was **#2 nationally at 87.45** on the strength of
  *negative reported debt*. Its figures are graded `unreliable`; v2 suppresses
  the raw-derived pillars and it is now unscored. The books that earned a
  qualified-at-best audit no longer earn a podium finish.
- **Ratlou (NW381)** — reported R793k annual revenue for ~110k people. Same
  treatment.

## Distribution — stable, no recalibration needed

| Band | v1 | v2 |
|---|---|---|
| 0–19 | 1 | 1 |
| 20–39 | 61 | 61 |
| 40–59 | 82 | 85 |
| 60–79 | 60 | 57 |
| 80–100 | 2 | 2 |
| **Mean** | 50.6 | **50.1** |

Mean Δ −0.53, median Δ −0.46, range −12.8 to +12.3. The hybrid anchors moved
individual municipalities without moving the center of gravity — which is what
"measure differently, not harder" should look like.

## Pillar means — where the redesign bit

| Pillar | v1 | v2 | Why |
|---|---|---|---|
| Financial Health | 38.3 | **49.9** | Own-revenue share replaces revenue-per-capita. v1 crushed every rural municipality toward 0 for being poor *per capita*; v2 centers the pillar on self-sufficiency, which is what it was supposed to measure. |
| Infrastructure | 80.3 | **72.8** | The R&M blend deflates a pillar that was systematically flattered. Municipalities *build* (capex is grant-funded) but do not *maintain* — the mean R&M intensity is far below Treasury's 8% norm. Not even Cape Town clears it (2.3%). |
| Efficiency | 26.2 | 26.2 | Unchanged by design. |
| Accountability | 62.4 | **51.6** | UIFW now costs points. Of 196 municipalities with reported UIFW, **118 waste ≥ 10% of their operating budget** (sub-score 0); only 20 keep it under 1%. |

## Top 10 under v2

| # | Municipality | v2 | v1 rank | Move |
|---|---|---|---|---|
| 1 | Swartland | 93.5 | 1 | — |
| 2 | Hessequa | 83.7 | 3 | +1 |
| 3 | Langeberg | 79.6 | 7 | +4 |
| 4 | Saldanha Bay | 79.5 | 5 | +1 |
| 5 | Greater Tzaneen | 79.3 | 22 | +17 |
| 6 | Mossel Bay | 77.8 | 10 | +4 |
| 7 | Witzenberg | 77.3 | 15 | +8 |
| 8 | George | 76.7 | 11 | +3 |
| 9 | Alfred Duma | 76.7 | 43 | +34 |
| 10 | Cape Town | 76.0 | 4 | −6 |

Swartland holds #1 for the right reasons: 85% own-revenue, 6.5% R&M, 1.4% UIFW.
The Western Cape still dominates, but the list now admits well-run non-Western-Cape
municipalities (Greater Tzaneen, Alfred Duma) that v1's revenue-per-capita
proxy had locked out. **Metro note:** Cape Town (76.0) edges eThekwini (75.9)
as top metro by 0.06 points; eThekwini *rose* #17 → #11 on near-norm
maintenance spend (R&M 7.3% — the best of the big metros).

## Movers — every one tells the intended story

**Gainers** are self-funded municipalities that v1 punished for modest
per-capita revenue: Alfred Duma +12.3 (FH 30→68), Msunduzi +11.5, Oudtshoorn
+10.1 (FH 53→82), uMngeni +9.4 (FH 48→80).

**Losers** are one repeated profile — grant-funded builders who don't maintain
and waste heavily. Jozini −12.8 is the archetype: 21% own-revenue, 1.1% R&M,
25% of opex wasted. Its Infrastructure pillar fell 100 → 74 (capex alone maxed
v1) and Accountability 75 → 52.5 (UIFW sub-score 0). Same shape for Moretele,
Impendle, Ndwedwe, eDumbe, Dr. A.B. Xuma, Mtubatuba, Kagisano-Molopo — almost
all rural KZN/NW. **v1's "infrastructure investment" pillar was rewarding the
grant-spend treadmill; v2 asks whether anyone maintains what gets built.**

## Sanity anchors

| Municipality | v1 | v2 | Check |
|---|---|---|---|
| Msinga (KZN244) | 87.45 (#2) | **unscored** | Negative debt suppressed ✓ |
| eThekwini | 73.9 (#17) | 75.9 (#11) | Best-maintaining metro rises ✓ |
| Cape Town | 78.9 (#4) | 76.0 (#10) | Low R&M (2.3%) costs it, stays elite ✓ |
| Johannesburg | 52.2 (#93) | 49.1 (#93) | Mid-table metro, rank unchanged ✓ |
| Matjhabeng (FS184) | 19.9 (#206) | 23.5 (#204) | R1.4bn UIFW = 31% of opex, stays bottom-5 ✓ |

## Input coverage (400 municipality-year rows, all now `score_version = 2`)

- Operational transfers (item 2200): **382/400** — own-revenue computable nearly everywhere
- Repairs & maintenance: **375/400**
- UIFW: **227/400** — blended only where reported, per design
- Confidence: 5 unreliable, 6 suspect

## Calibration observations (for the annual review, not blockers)

1. **UIFW blend can soften an earned zero.** Msunduzi's audit sub-score is 0
   (statements outstanding), but low reported UIFW lifts Accountability to
   27.8. When the statements were never audited, the UIFW figure deserves less
   trust, not a 30% say. v2.1 candidate: skip the UIFW blend when the audit
   sub-score is an earned 0.
2. **Verify UIFW is per-year incurrence, not cumulative balance.** 118/196
   municipalities at ≥ 10% of opex is consistent with the AG's irregular-
   expenditure crisis narrative, and spot checks (CPT 1.1%, Matjhabeng 31.2%)
   match published figures — but irregular expenditure balances accumulate
   until condoned, so confirm the cube's basis before hardening the 10% anchor.
