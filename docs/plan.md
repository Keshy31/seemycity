### plan.md - Development Plan

#### Overview
**Product Name:** Municipal Financial Dashboard  
**Purpose:** This plan outlines the step-by-step development of the SeeMyCity dashboard, prioritizing rapid deployment, robust data integration, and a warm, engaging user experience.

---

#### Phases and Tasks

**Phase 1: Initial Frontend Setup & Deployment**  
**Goal:** Deploy a minimal frontend to Fly.io for early feedback and visibility.  
**Status:** _Completed_  
**Tasks:**  
- [x] Initialize SvelteKit project  
- [x] Set up basic routing (`/`, `/[id]`)  
- [x] Configure static adapter and Dockerfile  
- [x] Deploy to Fly.io  
- [x] Apply global Ubuntu font, base colors, and SCSS setup  
**Milestone:** Basic web version live

---

**Phase 2: Database Setup on Fly.io**  
**Goal:** Establish a Postgres database (with PostGIS) for core data storage.  
**Status:** _Completed_  
**Tasks:**  
- [x] Provision Postgres on Fly.io  
- [x] Create schema for municipalities, geometries, financial data  
- [x] Import GeoJSON, population, and municipality info  
- [ ] (Deferred) District/province associations  
**Milestone:** Database ready and populated

---

**Phase 3: Frontend Expansion (MVP)**  
**Goal:** Build core UI views using dummy data for rapid prototyping.  
**Status:** _Completed_  
**Tasks:**  
- [x] Map view with MapLibre GL JS  
- [x] Clickable popups and navigation  
- [x] Single and comparison views for municipalities  
- [x] Dummy data store and iterative UI enhancements  
**Milestone:** Full UI prototype with dummy data

---

**Phase 4: Backend Development**  
**Goal:** Build the Rust backend (Actix Web, sqlx) and integrate the Municipal Money API.  
**Status:** _Completed_  
**Tasks:**  
- [x] Set up Rust project and dependencies  
- [x] Implement modular backend structure  
- [x] Connect to Postgres, define data models  
- [x] Integrate Municipal Money API client  
- [x] Implement API endpoints and caching  
- [ ] (Deferred) District/province aggregation endpoints  
**Milestone:** Backend API operational

---

**Phase 5: Frontend-Backend Integration**  
**Goal:** Connect the SvelteKit frontend to the Rust API, replacing dummy data with live data.  
**Status:** _Completed_  
**Tasks:**  
- [x] Fetch and display real GeoJSON and financial data  
- [x] Color map regions by score  
- [x] Update single and comparison views with live metrics  
- [x] Refactor data fields for consistency  
**Milestone:** Full data integration

---

**Phase 6: UI/UX Overhaul – Visual & Interaction Upgrade**  
**Goal:** Transform the dashboard to match the warm, SA-inspired, playful, and approachable vision in the UX doc, while preserving all data and backend functionality.  
**Status:** _In Progress_  
**Tasks:**  
- [x] **1. Setup & Cleanup:** Remove Tailwind CSS configuration, dependencies, and utility classes. (Status: Done)
- [x] **2. Single View Refinement & Cleanup:** Corrected metric calculations (OpEx % in KeyMetricsGrid, OpEx Ratio in ScoreBreakdown), consolidated year display in PageHeader, updated docs, replaced Sass `@import` with `@use`, fixed TS null errors. (Status: Done)
- [x] **2. Global SCSS Foundation:** Set up global SCSS files (`variables.scss`, `base.scss`, `typography.scss`), define variables, apply base styles. (Status: Done)
- [x] **3. Refactor Page Components:**
   - Break down large page Svelte components (`[id]/+page.svelte`, `compare/[ids]/+page.svelte`) into smaller, reusable sub-components.
   - [x] **Single View Components:** Implement/refactor `PageHeader` (with overall score), `KeyMetricsGrid` (for high-level metrics), and `ScoreBreakdown` (to clearly show pillar scores, weights, and contributing metrics per PRD/UX docs).
     - *Progress on ScoreBreakdown:* Added dynamic score coloring, progress bars, and fixed audit outcome display inconsistencies.
