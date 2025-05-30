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

**Future/Deferred:**  
- [ ] Advanced accessibility audits  
- [ ] Cross-browser and device testing  
- [ ] Deep performance optimization  
- [ ] Province/district drill-down and aggregation

---