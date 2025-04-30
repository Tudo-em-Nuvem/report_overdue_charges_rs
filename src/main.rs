mod utils;
use utils::api::omie::client::OmieClient;
use utils::api::asaas::client::AsaasClient;
use utils::api::sheets::client::SheetClient;
use utils::functions::process_single_charge::process_single_charge;
use utils::functions::clear_terminal::clear_terminal;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let omie_client: OmieClient = OmieClient::new();
    let sheet_client: SheetClient = SheetClient::new();
    let asaas_client: AsaasClient = AsaasClient::new();

    let mut sheet_table: Vec<Vec<String>> = Vec::new();

    let overdue_charges: Vec<utils::api::omie::omie_structs::Charge> = omie_client.list_overdue_charges().await.unwrap();

    let count_charges: usize = overdue_charges.len();
    let mut counter: usize = 0;

    let registered_sheet: Vec<Vec<String>> = match sheet_client.get_sheet().await {
        Ok(sheet) if !sheet.is_empty() => sheet,
        Ok(_) => {
            println!("Planilha registrada está vazia. Continuando...");
            Vec::new()
        }
        Err(e) => {
            eprintln!("Erro ao obter a planilha registrada: {}", e);
            Vec::new()
        }
    };

    for charge in overdue_charges {
        clear_terminal();
        counter += 1;
        println!("{} de {}", counter, count_charges);

        if registered_sheet.iter().any(|row: &Vec<String>| 
            row[1] == charge.cNumeroContrato.to_string() &&
            row[3] == charge.data_vencimento.to_string() &&
            row[4] == charge.valor_documento.to_string()
        )  { continue; }
        
        let row: Vec<String> = process_single_charge(
            &omie_client, 
            &asaas_client, 
            charge
        ).await.unwrap();

        sheet_table.push(row);

        if sheet_table.len() == 10 || counter == count_charges  {
            sheet_client.append_sheet(&sheet_table).await?;
            sheet_table = Vec::new();
        }
    }

    sleep(Duration::from_secs(1)).await;

    let registered_sheet: Vec<Vec<String>> = sheet_client.get_sheet().await.unwrap_or_else(|_| Vec::new());

    let mut sheet_index: usize = 1; // Índice real da planilha (começa em 1)
    let mut counter: usize = 1;

    let registered_sheet_len = registered_sheet.len();
    
    let mut batch: Vec<Vec<String>> = Vec::new();

    for row in registered_sheet {
        let mut new_row = row.clone();
        if row.len() < 8 {
            // enviar mensagem
            new_row.push("Primeira mensagem".to_string());
            batch.push(new_row);
        } else { batch.push(new_row); }

        if counter == 10 || sheet_index == registered_sheet_len {
            let start_index = if sheet_index < counter { 1 } else {
                sheet_index - counter + 1
            };

            let data_update = batch
                .iter()
                .rev()
                .take(counter)
                .cloned()
                .collect::<Vec<Vec<String>>>();
    
            println!("{:?}", data_update[0].clone());
            println!("A{}:H{}", start_index, sheet_index.clone());

            sheet_client.update_sheet(&data_update
                .iter()
                .rev()
                .cloned()
                .collect::<Vec<Vec<String>>>(), 
                format!("A{}:H{}", start_index, sheet_index )).await?;

            counter = 1;
        } else { counter+=1; }
        sheet_index += 1;
    }

    Ok(())
}
