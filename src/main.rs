mod utils;
use std::io::Write;

use utils::api::omie::client::OmieClient;
use utils::api::asaas::client::AsaasClient;
use utils::api::sheets::client::SheetClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let omie_client = OmieClient::new();
    let asaas_client = AsaasClient::new();

    let mut sheet_table: Vec<Vec<String>> = Vec::new();
    let overdue_charges = omie_client.list_overdue_charges().await.unwrap();

    let count_charges = overdue_charges.len();
    let mut counter = 0;
    for charge in overdue_charges {
        print!("\x1B[2J\x1B[1;1H");
        std::io::stdout().flush().unwrap();
        counter += 1;
        println!("{} de {}", counter, count_charges);

        let mut link = String::new();
        let client = omie_client.get_client_omie(charge.codigo_cliente_fornecedor).await.unwrap();

        if charge.id_conta_corrente == 1889067132 {
            match asaas_client.get_client_asaas_by_contract(charge.cNumeroContrato.clone()).await? {
                Some(client_data) => {
                    if let Some(cobranca_asaas) =  asaas_client.get_charge_by_client_id(client_data.id).await? {
                        let charge = cobranca_asaas[0].clone();
                        link = charge.invoiceUrl;
                    }
                }

                None => {
                    link = "Não encontrado".to_string();
                }
            }
        } else {
            let boleto = omie_client.get_boleto(charge.codigo_lancamento_omie).await.unwrap();
            if boleto.cLinkBoleto.is_empty() {
                println!("Não há cobrança na omie para o cliente {}", &charge.cNumeroContrato);
                continue;
            }

            link = boleto.cLinkBoleto;
        }

        let tel = format!("{}{}", client.telefone1_ddd, client.telefone1_numero.replace("-", "")).replace(" ", "");
  
        let row = vec![
            client.nome_fantasia,
            charge.cNumeroContrato,
            tel,
            charge.data_vencimento.to_string(),
            charge.valor_documento.to_string(),
            client.cnpj_cpf,
            link
        ];

        sheet_table.push(row);
    }

    let sheet = SheetClient::new();
    sheet.update_sheet(sheet_table).await?;

    Ok(())
}
