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

    pub async fn send_message(&self) -> Result<(), Box<dyn std::error::Error>> {
        let registered_sheet = self.sheet_client.get_sheet().await.unwrap_or_else(|_| Vec::new());
        let mut sheet_index = 1;
        let mut counter = 1;
        let registered_sheet_len = registered_sheet.len();
        let mut batch: Vec<Vec<String>> = Vec::new();
        let yesterday = get_yesterday_date_in_sao_paulo();

        for row in registered_sheet {
            let mut new_row = row.clone();

            if row.len() < 8 && row[3] == yesterday {
                let _ = &self.whatsapp_client.send_message(
                    row[0].clone(),
                    format!("Olá, {}! Sua cobrança está vencida desde {}. Por favor, entre em contato conosco para regularizar sua situação.", row[1], row[3])
                ).await;
                new_row.push("Primeira mensagem".to_string());
                batch.push(new_row);
            } else {
                batch.push(new_row);
            }

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
