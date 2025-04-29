
extern crate dotenv;

use dotenv::dotenv;
use std::env;

pub fn get_api_key() -> String {
    dotenv().ok();
    env::var("API_KEY").map_err(|_| {
        panic!("API_KEY not found in .env file")
    }).unwrap()
}