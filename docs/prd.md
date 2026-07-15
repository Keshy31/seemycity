### `prd.md` - Product Requirements Document

#### Overview
**Product Name**: Municipal Financial Dashboard  
**Purpose**: The Municipal Financial Dashboard is a web application designed to provide transparent, actionable insights into the financial health of South African municipalities. It empowers citizens, investors, and policymakers with a clear, engaging view of municipal performance through a composite score, visualized on an interactive map and detailed in single and comparison views.

**Objective**:  
- Deliver a simple, intuitive tool to assess municipal financial performance using open data from the Municipal Money API.
- Enable users to explore financial metrics, compare municipalities, and understand governance effectiveness without requiring technical expertise.
- Lay the groundwork for a future "SeeMyCity" app by proving a high-performance, scalable data platform.

**Target Audience**:  
- **Citizens**: Seeking transparency and accountability from local governments.  
- **Investors**: Evaluating municipal stability for funding or development decisions.  
- **Policymakers**: Monitoring and optimizing municipal performance.

**Date**: April 26, 2025

> **Vision addendum (July 2026):** the product should read like a management
> consultant's diagnosis without the jargon — tight data, effective scoring,
> striking and insightful visuals — bringing transparency to local government
> and, through it, pressure for change. **Primary audience for the current
> phase: journalists and civil society** (league tables, shareable dashboards,
> plain-English verdicts). The evidence base and headline findings live in
> `docs/data-insights-2026-07.md`; the committed roadmap and locked design
> decisions (hybrid scoring, own-revenue/UIFW/R&M metrics, 5-band map) live in
> `docs/plan.md` **Phase 8**.

---

#### Key Requirements

