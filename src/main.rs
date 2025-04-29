mod utils;
use utils::api::omie::client::OmieClient;
use utils::api::asaas::client::AsaasClient;
use utils::api::sheets::client::SheetClient;
use utils::functions::process_single_charge::process_single_charge;
use utils::functions::clear_terminal::clear_terminal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let omie_client: OmieClient = OmieClient::new();
    let asaas_client: AsaasClient = AsaasClient::new();

    let mut sheet_table: Vec<Vec<String>> = Vec::new();
    let sheet_client: SheetClient = SheetClient::new();

    let overdue_charges: Vec<utils::api::omie::omie_structs::Charge> = omie_client.list_overdue_charges().await.unwrap();

    let count_charges: usize = overdue_charges.len();
    let mut counter: i32 = 0;

    for charge in overdue_charges {
        clear_terminal();
        counter += 1;
        println!("{} de {}", counter, count_charges);

        let row: Vec<String> = process_single_charge(
            &omie_client, 
            &asaas_client, 
            charge
        ).await.unwrap();

        sheet_table.push(row);

        if sheet_table.len() == 10 {
            sheet_client.update_sheet(&sheet_table).await?;
            sheet_table = Vec::new();
        }
    }

    Ok(())
}
