mod utils;
mod charge_processor;
mod sheet_updater;

use std::io::{stdin, stdout, Write};
use utils::api::omie::client::OmieClient;
use utils::api::asaas::client::AsaasClient;
use utils::api::whatsapp::client::WhatsappClient;
use utils::api::sheets::client::SheetClient;
use utils::functions::clear_terminal::clear_terminal;
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

    loop {
        let message: String = String::from("
Escolha uma opção
_________________________________________________________________________
| [1] - Listar cobranças em atraso                                      |
| [2] - Enviar mensagens para cobranças que venceram ontem              |
| [3] - Enviar mensagens para cobranças que venceram em data específica |
_________________________________________________________________________\n
digite sua resposta: "
        );

        clear_terminal();
        let option: String = input(String::from(message));
    
        clear_terminal();
        match option.as_str() {
            "1" => {
                println!("Registrando cobranças em atraso...");
                charge_processor.process_overdue_charges().await?;       
                println!("\nPressione ENTER para sair...");
                let _ = input(String::new());
                return Ok(());
            }
            "2" => {
                println!("Enviando mensagens para cobranças que venceram ontem...");
                sheet_updater.send_message(None, false).await?;
                println!("\nPressione ENTER para sair...");
                let _ = input(String::new());
                return Ok(());
            }
            "3" => {
                let mut message: String = String::from("Digite a data (dd/mm/aaaa): ");
                loop {
                    clear_terminal();
                    let date: String = input(String::from(message.clone()));
                    let date_split: Vec<&str> = date.split('/').collect();

                    if date_split.len() != 3 || date_split[0].len() != 2 || date_split[1].len() != 2 || date_split[2].len() != 4 {
                        message = "Data inválida, tente novamente\nDigite a data (dd/mm/aaaa): ".to_string();
                        continue;
                    }

                    let option: String = input(String::from(String::from("Você deseja notificar que o sistema vai ser suspenso? S/N: ")));
                    let notify_suspension: bool = match option.to_lowercase().as_str() {
                        "s" | "sim" => true,
                        "n" | "não" | "nao" => false,
                        _ => {
                            println!("Opção inválida, não será notificado que o sistema vai ser suspenso");
                            false
                        }
                    };

                    println!("Enviando mensagens para cobranças que venceram em data específica...");
                    sheet_updater.send_message(Some(date), notify_suspension).await?;
                    println!("\nPressione ENTER para sair...");
                    let _ = input(String::new());
                    return Ok(());
                }
            }
            _ => {
                println!("Opção inválida");
                println!("\nPressione ENTER para sair...");
                let _ = input(String::new());
                return Ok(());
            }
        }
    }
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

