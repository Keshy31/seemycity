# The State of South Africa's Municipal Finances

### An analysis of 204 scored municipalities from the SeeMyCity dataset

**Date:** 7 July 2026
**Data:** National Treasury "Municipal Money" open data (audited actuals), scored by the SeeMyCity composite rubric
**Coverage:** 204 of 213 local and metropolitan municipalities; 197 scored on FY2024, 7 on FY2023

---

## 1. Executive summary

South Africa's local government is not uniformly failing — it is **splitting in two**. A quarter of municipalities (56 of 204) score 70+ on a composite financial-health scale and would be respectable in any country. A fifth (45) score below 40 and are, on the evidence of their own audited numbers, in structural distress. The dividing line is not money, not size, and not urbanisation — it is **governance**: municipalities with clean audits average **77/100**; those that never submitted financials for audit average **30/100**.

Five findings stand out:

1. **Wealth doesn't predict health.** Revenue per capita has essentially **zero correlation with the overall score (r = −0.03)**. The second-best municipality in the country is Msinga, a poor rural KZN municipality — while metros with ten times its per-capita revenue score 20+ points below it.
2. **The Free State is a systemic failure, not a collection of individual ones.** Its *best* municipality (57.4) scores below the Western Cape's *median* (76.5). Four of the twelve worst municipalities in the country are Free State towns.
3. **More than half of municipalities (56%) spend more on operations than they collect in total revenue.** A quarter (54 municipalities) overspend by more than 15%. A third carry debt exceeding a full year's revenue.
4. **Broken books are the loudest signal in the data.** The municipalities with the most implausible numbers (negative debt, revenue a fraction of payroll) are precisely those with Disclaimer or Outstanding audit opinions. Data quality *is* the accountability metric.
5. **Durban is missing.** eThekwini — 4.2 million people, one of eight metros — has **no audited actuals at all** in the Treasury's data for FY2023–2025. The country's third-largest city is financially invisible in the national open-data record.

---

## 2. The data, briefly

- **Source:** National Treasury Municipal Money API — four OLAP cubes (income & expenditure, capital spend, financial position, audit opinions), filtered to audited actuals (AUDA).
- **Score:** weighted composite of four pillars — Financial Health 30% (debt-to-revenue + revenue per capita), Infrastructure Investment 25% (capex share of total spend), Operating Efficiency 25% (opex-to-revenue), Accountability 20% (audit outcome). Full rubric in `docs/prd.md`.
- **Missing-data policy:** a pillar without data gets no score, and a municipality without all four pillars gets **no overall score** — never a zero. Grey on the map means "unknown", not "bad".
- **Vintage:** FY2024 audited figures for 197 municipalities. FY2025 *financial* figures are already published for 169 municipalities, but **zero FY2025 audit opinions** exist yet — scores will roll forward automatically when the Auditor-General publishes (typically Nov–Dec).
- **Not covered:** 44 district municipalities (out of scope for now), and 9 municipalities with no audited data (see §8).

---

## 3. Headline numbers

| Metric | Value |
| --- | --- |
| Municipalities scored | **204 / 213** |
| Mean score | **55.9** |
| Median score | **55.6** |
| Population-weighted mean | **59.7** (bigger municipalities do slightly better) |
| Best | **Swartland (WC), 90.9** |
| Worst | **Kopanong (FS), 20.9** |
| Std deviation | 17.3 |

**Score bands:**

| Band | Count | Share |
| --- | --- | --- |
| Healthy (70–100) | 56 | 27% |
| Middling (40–69) | 103 | 51% |
| Distressed (< 40) | 45 | 22% |

Roughly **57.2 million people** live in the scored municipalities. The population-weighted mean (59.7) sitting above the unweighted mean (55.9) says the distress is concentrated in *smaller* municipalities — but §7 shows one important exception.

---

## 4. The four pillars: where municipalities actually fail

| Pillar | Weight | Mean | Median |
| --- | --- | --- | --- |
| Infrastructure investment | 25% | **83.2** | 89.0 |
| Accountability (audit) | 20% | **62.1** | 75.0 |
| Operating efficiency | 25% | **45.4** | 42.9 |
| Financial health | 30% | **37.7** | 37.8 |

The story is lopsided:

