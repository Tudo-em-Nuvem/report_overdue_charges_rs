use super::omie_structs;
use omie_structs::{ResponseOverdueCharges, Charge, Client, Boleto};

use std::time::Duration;
use crate::utils::api::config::api_key::get_api_key;

const BASE_URL: &str = "https://apitdnomieasaas.squareweb.app/omie/";

pub struct OmieClient {
    client: reqwest::Client,
    api_key: String,
}

impl OmieClient {
    pub fn new() -> Self {
        let api_key = get_api_key();
        let client = reqwest::Client::new();
       Self { client, api_key }
    }

    pub async fn list_overdue_charges(&self) -> Result<Vec<Charge>, Box<dyn std::error::Error>> {
        let response: ResponseOverdueCharges = self
            .send_request("bills", Some(&[("status", "ATRASADO")]))
            .await?;
        Ok(response.conta_receber_cadastro)
    }

    pub async fn get_client_omie(&self, code_client: i64) -> Result<Client, Box<dyn std::error::Error>> {
        let endpoint = format!("client/{}", code_client);
        self.send_request(&endpoint, None).await
    }

    pub async fn get_boleto(&self, code_charge: i64) -> Result<Boleto, Box<dyn std::error::Error>> {
        self.send_request("boleto", Some(&[("nCodTitulo", &code_charge.to_string())]))
        .await
    }

    async fn send_request<T: serde::de::DeserializeOwned>(
        &self,
        endpoint: &str,
        query_params: Option<&[(&str, &str)]>,
    ) -> Result<T, Box<dyn std::error::Error>> {
        let url = format!("{}{}", BASE_URL, endpoint);

        let mut request = self.client.get(&url).header("x-api-key", &self.api_key);

        if let Some(params) = query_params {
            request = request.query(params);
        }

        let response = request
            .timeout(Duration::from_secs(10))
            .send()
            .await?
            .error_for_status()?;

        let res_text = response.text().await?;
        let body: T = serde_json::from_str(&res_text)?;
        Ok(body)
    }
}
