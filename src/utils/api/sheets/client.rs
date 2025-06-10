use std::time::Duration;

use crate::utils::api::config::api_key::get_api_key;

const BASE_URL: &str = "https://logssheetapi.squareweb.app/lateCharges";

#[derive(Clone)]
pub struct SheetClient {
  client: reqwest::Client,
  api_key: String,
}

impl SheetClient {
  pub fn new() ->Self {
      let api_key = get_api_key();
      let client = reqwest::Client::new();
     Self { client, api_key }
  }

  pub async fn append_sheet(&self, sheet: &Vec<Vec<String>>) -> Result<(), Box<dyn std::error::Error>> {
      let response = &self.client
          .post(BASE_URL)
          .header("Content-Type", "application/json")
          .header("x-api-key", &self.api_key)
          .body(serde_json::to_string(&sheet)?)
          .send()
          .await?;

      if response.status().is_success() {
          Ok(())
      } else {
        println!("{}", response.status().as_str());
          Err(Box::new(std::io::Error::new(
              std::io::ErrorKind::Other,
              "Falha ao atualizar planilha"
          )))
      }
    }

    pub async fn update_sheet(&self, sheet: &Vec<Vec<String>>, range: String) -> Result<(), Box<dyn std::error::Error>> {
        let body = serde_json::json!({
            "range": range,
            "data": sheet
        });

        let response = &self.client
            .put(BASE_URL)
            .header("Content-Type", "application/json")
            .header("x-api-key", &self.api_key)
            .body(body.to_string())
            .send()
            .await?;
  
        if response.status().is_success() {
            Ok(())
        } else {
          println!("{}", response.status().as_str());
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Falha ao atualizar planilha"
            )))
        }
      }

    pub async fn get_sheet(&self) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
        let response = self.client
            .get(BASE_URL)
            .header("Content-Type", "application/json")
            .header("x-api-key", &self.api_key)
            .timeout(Duration::from_secs(10))
            .send()
            .await?
            .error_for_status()?;

        if response.status().is_success() {
            let res_text = response.text().await?;
            let body: Vec<Vec<String>> = serde_json::from_str(&res_text)?;
            
            Ok(body)
        } else {
          println!("{}", response.status().as_str());
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Falha ao obter planilha"
            )))
        }
    }

    pub async fn clear_sheet(&self) -> Result<(), Box<dyn std::error::Error>> {
        let response: reqwest::Response = self.client.delete(BASE_URL)
            .header("x-api-key", &self.api_key)
            .timeout(Duration::from_secs(10))
            .send()
            .await?
            .error_for_status()?;

        if response.status().is_success() { Ok(()) }
        else {Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Falha para limpar planilha"
            )))}
    }
}