- **Financial health is the national weak point** (mean 37.7). Two forces: revenue per capita is low almost everywhere outside the metros (median around R3,000–R6,000 against a R14,000 top-of-scale), and 70 municipalities (34%) carry **debt greater than a year's total revenue**.
- **Operating efficiency is the second failure.** 114 of 204 municipalities (**56%**) spend more on operations alone than their entire revenue; 54 (**26%**) overspend by more than 15%. These are structural operating deficits, not rounding errors.
- **Infrastructure scores look remarkably good — treat with care.** 75 municipalities (37%) direct 30%+ of spending to capital works, and *not one* falls below the 5% floor. This almost certainly reflects **conditional infrastructure grants** (MIG and siblings) rather than locally-generated investment capacity: capital budgets arrive ring-fenced from national government even where operations are collapsing. The pillar is measuring grant flow as much as municipal virtue — a calibration point for v2 of the rubric (§9).
- **Accountability is bimodal**: a large group at 75 (unqualified with findings), a long tail at 0–25.

---

## 5. The provincial league

| Province | Munis | Mean | Median | Best | Worst |
| --- | --- | --- | --- | --- | --- |
| Western Cape | 24 | **72.9** | 76.5 | 90.9 | 48.8 |
| KwaZulu-Natal | 42 | **62.9** | 68.5 | 87.5 | 24.2 |
| Limpopo | 22 | **62.2** | 65.1 | 78.9 | 25.8 |
| Gauteng | 9 | 58.4 | 55.3 | 83.9 | 35.1 |
| Eastern Cape | 33 | 57.2 | 57.9 | 82.0 | 24.7 |
| Northern Cape | 23 | 46.4 | 43.9 | 76.8 | 25.5 |
| Mpumalanga | 17 | 45.2 | 44.2 | 71.7 | 31.3 |
| North West | 17 | 42.4 | 37.2 | 76.9 | 25.5 |
| Free State | 17 | **39.8** | 41.1 | **57.4** | 20.9 |

Three observations:

- **The 33-point Western Cape–Free State gap is the single largest fact in the dataset.** And it isn't driven by Cape Town: the WC *median* (76.5) is small-town municipalities like Swartland, Cape Agulhas and Hessequa.
- **The Free State has no good municipalities.** Its ceiling (57.4) is below five provinces' averages. Whatever is wrong there is provincial-systemic — shared administrative capacity, shared politics, shared collapse — not a few bad councils.
- **A distress belt runs through the interior**: Free State, North West, Mpumalanga, Northern Cape occupy the bottom four slots, all averaging under 47. The coastal/escarpment provinces (WC, KZN, Limpopo) hold the top three.

---

## 6. Audits: governance is the dividing line

| Audit outcome (FY2024) | Munis | Avg overall | Avg efficiency |
| --- | --- | --- | --- |
| Unqualified – no findings ("clean") | **26 (13%)** | **77.0** | 70.6 |
| Unqualified – emphasis of matter | 78 (38%) | 62.1 | 56.0 |
| Qualified | 77 (38%) | 48.9 | 35.0 |
| Adverse | 4 | 39.0 | 14.8 |
| Disclaimer | 11 | 35.8 | 15.4 |
| Outstanding (never submitted) | 8 | 30.1 | 16.8 |

- Only **13% of South African municipalities earn a clean audit**. Three-quarters sit in the "findings" or "qualified" middle.
- The audit outcome doesn't just contribute 20% of the score — it **predicts the other 80%**. Audit quality correlates with operating efficiency at r = 0.42: municipalities that can't keep books also can't keep budgets balanced. Clean-audit municipalities outscore disclaimer municipalities on *every* pillar.
- The step from "clean" to "emphasis of matter" costs 15 overall points on average; the step from Qualified to Disclaimer costs another 13. Audit categories are effectively a proxy ranking on their own.

---

## 7. Size: metros hold, secondary cities sag, small towns struggle

| Population band | Munis | Avg score | Median rev/capita |
| --- | --- | --- | --- |
| 1M+ (metros) | 5 | **64.9** | R12,697 |
| 500k–1M (secondary cities) | 14 | **50.9** | R4,891 |
| 100k–500k | 120 | 60.2 | R2,954 |
| < 100k | 65 | 48.4 | R5,940 |

- **The metros hold the line but do not lead.** Cape Town (75.7) is the only metro in the national top 30. Tshwane — the capital city — scores **55.3 with a *qualified* audit**, as does Nelson Mandela Bay (59.4). No metro is distressed; none except Cape Town is excellent.
- **The most troubled size class is the secondary city (500k–1M): average 50.9, the *worst* of all bands.** These are places like Matjhabeng (Welkom, 28.7) and Maluti-a-Phofung (QwaQwa, 23.0) — big enough to have metro-scale service obligations and payrolls, without metro-scale tax bases or skills. This band is where South Africa's municipal crisis is most concentrated *per capita*.
- Population size only mildly predicts score overall (r = 0.22 on log population). **Revenue per capita predicts nothing (r = −0.03)** — the strongest argument in the dataset that management, not money, separates outcomes.

