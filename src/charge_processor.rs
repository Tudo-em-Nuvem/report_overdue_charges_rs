use std::error::Error;

use crate::utils::api::omie::client::OmieClient;
use crate::utils::api::asaas::client::AsaasClient;
use crate::utils::api::sheets::client::SheetClient;
use crate::utils::api::omie::omie_structs::Charge;
use crate::utils::functions::clear_terminal::clear_terminal;
use crate::utils::functions::process_single_charge::process_single_charge;
use crate::utils::functions::try_until_works::{try_until_works, RetryConfig};

pub struct ChargeProcessor {
    omie_client: OmieClient,
    asaas_client: AsaasClient,
    sheet_client: SheetClient,
}

impl ChargeProcessor {
    pub fn new(omie_client: OmieClient, asaas_client: AsaasClient, sheet_client: SheetClient) -> Self {
        Self {
            omie_client,
            asaas_client,
            sheet_client,
        }
    }

    pub async fn process_overdue_charges(&self) -> Result<(), Box<dyn std::error::Error>> {        
        let overdue_charges = self.omie_client.list_overdue_charges().await?;
        println!("{}", overdue_charges.len());

        self.process_charges_to_sheet(overdue_charges).await;
        
        Ok(())
    }

    async fn process_charges_to_sheet(&self, overdue_charges: Vec<Charge>) -> () {
        let _ = try_until_works(|| async { self.sheet_client.clear_sheet().await }, RetryConfig::Sheets).await;
        
        let mut table: Vec<Vec<String>> = Vec::new();
        let count_charges = overdue_charges.len();

        let mut counter = 0;
        let mut counter_charges = 0;
        for charge in overdue_charges {
            counter += 1;
            counter_charges += 1;
            
            clear_terminal();
            println!("Processando cobrança {}/{}", counter_charges, count_charges);

            // Log para depurar o processamento de uma cobrança
            println!("Processando cobrança com contrato: {:?}", charge.cNumeroContrato);

            let row: Result<Vec<String>, Box<dyn Error + 'static>> = 
                try_until_works(|| {
                    async { 
                        process_single_charge(
                        &self.omie_client, 
                        &self.asaas_client, 
                        charge.clone()
                    ).await 
                }
            }, RetryConfig::Omie).await;

            match row {
                Ok(row) => {
                    table.push(row);
                    println!("Cobrança processada com sucesso.");

                    if counter >= 10 || counter_charges == count_charges {
                        println!("Enviando lote de cobranças para a planilha...");
                        let _ = try_until_works(|| async { self.sheet_client.append_sheet(&table).await }, RetryConfig::Sheets).await;
                        table.clear();
                        counter = 0;
                    }
                }
                Err(e) => {
                    eprintln!("Erro ao processar a cobrança: {}", e);
                    continue;
                }
            }
        }
    }
}
