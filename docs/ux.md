### Revised `ux.md` - User Experience Document

#### Overview
**Product Name**: Municipal Financial Dashboard  
**Purpose**: The Municipal Financial Dashboard is a warm, approachable web app that invites users to explore South African municipal financial health. With a friendly, community-driven vibe, it sparks curiosity (“Wow, I didn’t know this!”) and inspires action, acting as a helpful guide saying, “Let’s explore this together.”

**Goals**:  
- Present financial data in a cozy, engaging way that feels tied to South African pride.  
- Encourage exploration with playful interactions and clear visuals.  
- Build trust through transparent breakdowns and a welcoming tone.

**Target Audience**: Citizens, investors, policymakers seeking friendly insights.

---

#### Design Principles
1. **Warmth**: Earthy tones and rounded edges create a community feel.  
2. **Curiosity**: Pulsing elements and animations nudge users to dig deeper.  
3. **Approachability**: Simple layouts and friendly text lower barriers.  
4. **Pride**: SA-inspired colors and fonts celebrate local identity.

---

#### Visual Design

##### Color Scheme
- **Primary**:  
  - Green (#2E8B57): High scores—lush, vibrant SA greenery.  
  - Orange (#F28C38): Medium scores—warm SA sunset glow.  
  - Red (#CD5C5C): Low scores—soft, earthy SA soil.  
- **Neutral**:  
  - Cream (#FDF6E3): Background—cozy, inviting base.  
  - Charcoal (#3C2F2F): Text/icons—rich and grounding.  
- **Accent**:  
  - Teal (#008080): Buttons/links—SA coastal spirit.  
- **Usage**: Map gradient (red → orange → green); teal for calls-to-action.

##### Typography
- **Font**: Ubuntu (SA-designed, open-source).  
  - Headings: Bold, 24px (views), 18px (subsections)—friendly authority.  
  - Body: Regular, 16px (metrics), 14px (details)—soft and clear.  
- **Contrast**: Charcoal on cream (light mode); cream on charcoal (dark mode).

##### Icons
- **Source**: Iconify (`@iconify/svelte`).  
- **Examples**: 💰 (revenue), 🏡 (infrastructure), ⚖️ (efficiency), 🌟 (audit).  
- **Style**: 20px, teal or metric-matched (e.g., green for audit “Clean”).

##### Animations
- **Transitions**: Svelte `fade` (200ms) for views; `slide` for accordions.  
- **Micro-Interactions**: Pulsing score badge on load (200ms scale), hover glow on map regions.

---

#### User Interface Layouts

##### 1. Map View
**Purpose**: Welcome users with a warm, curious peek at municipal scores.  
**Layout**:  
```
-----------------------------------------------------
|                                   [Dark Mode 🌙]  |
-----------------------------------------------------
|                                                   |
|   [Choropleth Map: Red → Orange → Green]          |
|   - Tooltip: "Cape Town’s at 84—pretty solid!"    |
|   - Pulse on hover                                |
|                                                   |
-----------------------------------------------------
| [Reset Zoom 🔍]                                   |
-----------------------------------------------------
| [KeyMetricsGrid]                                  |
-----------------------------------------------------
```
- **Interactions**:  
  - Hover: Pulsing tooltip with friendly text.  
  - Click: Fade to Single View.  
  - Zoom: Smooth scale, scores emerge with a gentle bounce.  
- **Vibe**: “Wow, look at this map—let’s dive in!” (MVP focuses on municipality view; province/district drill-down is a future enhancement).

##### 2. Single View (Municipality Profile)
**Purpose**: Share a municipality’s story in a cozy, approachable way.  
**Layout**:  
```
-----------------------------------------------------
| Cape Town            | Score: 84/100    | 🟢       | Website: [🔗]
-----------------------------------------------------
| Province: WC    Pop: 4.7M                          |
-----------------------------------------------------
| [💰 Rev/Cap: R6,200] [🏡 CapEx: 18%] [📊 Debt: R12bn]|
| [⚖️ OpEx: 82%]        [🌟 Audit: Clean]           |
-----------------------------------------------------
| [ScoreBreakdown]                                  |
-----------------------------------------------------
| [What’s behind this score? ▼]                     |
|  - **Financial Health (30%):** [||||||||--] 88/100 |
|    * Debt Ratio: 0.45 | Revenue/Capita: R6,200    |
|  - **Infrastructure Investment (25%):** [|||||||---] 75/100 |
|    * CapEx Ratio: 18%                              |
|  - **Efficiency & Service Delivery (25%):** [||||||----] 70/100 |
|    * OpEx Ratio: 82%                               |
|  - **Accountability (20%):** [||||||||||] 100/100 |
|    * Audit Outcome: Unqualified - No findings    |
-----------------------------------------------------
| [Compare ➕]              [Refresh 🔄]             |
-----------------------------------------------------
```
- **Interactions**:  
  - Score Badge: Pulses on load (200ms).  
  - Expand Breakdown: Slides open showing pillar scores with progress bars and contributing metrics.
  - Compare/Refresh: Teal buttons with hover bounce.  
- **Vibe**: “Cape Town’s doing great—let’s see why!”

##### 3. Comparison View
**Purpose**: Spark curiosity with a warm, side-by-side look using `ComparisonContainer` and `ComparisonCard`.  
 **Layout**:  
```
-----------------------------------------------------
| [ComparisonCard: Cape Town] | [ComparisonCard: Joburg] |
|-----------------------------|--------------------------|
| **Cape Town** (GP)          | **Johannesburg** (WC)    |
| Pop: 4.7M                   | Pop: 5.6M                |
| Score: 84 🟢               | Score: 57 🟡            |
|-----------------------------|--------------------------|
| 💰 Rev/Cap: R6,200          | 💰 Rev/Cap: R3,100       |
| 🏡 CapEx: 18%             | 🏡 CapEx: 12%          |
| ⚖️ OpEx: 82%             | ⚖️ OpEx: 95%         |
| 📊 Debt: R12bn            | 📊 Debt: R25bn         |
| 🌟 Audit: Clean           | 🌟 Audit: Qualified    |
-----------------------------------------------------
|                      [Add Another ➕]            |
-----------------------------------------------------
```
- **Interactions**:  
  - Add: Teal button returns to map/selection. 
  - (Future) Highlight: Winning metric gets a soft tint.  
- **Vibe**: “Let’s see how they compare—cool, right?”

---

#### User Flow
1. **Entry**: Map View loads with a warm “Hey, check out SA’s municipalities!” feel.  
2. **Exploration**: Hover tooltips pulse, click “Cape Town” → Single View fades in.  
3. **Detail**: Score pulses, breakdown expands—“Wow, I didn’t know this!”  
4. **Comparison**: Add “Johannesburg,” swap playfully—“Let’s mix it up!”  
5. **Refresh**: Teal button triggers “Data refreshed—nice!” toast.

---

#### Engagement Features
- **Pulsing Badges**: Scores animate on load—eye-catching and fun.  
- **Friendly Text**: Tooltips and breakdowns chat like a guide (e.g., “Top marks!”).  
- **Warm Map**: Earthy tones feel like home, inviting exploration.  
- **Teal Accents**: Playful buttons nudge action—“Let’s compare!”  

---

#### Accessibility
- **Contrast**: Charcoal on cream (4.5:1 ratio).  
- **Keyboard**: Tab through map, buttons, dropdowns.  
- **Screen Readers**: “Cape Town, score 84, good performance.”  
- **Responsive**: Stack vertically on mobile, map shrinks gracefully.

---

#### Assumptions
- Users love a friendly nudge over sterile data.  
- Warm tones outweigh sleek minimalism for this audience.  
- Pulse animations won’t overwhelm—tested for subtlety.

#### Risks
- **Tone**: Too casual might undercut trust—keep breakdowns factual.  
- **Mobile**: Warm map needs clear boundaries—test small screens.  
- **Load**: Animations mustn’t lag—optimize for speed.

---