**The five scored metros:**

| Metro | Score | Rev/capita | OpEx ratio | Debt ratio | Audit |
| --- | --- | --- | --- | --- | --- |
| Cape Town | 75.7 | R11,858 | 0.97 | 0.45 | Clean |
| Johannesburg | 70.2 | R14,817 | 0.86 | 0.75 | Emphasis of matter |
| Ekurhuleni | 64.0 | R12,697 | 1.00 | 0.54 | Emphasis of matter |
| Nelson Mandela Bay | 59.4 | R13,723 | 1.01 | 0.58 | Qualified |
| Tshwane | 55.3 | R11,308 | 0.93 | 0.69 | Qualified |

(eThekwini is absent — see §8.)

---

## 8. Leaders, laggards, and the missing

**Top 10:**

| # | Municipality | Province | Score | Audit |
| --- | --- | --- | --- | --- |
| 1 | Swartland | WC | 90.9 | Clean |
| 2 | **Msinga** | KZN | 87.5 | Clean |
| 3 | uMshwathi | KZN | 85.8 | Clean |
| 4 | Cape Agulhas | WC | 85.2 | Clean |
| 5 | Saldanha Bay | WC | 84.2 | Clean |
| 6 | Midvaal | GP | 83.9 | Clean |
| 7 | Dr. A.B. Xuma | EC | 82.0 | Emphasis |
| 8 | Hessequa | WC | 82.0 | Clean |
| 9 | Mbhashe | EC | 82.0 | Emphasis |
| 10 | Langeberg | WC | 81.9 | Clean |

**Bottom 10:**

| # | Municipality | Province | Score | Audit |
| --- | --- | --- | --- | --- |
| 204 | Kopanong | FS | 20.9 | Outstanding |
| 203 | Mohokare | FS | 22.6 | Outstanding |
| 202 | Maluti-a-Phofung | FS | 23.0 | Outstanding |
| 201 | Endumeni | KZN | 24.2 | Disclaimer |
| 200 | Sundays River Valley | EC | 24.7 | Disclaimer |
| 199 | Phokwane | NC | 25.5 | Outstanding |
| 198 | Maquassi Hills | NW | 25.5 | Adverse |
| 197 | Thabazimbi | LP | 25.8 | Disclaimer |
| 196 | Ditsobotla | NW | 26.9 | Disclaimer |
| 195 | Mamusa | NW | 27.1 | Adverse |

Every one of the bottom ten has a failed audit relationship (Outstanding/Disclaimer/Adverse). Not one of the top ten does.

**The nine with no score at all** — the most consequential being **eThekwini (Durban, 4.2M people)**, which has zero audited actuals in any Treasury cube for FY2023–2025. The other eight are small: Ramotshere Moiloa (NW), Nala (FS), !Kai! Garib (NC), Greater Kokstad (KZN), Matzikama (WC), Mafube (FS), Ubuntu (NC), Kareeberg (NC). Absence of data at this level is itself an accountability datapoint.

---

## 9. What's surprising

1. **Msinga.** One of the poorest rural municipalities in KZN is the country's #2. Perfect infrastructure and efficiency pillars, clean audit. Whatever its administration is doing deserves a case study — and it demolishes the assumption that poverty forces municipal failure. (Nkandla, of all places, is #12 at 81.0.)
2. **Rural Eastern Cape outperforming Gauteng.** Mbhashe, Dr. A.B. Xuma and Senqu — deep-rural EC — all score above 81, higher than every Gauteng municipality except Midvaal. A caveat applies: heavily grant-funded municipalities can post strong efficiency/capex ratios because the equitable-share transfer inflates revenue relative to their thin operations. The rubric should eventually distinguish own-revenue performance (see §10).
3. **Zero correlation between wealth and health** (r = −0.03). Money is not the differentiator; management is.
4. **The secondary-city trap.** The 500k–1M band scores *worse* than towns a tenth their size. The national conversation focuses on metros and small-town collapse; the data says the sharpest structural problem is one tier below the metros.
5. **The capital scores 55.** Tshwane, seat of national government, has a qualified audit and a middling score — outperformed by dozens of villages.
6. **Durban is a data black hole.** A metro of 4.2M with no auditable record in the national open-data platform for three consecutive years.
7. **The books of failing municipalities are literally unreadable.** Ratlou reported annual revenue of **R793,551** (less than a suburban house price) against R13M of operating spend — and *negative* debt. Gamagara shows R13.3bn in liabilities against R673M revenue. Both carry failed audit opinions; the garbage numbers and the audit findings are the same fact seen twice.

