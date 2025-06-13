use std::future::Future;
use std::time::Duration;
use tokio::time::sleep;
#[derive(Debug)]
pub enum RetryConfig {
    Sheets,    // Operações com Google Sheets
    Omie,      // Operações com API Omie
    Asaas,     // Operações com API Asaas
    WhatsApp,  // Operações com WhatsApp
}

impl RetryConfig {
    fn max_attempts(&self) -> u32 {
        match self {
            RetryConfig::Sheets => 8,    // Mais tentativas para Sheets
            RetryConfig::Omie => 5,      // Tentativas padrão para Omie
            RetryConfig::Asaas => 5,     // Tentativas padrão para Asaas
            RetryConfig::WhatsApp => 3,  // Menos tentativas para WhatsApp
        }
    }

    fn initial_backoff(&self) -> u64 {
        match self {
            RetryConfig::Sheets => 2000,   // 2 segundos para Sheets
            RetryConfig::Omie => 1000,     // 1 segundo para Omie
            RetryConfig::Asaas => 1000,    // 1 segundo para Asaas
            RetryConfig::WhatsApp => 500,  // 0.5 segundos para WhatsApp
        }
    }

    fn max_backoff(&self) -> u64 {
        match self {
            RetryConfig::Sheets => 64000,  // 64 segundos para Sheets
            RetryConfig::Omie => 32000,    // 32 segundos para Omie
            RetryConfig::Asaas => 32000,   // 32 segundos para Asaas
            RetryConfig::WhatsApp => 8000, // 8 segundos para WhatsApp
        }
    }
}

pub async fn try_until_works<T, E, Fut, F>(mut function: F, config: RetryConfig) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, E>>,
    E: std::fmt::Debug,
{
    let mut attempts = 0;
    let max_attempts = config.max_attempts();
    let mut backoff = config.initial_backoff();
    let max_backoff = config.max_backoff();

    loop {
        match function().await {
            Ok(val) => return Ok(val),
            Err(e) => {
                attempts += 1;
                if attempts >= max_attempts {
                    println!("Número máximo de tentativas ({}) atingido para {:?}. Último erro: {:?}", 
                        max_attempts, config, e);
                    return Err(e);
                }

                println!("Tentativa {} falhou para {:?}. Erro: {:?}. Aguardando {}ms antes da próxima tentativa...", 
                    attempts, config, e, backoff);
                
                sleep(Duration::from_millis(backoff)).await;
                backoff = std::cmp::min(backoff * 2, max_backoff);
                continue;
            }
        }
    }
}
