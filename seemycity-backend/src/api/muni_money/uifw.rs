use super::{client::MunicipalMoneyClient, types::*};

impl MunicipalMoneyClient {
    /// Fetches Unauthorised, Irregular, Fruitless & Wasteful expenditure facts
    /// for a municipality and financial year.
    ///
    /// Cube quirks (probed 2026-07-07): `uifwexp` is keyed by
    /// `financial_year_end.year` and has NO `amount_type` dimension. Items are
    /// `unauthorised` / `irregular` / `fruitless`.
    pub async fn fetch_uifw_aggregate(
        &self,
        municipality_code: &str,
        year: i32,
    ) -> Result<FactsApiResponse<FinancialItemFact>, ApiClientError> {
        const UIFW_CUBE: &str = "uifwexp";
        const DRILLDOWNS: &str = "demarcation.code|demarcation.label|item.code|item.label";
        const AGGREGATES: &str = "amount.sum";

        let cuts = format!(
            "financial_year_end.year:{}|demarcation.code:\"{}\"",
            year, municipality_code
        );

        let url = format!(
            "{}/cubes/{}/aggregate?drilldown={}&cut={}&aggregates={}",
            self.base_url(), UIFW_CUBE, DRILLDOWNS, cuts, AGGREGATES
        );

        log::debug!("Fetching UIFW Aggregate URL: {}", url);

        let response = self.client().get(&url).send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response
                .text()
                .await
                .unwrap_or_else(|_| "Failed to read error body".to_string());
            log::error!("UIFW Aggregate API request failed with status {}: {}", status, body);
            return Err(ApiClientError::ApiError {
                status: status.as_u16(),
                body: Some(body),
            });
        }

        let data: FactsApiResponse<FinancialItemFact> =
            response.json().await.map_err(ApiClientError::RequestError)?;

        Ok(data)
    }
}