##### 1. Functional Requirements
1. **Data Source**:
   - Fetch financial data from the Municipal Money API (http://municipaldata.treasury.gov.za/api) for the latest year (e.g., 2024).
   - Incorporate static population data from external sources (e.g., StatsSA) and GeoJSON boundaries from the [Municipal Demarcation Board ArcGIS Hub](https://spatialhub-mdb-sa.opendata.arcgis.com/) for per-capita metrics and map visualization.
2. **Scoring System v2** *(rubric matches `seemycity-backend/src/scoring.rs` (`SCORE_VERSION = 2`, July 2026) — the canonical source. Philosophy: hybrid — absolute anchors where they exist (break-even = 50, clean audit = 100), range endpoints reviewed annually against the observed national distribution. v2 anchors are provisional until the full-cache backtest lands.)*:
   - Calculate a composite score (0-100) for each municipality based on four weighted pillars, detailed below.
   - **Missing data policy**: a pillar whose inputs are missing or invalid (NULL, zero denominator) has **no score** (NULL) — it is *not* scored 0. The overall score exists only when **all four** pillars could be computed; otherwise it is NULL and the UI shows "no data" (grey on the map). "No data" must never be indistinguishable from "worst".
   - **Data-reliability policy**: when the confidence layer grades a year's figures `unreliable`, the three pillars derived from those figures (Financial Health, Infrastructure, Efficiency) are suppressed to NULL — artifacts like negative debt must not earn perfect sub-scores. The Accountability pillar survives: the AG's opinion is a statement *about* the books, not a product of them.
   - **Accountability (20% weight)** — audit outcome, blended with UIFW when reported:
     - Sub-metric 1 — Audit Outcome (string from `financial_data.audit_outcome`). Label matching is case-insensitive and covers the real Treasury/Auditor-General variants (e.g. "Unqualified opinion with no findings", "Disclaimer of opinion").
       - Unqualified, no findings: **100**
       - Unqualified with findings / emphasis of matter / financially unqualified: **75**
       - Qualified: **50**
       - Adverse, Disclaimer: **25**
       - Outstanding / statements not submitted: **0** (an earned zero — the municipality failed to submit)
       - NULL or unrecognized label: **no score** (treated as missing data, not failure)
     - Sub-metric 2 — UIFW intensity (`uifw_expenditure / operational_expenditure`, from the `uifwexp` cube): linear from **100** at 0% down to **0** at ≥ 10% of opex.
     - Pillar: audit sub-score alone when UIFW is unreported; `0.7 * audit + 0.3 * uifw` when reported. (No UIFW facts usually means none was identified, but it is treated as *unknown*, never as an earned 100.)
   - **Infrastructure Investment (25% weight)** — building new assets AND maintaining existing ones:
     - Sub-metric 1 — CapEx share (`capital_expenditure / (operational_expenditure + capital_expenditure)`): piecewise linear — 0 at ratio 0.00, 50 at 0.10, 100 at ≥ 0.30.
     - Sub-metric 2 — Repairs & maintenance intensity (`repairs_maintenance / operational_expenditure`, from `repmaint_v2`): linear from 0 at 0% up to **100** at ≥ 8% of opex (proxy for Treasury's 8%-of-asset-value norm; asset registers are out of scope).
     - Pillar: capex sub-score alone when R&M is unreported; `0.7 * capex + 0.3 * rm` when reported.
   - **Efficiency & Service Delivery (25% weight)**:
     - Metric: Operational Expenditure Ratio (`OpEx Ratio = operational_expenditure / revenue`).
     - Scoring (0-100): linear from Score 100 at Ratio <= 0.85 down to Score 0 at Ratio >= 1.15, which puts break-even (Ratio 1.0) at exactly 50.
   - **Financial Health (30% weight)** — self-sufficiency + solvency, averaged:
     - Sub-metric 1 — Own-revenue share (`1 - transfers_operational / revenue`, transfers = incexp item 2200): linear from **0** at share ≤ 0.25 (grant-dependent) up to **100** at share ≥ 0.75 (self-funded). *Replaces v1's revenue-per-capita, which measured urbanity, not health (r ≈ 0 with the overall score across 208 munis).* No population input needed.
     - Sub-metric 2 — Debt-to-Revenue Ratio (`Debt Ratio = debt / revenue`): normalize on [0.1, 1.0], lower is better. `Debt Score = 100 * (1 - max(0, min(1, (Debt Ratio - 0.1) / (1.0 - 0.1))))`.
     - Pillar Score (0-100): `Score = (Own-Revenue Score * 0.5) + (Debt Score * 0.5)`.
   - **Overall Score (0-100)**:
     - Metric: Weighted average of the four pillar scores (all four must be present).
     - Scoring: `Overall = (Accountability Score * 0.20) + (Infrastructure Score * 0.25) + (Efficiency Score * 0.25) + (Financial Health Score * 0.30)`, rounded to 2 decimal places.
   - **Versioning**: every scored row stores `score_version`; the lazy healing pass re-derives rows stamped with an older version from their stored raw inputs, migrating the whole cache without upstream calls.
3. **Views**:
   - **Map View**: Display municipalities on a choropleth map, color-coded by the `Overall Score`. Users can click a municipality to navigate to its Single View. (Province/District level views are post-MVP).
   - **Single View**: Show a selected municipality’s `Overall Score`, key metrics, and the breakdown of the four pillar scores.
   - **Comparison View**: Present side-by-side metrics and scores (including `Overall Score`) for multiple municipalities.
4. **Data Storage**:
   - Cache API data in a local Postgres database, updated quarterly or on manual refresh.
   - Pre-populate municipality details (name, province, population, GeoJSON) for map and per-capita calculations.
5. **User Interactions**:
   - Click a municipality on the map to view its single profile.
   - Add municipalities from the single view to a comparison table.
   - Refresh data manually to update from the API.

##### 2. Non-Functional Requirements
- **Performance**: Load map and initial data in under 2 seconds (assuming cached data); API fetches under 5 seconds.
- **Scalability**: Handle data for all South African municipalities (~257) with room to grow.
- **Usability**: Intuitive design requiring no training; mobile-friendly layout.
- **Reliability**: Graceful fallback to cached data if API is unavailable.
- **Engagement**: Visually appealing presentation with subtle animations and clear feedback.

##### 3. Constraints
- **Scope**: Limited to latest financial snapshot (no historical trends for MVP).
- **Data**: Dependent on Municipal Money API availability; population and GeoJSON sourced manually for MVP.
- **Features**: No user accounts, export functionality, or advanced analytics in MVP.

##### 4. Success Metrics
- **Adoption**: 100 users exploring at least 3 municipalities within 30 days of launch.
- **Engagement**: Average session time > 2 minutes; 50% of users interact with comparison view.
- **Performance**: 95% of page loads complete within target times.

---

#### User Stories
1. **As a citizen**, I want to see a map of municipal scores so I can quickly identify how my area performs financially.
2. **As an investor**, I want to view detailed financial metrics for a municipality so I can assess its stability for investment.
3. **As a policymaker**, I want to compare two municipalities side-by-side so I can prioritize resource allocation.
4. **As a user**, I want to understand how a score is calculated so I can trust the data presented.

---

#### Assumptions
- Municipal Money API provides consistent, structured data (revenue, expenditure, etc.).
- Users have basic internet access for initial data loads; cached data supports offline use.
- Population and GeoJSON data can be sourced and aligned with API municipality IDs.

#### Risks
- **API Downtime**: Mitigated by caching in Postgres.
- **Data Gaps**: Missing metrics (e.g., service backlogs) may limit pillar accuracy—display as “Unavailable.”
- **Sourcing Effort**: Manual population/GeoJSON collection may delay setup—prioritize key municipalities (e.g., Cape Town, Johannesburg).

#### Future Considerations
- Expand to historical trends (5-year view).
- Integrate satellite/edge data (e.g., air quality) for a "SeeMyCity" evolution.
- Add exportable reports or user customization.
- **Implement Hierarchical Map Navigation**: Allow users to view aggregated scores and navigate from Province -> District -> Municipality levels.

---