---

## 10. What should we do about it?

### Product (SeeMyCity)

1. **Ship the map with a "governance lens."** The audit outcome is the strongest single explanatory variable — offer it as an alternative map coloring alongside the composite score. It's also the easiest number for citizens to grasp: *"your town hasn't submitted its books for audit."*
2. **Add a data-confidence flag.** Where ratios are implausible (opex ratio > 3, debt ratio > 5, negative values, revenue below R10M for a 50k+ population), badge the municipality "figures unreliable — reflected in its audit outcome" rather than presenting artifacts as facts. Ratlou at 45.0 currently *overstates* its health because nonsense inputs partially cancel.
3. **Recalibrate the infrastructure pillar (rubric v2).** With a mean of 83 and a floor nobody hits, it barely discriminates and mostly measures grant flow. Options: raise thresholds, or replace with capex *per capita*, or net out conditional grants if the incexp cube allows it.
4. **Consider an own-revenue metric.** Grant dependency is the biggest known confounder (flattering rural municipalities' efficiency). The incexp cube distinguishes transfers (item 1600) from own revenue — a "self-sufficiency" sub-metric is one query away.
5. **Investigate the eThekwini gap** — check whether its data lives under different amount types (e.g. unaudited/adjusted) and, if so, show it with a clearly-labelled "unaudited figures" fallback rather than a blank.
6. **Build the trend view.** The cache already holds multi-year history (460 rows and growing); year-over-year deltas ("most improved", "fastest decline") are more newsworthy than levels.
7. **When FY2025 opinions land (Nov–Dec 2026), the dataset refreshes itself** — the warmer and TTL logic will roll everyone forward automatically. Plan a "2025 results" moment around it.

### For users of the data (citizens, journalists, analysts)

- Treat the **audit outcome as the headline** and the composite as texture.
- Watch the **secondary cities** — that's where service-delivery risk per resident is highest.
- Provincial averages justify **province-level intervention questions**, especially for the Free State, where the problem is clearly not municipality-specific.
- Don't read grant-inflated efficiency in poor rural municipalities as fiscal self-sufficiency.

---

## 11. Is there any value here?

**Yes — with honest boundaries.**

**What's genuinely valuable:**

- **A composite, comparable, always-current score does not otherwise exist in public.** The Auditor-General publishes annual outcomes (as reports and PDFs, not spatial data); Treasury's own portal exposes raw cubes but no synthesis; commercial indices (e.g. Ratings Afrika's MFSI) are paywalled. A free choropleth + drill-down over official audited data occupies a real gap.
- **The refresh mechanic is a moat.** Because the pipeline pulls from the live Treasury API with caching and self-healing scores, the product updates itself each audit season — most civic-data projects die precisely because they were one-off scrapes.
- **Clear audiences:** local journalists (province/town league tables are ready-made stories), civil-society accountability groups, municipal bond/credit analysts wanting a screening layer, property and relocation decisions, councils benchmarking neighbours, and researchers (the Msinga question alone is a paper).
- **The dataset supports an annual flagship artifact** — a "State of Local Government Finances" report (this document is effectively its pilot).

**Honest limitations to state wherever the data is shown:**

- The composite reflects *our* weights; different weights reorder mid-table municipalities (the extremes are robust).
- One year's snapshot; audited data lags ~18 months by nature.
- Debt = total liabilities (includes e.g. provisions), revenue includes grants — ratios are indicative, not credit-grade.
- Population denominators are dated census figures; per-capita numbers drift accordingly.
- Garbage-in risk is real at the bottom of the table — which is why the audit outcome must always be shown beside any number.

**Bottom line:** the data supports a defensible, differentiated public product whose strongest insight — *governance, not money, separates working towns from failing ones* — is legible to any citizen, and checkable by anyone against the official sources.

---

## Appendix: reproduction

- Scoring rubric: `docs/prd.md` §Scoring System; implementation `seemycity-backend/src/scoring.rs` (16 unit tests).
- All figures derive from the `financial_data` cache joined to `municipalities`, latest scored year per municipality (197× FY2024, 7× FY2023), queried 2026-07-07.
- Analysis queries: aggregate statistics, provincial group-bys, audit-outcome group-bys, population-band group-bys, and Pearson correlations computed in PostgreSQL 17 (`corr()`, `percentile_cont()`).
- Fiscal-stress definitions: deficit = opex > revenue; severe = opex > 1.15 × revenue; over-indebted = total liabilities > annual revenue.
