use crate::utils::functions::try_until_works::{try_until_works, RetryConfig};
use crate::utils::{api::sheets::client::SheetClient, functions::get_current_date::get_yesterday_date_in_sao_paulo};
use crate::utils::api::whatsapp::client::WhatsappClient;

pub struct SheetUpdater {
    sheet_client: SheetClient,
    whatsapp_client: WhatsappClient,
}

impl SheetUpdater {
    pub fn new(sheet_client: SheetClient, whatsapp_client: WhatsappClient) -> Self {
        Self { sheet_client, whatsapp_client }
    }

    pub async fn send_message(&self, date: Option<String>, notify_suspension: bool) -> Result<(), Box<dyn std::error::Error>> {  
        let registered_sheet = try_until_works(|| async { self.sheet_client.get_sheet().await }, RetryConfig::Sheets).await?;

        let mut sheet_index = 1;
        let mut counter = 1;
        let registered_sheet_len = registered_sheet.len();
        let mut batch: Vec<Vec<String>> = Vec::new();

        let date: String = date.unwrap_or_else(|| {
            get_yesterday_date_in_sao_paulo()
        });
        
        let mut sent_tel: Vec<String> = Vec::new();
        for row in registered_sheet {
            let mut new_row = row.clone();

            if row.len() < 8  && !sent_tel.contains(&row[2]) && row[3] == date {
                let message = match notify_suspension {
                    true => format!("Olá {}!,\nAmanhã o seu serviço será suspenso!\nRealize o pagamento e evite a suspensão.", row[0]),
                    false => format!("Olá {}!,\nA sua cobrança venceu!\nRealize o pagamento para evitar a suspensão do serviço.", row[0])
                };
                println!("Enviando mensagem para ({}): {}", row[2], date);
                
                let message_status: &'static str = match try_until_works(|| async {
                    self.whatsapp_client.send_message( row[2].clone(), message.clone()).await
                }, RetryConfig::WhatsApp).await {
                    Ok(_) => "Primeira mensagem",
                    Err(e) => {
                        println!("Falha ao enviar mensagem para {}: {:?}", row[2], e);
                        "Não enviado"
                    }
                };

                new_row.push(message_status.to_string());
                batch.push(new_row);
                sent_tel.push(row[2].clone());
            } else { batch.push(new_row); }

            if counter == 10 || sheet_index == registered_sheet_len {
                self.process_batch(&batch, counter, sheet_index).await?;
                counter = 1;
                batch.clear();
            } else {
                counter += 1;
            }
            sheet_index += 1;
        }

        Ok(())
    }

    async fn process_batch(&self, batch: &[Vec<String>], counter: usize, sheet_index: usize) -> Result<(), Box<dyn std::error::Error>> {
        let start_index: usize = if sheet_index < counter { 1 } else {
            sheet_index - counter + 1
        };

        let data_update = batch
            .iter()
            .rev()
            .take(counter)
            .cloned()
            .collect::<Vec<Vec<String>>>();

        println!("Atualizando planilha: A{}:H{}", start_index, sheet_index);

        try_until_works(|| async {
            self.sheet_client.update_sheet(
                &data_update.iter().rev().cloned().collect::<Vec<Vec<String>>>(),
                format!("A{}:H{}", start_index, sheet_index)
            ).await
        }, RetryConfig::Sheets).await?;

        Ok(())
    }
}
