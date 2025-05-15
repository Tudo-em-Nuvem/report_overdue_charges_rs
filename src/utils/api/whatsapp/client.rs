use crate::utils::api::config::whatsapp_key::get_keys;

pub struct WhatsappClient {
  client: reqwest::Client,
  api_key: String,
  base_url: String,
}

impl WhatsappClient {
  pub fn new() -> Self {
    let keys = get_keys();
    let base_url = format!("https://api.z-api.io/instances/{}/token/{}/send-text", keys.instance_id, keys.token_instance);
    let api_key = keys.api_key.clone();
    let client = reqwest::Client::new();
    Self { client, api_key, base_url }
    }

    pub async fn send_message(&self, tel_number: String, message: String) -> Result<(), Box<dyn std::error::Error>> {
      let response = self.client
        .post(&self.base_url)
        .header("Content-Type", "application/json")
        .header("Client-Token", &self.api_key)
        .body(serde_json::to_string(&serde_json::json!({
          "phone": tel_number,
          "message": message
        }))?)
        .send()
        .await?;

      if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await?;
        return Err(format!("Erro ao enviar mensagem. Status: {}. Resposta: {}", status, error_text).into());
      }

      Ok(())
    }
}
