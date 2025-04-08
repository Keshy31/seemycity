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

**Date**: April 08, 2025 (aligned with current context).

---

#### Key Requirements

##### 1. Functional Requirements
1. **Data Source**:
   - Fetch financial data from the Municipal Money API (http://municipaldata.treasury.gov.za/api) for the latest year (e.g., 2024).
   - Incorporate static population and GeoJSON data from external sources (e.g., StatsSA, OpenStreetMap) for per-capita metrics and map visualization.
2. **Scoring System**:
   - Calculate a composite score (0-100) for each municipality based on four pillars:
     - **Financial Health (30%)**: Revenue per capita, debt-to-revenue ratio.
     - **Infrastructure Investment (25%)**: Capital expenditure as a percentage of total expenditure.
     - **Efficiency & Service Delivery (25%)**: Operating expenditure ratio.
     - **Accountability (20%)**: Audit outcome (e.g., Clean = 100, Qualified = 50, Adverse = 0).
   - Normalize metrics and weight them to produce a transparent, comparable score.
3. **Views**:
   - **Map View**: Display municipalities on a choropleth map, color-coded by score, with province filtering.
   - **Single View**: Show a selected municipality’s score, metrics, and score breakdown.
   - **Comparison View**: Present side-by-side metrics and scores for multiple municipalities.
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

---