- [ ] **4. Component Styling:** Restyle core UI components (`Card`, `Button`, `MetricDisplay`, `ScoreBadge`, map tooltips etc.) using SCSS variables, semantic classes, and Iconify icons.
  - **Scope:** Apply a full aesthetic redesign, similar to the `ComparisonCard` process, to **all** UI elements.
  - **Approach:** Work top-down from page layouts (Map, Single, Compare) to container components, and finally to atomic elements (Cards, Buttons, Metric Displays, Badges, Tooltips, etc.).
  - **Goal:** Ensure visual consistency, alignment with `ux.md`, and a high-quality aesthetic throughout the application using SCSS variables, semantic classes, and Iconify icons.
- [ ] **5. View Layout Refinement:** Adjust Map, Single, and Comparison view layouts (CSS Grid/Flexbox) to match `ux.md` sketches.
- [ ] **6. Map Styling:** Configure MapLibre choropleth color ramp (Red -> Orange -> Green) based on `overall_score`; style map interactions.
- [ ] **7. Interactions & Animations:** Implement Svelte transitions (`fade`, `slide`), CSS hover effects, and score badge pulse animation (`ux.md`).
- [ ] **8. Accessibility Check:** Perform basic review (contrast, keyboard navigation, semantic HTML/ARIA).
- [ ] **9. Review & Commit:** Final visual review, responsiveness check, code cleanup, Git commit.
**Milestone:** Visually inviting, consistent, and playful UI across all major views

---

**Phase 7: 2026 Revival — Correctness, Speed, Resilience, Refresh**
**Goal:** Bring the dormant 2025 MVP to a trustworthy, fast, deployable state.
**Status:** _Completed (July 2026; commits `00dcfda`..`4542da2`)_
**Delivered:**
- [x] Scoring correctness: missing data scores NULL (never 0), real Treasury audit-label matching, efficiency midpoint fixed, 16 unit tests, 2-dp rounding aligned with storage
- [x] Cache & year policy: latest-scorable-year walk (no hardcoded year), 7-day TTL, negative caching, lazy score healing on formula change
- [x] Treasury-outage resilience: 10s timeouts, transport failures never persisted, 5-minute circuit breaker
- [x] API contract unified on `overall_score`; map coloring, navigation, and null-score crashes fixed
- [x] Performance: map payload 18 MB → ~305 KB gzipped (ST_SimplifyPreserveTopology + 5-dp coords), 60s in-memory map cache (~15-30 ms warm in release), gzip middleware, deduped incexp fetch (4 upstream calls/year, not 5)
- [x] Reproducibility: migrations/, .sqlx offline compile data, .env.example files, all pipelines green (cargo 0 warnings, svelte-check 0/0, prettier+eslint+vitest pass)
- [x] Deploy path fixed & locally verified (SPA fallback + nginx + env injection; backend Dockerfile + fly.toml prepared — **not deployed by decision**)
- [x] Modern visual refresh: new token sheet (stone neutrals, teal primary, emerald/amber/red score palette), Inter, token-driven MapLibre ramp, responsive map view; ux.md rewritten
- [x] Background cache warmer (startup + daily, `CACHE_WARMER` env): 204/213 municipalities scored
- [x] Data insights report: `docs/data-insights-2026-07.md`

---

**Phase 8: "Consultant's Dream" — Trust, Scoring v2, Insight UI**
**Goal:** Tight data, effective scoring, striking visuals — transparency that can drive change. Primary audience: journalists & civil society.
**Status:** _In progress (started 2026-07-07)_

**Locked decisions (2026-07-07):**
1. **Scoring philosophy:** hybrid — absolute anchors (break-even = 50, clean audit = 100) with range endpoints calibrated annually against the observed national distribution.
2. **New metrics:** own-revenue split (transfers item **2200**), UIFW (`uifwexp` cube), repairs & maintenance (`repmaint_v2` cube) — validating with sample data at every step, not at the end.
3. **Map:** 5 quantized score bands + legend with counts (not a continuous ramp).
4. **UI priority:** journalists & civil society (league tables, shareable dashboard, plain-English verdicts, permalinks).

