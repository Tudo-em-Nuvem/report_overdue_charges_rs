use serde::Deserialize;

#[derive(Deserialize)]
#[derive(Debug)]
pub struct Client {
  pub id: String,
}

#[derive(Deserialize, Clone)]
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct Cobranca {
  pub invoiceUrl: String,
}