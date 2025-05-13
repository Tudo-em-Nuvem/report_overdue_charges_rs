
extern crate dotenv;

use dotenv::dotenv;
use std::env;

pub struct ApiKeyConfig {
    pub api_key: String,
    pub instance_id: String,
    pub token_instance: String,
}

pub fn get_keys() -> ApiKeyConfig {
    ApiKeyConfig {
        api_key: extract_env_var("API_KEY_WHATSAPP"),
        instance_id: extract_env_var("INSTANCE_ID_WHATSAPP"),
        token_instance: extract_env_var("TOKEN_INSTANCE_WHATSAPP")
    }
}

fn extract_env_var(key: &str) -> String {
    dotenv().ok();
    env::var(key).unwrap_or_else(|_| {
        panic!("Environment variable {} not found", key)
    })
}