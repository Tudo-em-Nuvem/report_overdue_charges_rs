use chrono::prelude::*;
use chrono_tz::America::Sao_Paulo;

pub fn get_yesterday_date_in_sao_paulo() -> String {
    // Obtém a data e hora atual no fuso horário de São Paulo
    let yesterday = Utc::now().with_timezone(&Sao_Paulo) - chrono::Duration::days(1);
    
    // Formata a data para o formato dd/mm/yyyy
    yesterday.format("%d/%m/%Y").to_string()
}