**Sub-phase A — Trust the numbers:**
- [x] Probe cubes with sample munis (CPT / KZN244 Msinga / FS184 Matjhabeng) — see findings below
- [x] **Re-pin revenue/opex item sets** — done 2026-07-07: revenue = 0200-2800, expenditure = 3000-4300, validated against CPT's audited FY2024 AFS (+0.6% / +0.4% on the mSCOA basis). Item 2900 proven to be a mislabeled **total-revenue rollup** (identity holds at 0.00% across 8 test municipalities) — excluded, and available as a checksum for the confidence layer. Requires a one-time financial_data cache wipe + re-warm so all stored raw figures use the corrected definitions.
- [x] Data-confidence layer — done 2026-07-08: `src/confidence.rs` grades every muni-year `ok`/`suspect`/`unreliable` (negatives, implausible ratios, revenue-vs-population sanity, one-sided statements, and the item-2900 revenue checksum at fetch time); grades + human-readable notes stored via migration 0002, backfilled by the healing pass, served in the API, and rendered as a plain-English badge on the detail page. Empty-but-200 upstream responses can no longer overwrite real cached data. First catch: Msinga (#2 nationally) reports negative debt — its financial-health pillar is inflated; flagged for scoring v2 (consider nulling pillars fed by unreliable inputs). Current census: 388 ok / 6 suspect / 4 unreliable.
- [ ] Own-revenue split fetched and stored (item 2200 transfers)
- [x] Census 2022 population check — verified 2026-07-08: the stored populations already ARE Census 2022 figures (JHB 4,803,262 / CPT 4,772,846 / ETH 4,239,901 match StatsSA exactly; ETH off by 1 from the `real` column's f32 rounding — nit only). No refresh needed until the next census.
- [x] eThekwini mystery solved: Treasury HAS its FY2024 audited data (R170bn incexp); the 9 unscored munis are false negatives from empty-but-200 responses during the 2026-07-07 Treasury degradation → purge + re-warm

**Sub-phase B — Scoring engine v2:**
- [ ] Pillar redesign: financial health = debt ratio + **own-revenue** strength; efficiency = operating balance vs own revenue; infrastructure = capex per capita or grant-netted (+ R&M vs Treasury 8% norm); accountability = audit outcome + **UIFW** intensity
- [ ] `score_version` column; healing/warmer migrate old rows automatically
- [ ] Backtest harness: score all munis old-vs-new, diff rankings, review before shipping
- [ ] Update prd.md rubric to v2 when it lands

**Sub-phase C — Insight UI (journalists first):**
- [ ] 5-band quantized map + legend with per-band counts
- [ ] Map lens switcher: composite / audit outcome / per-pillar / (later) trend
- [ ] Hover tooltips: name, score, one-line plain-English verdict
- [ ] Detail page as diagnosis: rank badge (#n of 204), pillar waterfall, peer comparison vs province & size-band medians
- [ ] National dashboard page (live version of the insights report: bands, provincial league, movers)
- [ ] Compare entry point (button exists but was never wired)
- [ ] Data-confidence badge on detail + map hatching for low-confidence

**Sub-phase D — Reach:**
- [ ] Deploy to Fly (backend app creation + secrets + both deploys — awaiting go decision)
- [ ] "FY2025 results day" (Nov-Dec 2026 when AG publishes; pipeline rolls forward automatically)
- [ ] Annual "State of Local Government Finances" report from the live data

**Key probe findings (2026-07-07, must inform all Phase 8 work):**
- Real operational-transfers item is **2200** ("Transfer and subsidies - Operational"); item 1600 is generic "Operational Revenue" — older docs saying 1600 = transfers are wrong.
- `uifwexp` cube: keyed by `financial_year_end.year` + `item` (unauthorised / irregular / fruitless), no amount_type. Validated: Matjhabeng R1.4bn total UIFW FY2024 vs CPT R634m irregular-only vs Msinga R12m.
- `repmaint_v2`: standard AUDA/financial_period shape, works.
- Treasury can return **empty-but-HTTP-200** responses while degraded; indistinguishable from "no data" without heuristics. Negative caching must be confidence-aware.

---

**Future/Deferred:**  
- [ ] District/province drill-down and aggregation
- [ ] Collection rates from `aged_debtor_v2`; liquidity from `cflow_v2` (scoring v3 candidates)
- [ ] Dark mode (token architecture ready)
- [ ] Advanced accessibility audits, cross-browser testing
- [ ] Self-hosted fonts

---