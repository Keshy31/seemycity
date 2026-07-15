-- Scoring v2 inputs (Phase 8-B): own-revenue split, UIFW, repairs & maintenance,
-- plus a score_version stamp so the healing pass can migrate rows wholesale
-- when the scoring formula changes.
ALTER TABLE public.financial_data
    ADD COLUMN transfers_operational numeric,
    ADD COLUMN uifw_expenditure numeric,
    ADD COLUMN repairs_maintenance numeric,
    ADD COLUMN score_version integer;

COMMENT ON COLUMN public.financial_data.transfers_operational IS
    'Operational grants received (incexp_v2 item 2200); part of revenue. own revenue = revenue - transfers.';
COMMENT ON COLUMN public.financial_data.uifw_expenditure IS
    'Total unauthorised + irregular + fruitless & wasteful expenditure (uifwexp cube). NULL = none reported.';
COMMENT ON COLUMN public.financial_data.repairs_maintenance IS
    'Repairs & maintenance spend, audited actuals (repmaint_v2 cube).';
COMMENT ON COLUMN public.financial_data.score_version IS
    'Version of the scoring formula the stored scores were computed under.';
