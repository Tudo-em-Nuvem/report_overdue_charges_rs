mod utils;
mod charge_processor;
mod sheet_updater;

use std::io::{stdin, stdout, Write};
use utils::api::omie::client::OmieClient;
use utils::api::asaas::client::AsaasClient;
use utils::api::whatsapp::client::WhatsappClient;
use utils::api::sheets::client::SheetClient;
use charge_processor::ChargeProcessor;
use sheet_updater::SheetUpdater;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let omie_client = OmieClient::new();
    let sheet_client = SheetClient::new();
    let asaas_client = AsaasClient::new();
    let whatsapp_client = WhatsappClient::new();
    let charge_processor = ChargeProcessor::new(omie_client, asaas_client, sheet_client.clone());
    let sheet_updater = SheetUpdater::new(sheet_client, whatsapp_client);

    let message: String = String::from("
Escolha uma opção
____________________________________________________________
| [1] - Listar cobranças em atraso                         |
| [2] - Enviar mensagens para cobranças que venceram ontem |
____________________________________________________________\n
digite sua resposta: "
    );

    let option: String = input(String::from(message));

    match option.as_str() {
        "1" => {
            println!("Regitrando cobranças em atraso...");
            charge_processor.process_overdue_charges().await?;       
        }
        "2" => {
            println!("Enviando mensagens para cobranças que venceram ontem...");
            sheet_updater.send_message().await?;

            return Ok(());
        }
        _ => {
            println!("Opção inválida");
            return Ok(());
        }
    }

    Ok(())
}

fn input(question: String) -> String {
    let mut s: String = String::new();
    print!("{}", question.to_string());

    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("Algo deu errado");

    if let Some('\n') = s.chars().next_back() {
        s.pop();
    } if let Some('\r') = s.chars().next_back() {
        s.pop();
    }

    s
}

