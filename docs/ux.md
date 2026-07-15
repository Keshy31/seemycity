# `ux.md` — User Experience & Visual Design

**Product**: SeeMyCity — Municipal Financial Dashboard
**Last major revision**: July 2026 ("modern refresh" — replaces the 2025 warm/playful direction)

## Overview

SeeMyCity presents the financial health of South African municipalities with the
clarity of a modern data product: quiet neutrals, one confident accent, and a
score palette that does the talking. The interface should feel trustworthy and
fast — the data is the personality.

## Design principles

1. **Data first.** Color is reserved for meaning (scores, audit outcomes,
   status). Everything else stays neutral.
2. **Honest about gaps.** "No data" renders grey — visually distinct from a low
   score, never conflated with failure.
3. **Fast and calm.** Sub-200ms micro-transitions, no decorative animation.
   `prefers-reduced-motion` is respected globally.
4. **One system.** Every color, size, and radius comes from the token sheet
   (`src/styles/_variables.scss`). The MapLibre choropleth reads the same
   tokens at runtime, so the map can never drift from the UI.

## Visual language

### Color

| Role | Token | Value |
| --- | --- | --- |
| App background | `--background-color` | `#f7f6f3` warm stone |
| Surfaces (cards) | `--surface-color` | `#ffffff` |
| Text | `--text-color` | `#1c1917` |
| Muted text | `--text-muted-color` | `#6f6a64` |
| Primary / actions | `--primary-color` | `#0f766e` teal |
| Score high (≥70) | `--score-high-color` | `#059669` emerald |
| Score medium (≥40) | `--score-medium-color` | `#d97706` amber |
| Score low (<40) | `--score-low-color` | `#dc2626` red |
| Score missing | `--score-none-color` | `#d6d3d1` grey |

Audit outcomes: clean = emerald, emphasis-of-matter = sky, qualified = amber,
adverse/disclaimer = red, outstanding = grey.

The map ramp interpolates low → medium → high across 0/40/70/100 using the same
tokens (read from `:root` in `MapComponent.svelte`).

### Typography

- **Inter** (Google Fonts, 300–700 variable range) with a system fallback.
- Headings: semibold/bold, tight tracking (`-0.015em`), ~1.25 scale
  (36 / 28 / 22 / 18 px).
- Body 16px / 1.6; small text 14px; labels 12px uppercase with letter-spacing.
- Numbers are the heroes: scores use 40px bold in the semantic score color.

### Shape & elevation

- Radii: 6 / 10 / 14 / 20 px (`--border-radius-sm/md/lg/xl`).
- Shadows are layered and subtle (`--box-shadow-sm/md/lg`); cards sit on
  hairline borders (`--border-color-light`) plus a small shadow, lifting
  slightly on hover.

### Motion

- Transitions: 180ms, `cubic-bezier(0.2, 0, 0, 1)`.
- Score display fades/scales in once on load (`pulse-in`).
- Hover: cards translate −2px with a deeper shadow; map polygons show a pointer
  cursor.
- All animation collapses under `prefers-reduced-motion: reduce`.

## Views

### Map (`/`)
Full-viewport split: fixed 380px sidebar (search → results → detail card) and
the choropleth filling the rest. On ≤768px the layout stacks — sidebar on top
(scrollable, capped at 45dvh), map below. Clicking a polygon or search result
opens the sidebar detail card; "View Full Details" navigates to `/{id}`.

### Detail (`/{id}`)
Header (name, score badge with emoticon status icon, website button, province /
population / financial year) → five metric cards (revenue per capita, capex %,
opex %, debt, audit outcome) → expandable score breakdown with per-pillar
progress bars → about section → refresh action. A `null` score renders
"Insufficient data" — never a fabricated zero.

### Compare (`/compare/{ids}`)
Horizontally scroll-snapped cards with edge-fade hints, one card per
municipality, latest-year metrics and score.

### Errors
Route-level `+error.svelte`: status, message, and a way back to the map.

## Accessibility

- Keyboard: global `:focus-visible` ring in the primary color; search is the
  keyboard path to any municipality (map polygons are mouse-first).
- Screen readers: labelled search input, `role="alert"` errors,
  `role="status"` loading, `aria-busy` skeletons, full ARIA on progress bars.
- Contrast: text on background ≥ 12:1; muted text ≥ 4.5:1.

## Deferred / future

- Dark mode: the token architecture supports it (single sheet swap); not yet
  shipped.
- Map tooltips on hover, province/district drill-down, self-hosted fonts.
