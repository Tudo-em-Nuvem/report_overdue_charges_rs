use crate::utils::functions::try_until_wokrs::try_until_works;
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

    pub async fn send_message(&self, date: Option<String>) -> Result<(), Box<dyn std::error::Error>> {  
        let registered_sheet = try_until_works(|| async { self.sheet_client.get_sheet().await }).await?;
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
                println!("Mensagem enviada para ({}): {}", row[2], date);
                let message_status = match self.whatsapp_client.send_message(
                    row[2].clone(),
                    format!("Olá, verifiquei que a sua cobrança está vencida desde {}. Posso te enviar o link para pagamento?", row[3])
                ).await {
                    Ok(_) => "Primeira mensagem",
                    Err(_) => "Não enviado"
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

        println!("A{}:H{}", start_index, sheet_index);

        self.sheet_client.update_sheet(
            &data_update.iter().rev().cloned().collect::<Vec<Vec<String>>>(),
            format!("A{}:H{}", start_index, sheet_index)
        ).await?;

        Ok(())
    }
}
