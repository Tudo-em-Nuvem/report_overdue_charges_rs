use super::asaas_structs;
use asaas_structs::{Client, Cobranca};

use crate::utils::api::config::api_key::get_api_key;
use std::time::Duration;

const BASE_URL: &str = "https://apitdnomieasaas.squareweb.app/asaas/";

pub struct AsaasClient {
    client: reqwest::Client,
    api_key: String,
}

impl AsaasClient {
    pub fn new() -> Self {
        let api_key = get_api_key();
        let client = reqwest::Client::new();
       Self { client, api_key }
    }

    pub async fn get_client_asaas_by_contract (&self, c_num_ctr: String)-> Result<Option<Client>, Box<dyn std::error::Error>> {
        let c_num_ctr = c_num_ctr.replace("/", "%2f");
        let endpoint = format!("customers/cNumCtr/{}", c_num_ctr);
        self.send_request(&endpoint, None).await
    }

    pub async fn get_charge_by_client_id(&self, client_id: String) -> Result<Option<Vec<Cobranca>>, Box<dyn std::error::Error>>  {
        let endpoint = format!("charges/{}", client_id);
        self.send_request(&endpoint, None).await
    }

    pub async fn send_request<T: serde::de::DeserializeOwned>(
    &self,
    endpoint: &str,
    query_params: Option<&[(&str, &str)]>) -> Result<Option<T>, Box<dyn std::error::Error>> {
        let url = format!("{}{}", BASE_URL, endpoint);

        let mut request = self.client.get(&url).header("x-api-key", &self.api_key);
        if let Some(params) = query_params {
            request = request.query(params);
        }

        let response = request
            .timeout(Duration::from_secs(10))
            .send()
            .await?;

        if response.status() == 404 {
            return Ok(None);
        }

        let response = response.error_for_status()?;
        let res_text = response.text().await?;
        let body: T = serde_json::from_str(&res_text)?;
        Ok(Some(body))
    }
}
