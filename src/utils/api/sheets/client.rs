use crate::utils::api::config::api_key::get_api_key;

const BASE_URL: &str = "https://logssheetapi.squareweb.app/lateCharges";

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
  pub async fn update_sheet(&self, sheet: Vec<Vec<String>>) -> Result<(), Box<dyn std::error::Error>> {
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
  }
