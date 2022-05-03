use crate::blockchain_address::BlockchainAddress;
use crate::blockchain_status::BlockchainStatus;
use crate::blockchain_transaction::BlockchainTransaction;
use dotenv;
use reqwest;
use serde_json::Result;
use tokio;

const HOST_ROOT: &str = "https://btcbook.nownodes.io/api/";

#[tokio::main]
pub async fn send_request(url: &str) -> String {
    let client = reqwest::Client::new();

    client
        .get(url)
        .header(
            "api-key",
            dotenv::var("API_KEY").expect("API_KEY not found"),
        )
        .send()
        .await
        .expect("Fail to get response")
        .text()
        .await
        .expect("Fail to convert payload")
}

pub fn blockchain_status_request() -> BlockchainStatus {
    let response = send_request(&HOST_ROOT);
    serde_json::from_str(&response).expect("Fail to parse JSON")
}

pub fn blockchain_address_request(address: &str) -> BlockchainAddress {
    let response = send_request(&[HOST_ROOT, "v2/address/", &address].join(""));
    serde_json::from_str(&response).expect("Fail to parse JSON")
}

pub fn blockchain_transaction_request(transaction: &str) -> BlockchainTransaction {
    let response = send_request(&[HOST_ROOT, "v2/tx/", &transaction].join(""));
    serde_json::from_str(&response).expect("Fail to parse JSON")
}
