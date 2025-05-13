use std::error::Error;

use crate::utils::api::omie::client::OmieClient;
use crate::utils::api::asaas::client::AsaasClient;
use crate::utils::api::sheets::client::SheetClient;
use crate::utils::api::omie::omie_structs::Charge;
use crate::utils::functions::clear_terminal::clear_terminal;
use crate::utils::functions::process_single_charge::process_single_charge;

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
        let sheet_table = self.get_registered_sheet().await?;
        let overdue_charges = self.omie_client.list_overdue_charges().await?;
        println!("{}", overdue_charges.len());
        

        println!("{}", overdue_charges.len());
        
        self.process_charges_to_sheet(sheet_table, overdue_charges).await;
        
        Ok(())
    }

    async fn process_charges_to_sheet(&self, sheet_table: Vec<Vec<String>>, overdue_charges: Vec<Charge>) -> () {
        let mut table = Vec::new();
        let count_charges = overdue_charges.len();

        let mut counter = 0;
        let mut counter_charges = 0;
        for charge in overdue_charges {
            counter += 1;
            counter_charges += 1;
            
            clear_terminal();
            println!("Processando cobrança {}/{}", counter_charges, count_charges);

            // Log para verificar se a cobrança já está registrada
            if self.is_charge_registered(&charge, &sheet_table) {
                println!("Cobrança já registrada: {:?}", charge.cNumeroContrato);
                continue;
            }

            // Log para depurar o processamento de uma cobrança
            println!("Processando cobrança com contrato: {:?}", charge.cNumeroContrato);

            let row: Result<Vec<String>, Box<dyn Error + 'static>> = process_single_charge(&self.omie_client, &self.asaas_client, charge).await;

            match row {
                Ok(row) => {
                    table.push(row);
                    println!("Cobrança processada com sucesso.");

                    if counter >= 10 || counter_charges == count_charges {
                        println!("Enviando lote de cobranças para a planilha...");
                        let _ = self.sheet_client.append_sheet(&table).await;
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

    async fn get_registered_sheet(&self) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
        match self.sheet_client.get_sheet().await {
            Ok(sheet) if !sheet.is_empty() => Ok(sheet),
            Ok(_) => {
                println!("Planilha registrada está vazia. Continuando...");
                Ok(Vec::new())
            }
            Err(e) => {
                eprintln!("Erro ao obter a planilha registrada: {}", e);
                Ok(Vec::new())
            }
        }
    }

    fn is_charge_registered(&self, charge: &Charge, registered_sheet: &[Vec<String>]) -> bool {
        if let Some(c_numero_contrato) = &charge.cNumeroContrato {
            registered_sheet.iter().any(|row| 
                row[1] == *c_numero_contrato &&
                row[3] == charge.data_vencimento.to_string() &&
                row[4] == charge.valor_documento.to_string()
            )
        } else {
            false
        }
    }
}
