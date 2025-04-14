use super::{client::MunicipalMoneyClient, types::*};

impl MunicipalMoneyClient {
    /// Fetches all capital items for a specific municipality and year
    /// using the aggregate endpoint.
    pub async fn fetch_capital_aggregate(
        &self,
        municipality_code: &str,
        year: i32,
        amount_type: &str,
    ) -> Result<FactsApiResponse<FinancialItemFact>, ApiClientError> {
        const CAPEX_CUBE: &str = "capital_v2";
        const DRILLDOWNS: &str = "demarcation.code|demarcation.label|item.code|item.label";
        const AGGREGATES: &str = "amount.sum";

        let cuts = format!(
            "amount_type.code:{}|financial_period.period:{}|demarcation.code:\"{}\"",
            amount_type, year, municipality_code
        );

        let url = format!(
            "{}/cubes/{}/aggregate?drilldown={}&cut={}&aggregates={}",
            self.base_url(), CAPEX_CUBE, DRILLDOWNS, cuts, AGGREGATES
        );

        log::debug!("Fetching CapEx Aggregate URL: {}", url);

        let response = self.client().get(&url).send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response
                .text()
                .await
                .unwrap_or_else(|_| "Failed to read error body".to_string());
            log::error!(
                "CapEx Aggregate API request failed with status {}: {}",
                status,
                body
            );
            return Err(ApiClientError::ApiError {
                status: status.as_u16(),
                body: Some(body),
            });
        }

        let data: FactsApiResponse<FinancialItemFact> = response.json().await.map_err(ApiClientError::RequestError)?;

        log::trace!("Received CapEx Aggregate API response data: {:?}", data);

        Ok(data)
    }
}
