-- Data-confidence grade per municipality-year (Phase 8-A2).
-- NULL = not yet evaluated; the healing pass backfills from stored values.
-- Grades: 'ok' | 'suspect' | 'unreliable' (see src/confidence.rs).
ALTER TABLE public.financial_data
    ADD COLUMN data_confidence text,
    ADD COLUMN confidence_notes text;

COMMENT ON COLUMN public.financial_data.data_confidence IS
    'Plausibility grade of the raw figures: ok | suspect | unreliable. NULL = not evaluated.';
COMMENT ON COLUMN public.financial_data.confidence_notes IS
    'Human-readable reasons when data_confidence is not ok.';
