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
**Status:** _Planned_  
**Tasks:**  
- [ ] Remove all Tailwind remnants and use only SCSS/CSS  
- [ ] Define a global theme SCSS file for colors, spacing, and typography  
- [ ] Redesign components: cards, metric rows, badges, buttons, and layout  
- [ ] Apply consistent Ubuntu font and heading hierarchy  
- [ ] Integrate Iconify icons per metric and audit outcome  
- [ ] Add Svelte transitions (fade, slide) and pulse/hover effects  
- [ ] Style the map with a red-orange-green gradient and animated tooltips  
- [ ] Ensure basic accessibility and semantic HTML  
- [ ] (Optional) Add dark mode toggle  
**Milestone:** Visually inviting, consistent, and playful UI across all major views

---

**Future/Deferred:**  
- [ ] Advanced accessibility audits  
- [ ] Cross-browser and device testing  
- [ ] Deep performance optimization  
- [ ] Province/district drill-down and aggregation

---