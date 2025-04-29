use serde::Deserialize;

#[derive(Deserialize)]
pub struct ResponseOverdueCharges {
  pub conta_receber_cadastro: Vec<Charge>,
}


#[derive(Deserialize)]
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct Charge {
  pub codigo_cliente_fornecedor: i64,
  pub codigo_lancamento_omie: i64,
  pub id_conta_corrente: i64,
  pub data_vencimento: String,
  pub valor_documento: f64,
  pub cNumeroContrato: String
}

#[derive(Deserialize)]
#[derive(Debug)]
pub struct Client {
  pub cnpj_cpf: String,
  pub nome_fantasia: String,
  pub telefone1_ddd: String,
  pub telefone1_numero: String,
}

#[derive(Deserialize)]
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct Boleto {
  pub cLinkBoleto: String,
}
