use super::format_phone_number;
use format_phone_number::format_phone_number;

use crate::utils::api::omie::client::OmieClient;
use crate::utils::api::omie::omie_structs::Charge;
use crate::utils::api::asaas::client::AsaasClient;

pub async fn process_single_charge(
  omie_client: &OmieClient,
  asaas_client: &AsaasClient,
  charge: Charge
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut link = String::new();
        let client = omie_client.get_client_omie(charge.codigo_cliente_fornecedor).await.unwrap();
        let mut c_numero_contrato: String;
        if charge.id_conta_corrente == 1889067132 {
            if let Some(c_num_ctr) = charge.cNumeroContrato {
                c_numero_contrato = c_num_ctr;
                match asaas_client.get_client_asaas_by_contract(c_numero_contrato.clone()).await? {
                    Some(client_data) => {
                        if let Some(cobranca_asaas) =  asaas_client.get_charge_by_client_id(client_data.id).await? {
                            let charge = cobranca_asaas[0].clone();
                            link = charge.invoiceUrl;
                        }
                    }

                    None => {
                        c_numero_contrato = "Não encontrado".to_string();
                        link = "Não encontrado".to_string();
                    }
                }
            } else {
                c_numero_contrato = charge.cNumeroContrato.unwrap_or_default();
                link = "Contrato não informado".to_string();
            }
        } else {
            c_numero_contrato = charge.cNumeroContrato.unwrap_or_default();
            let boleto = omie_client.get_boleto(charge.codigo_lancamento_omie).await.unwrap();

            if boleto.cLinkBoleto.is_empty() {
                link = "Não há cobrança na omie para o cliente".to_string();
            } else {
                link = boleto.cLinkBoleto;
            }
        }

        if link.is_empty() {
            link = "Não encontrado".to_string();
        }
        let tel = format_phone_number(
            client.telefone1_ddd, 
            client.telefone1_numero
        );

        let row = vec![
            client.nome_fantasia,
            c_numero_contrato.to_string(),
            tel,
            charge.data_vencimento.to_string(),
            charge.valor_documento.to_string(),
            client.cnpj_cpf,
            link
        ];

        Ok(row)
}
