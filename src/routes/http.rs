use crate::helpers;
use actix_web::{get, HttpResponse, Result};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Info {
    timestamp: String,
}

#[get("/currentBlockTime")]
pub async fn index() -> Result<HttpResponse> {
    let block_data = helpers::http_client(String::from("http://api.etherscan.io/api?module=proxy&action=eth_blockNumber&apikey=YourApiKeyToken"))
        .await.
        unwrap();
    let block_id =  helpers::get_id_from_json(block_data).unwrap();
    let block_details = helpers::http_client(format!("https://api.etherscan.io/api?module=proxy&action=eth_getBlockByNumber&tag={}&boolean=true&apikey=YourApiKeyToken", block_id).replace("\"", ""))
        .await.
        unwrap();
    Ok(HttpResponse::Ok().json(Info {
        timestamp: helpers::get_timestamp_from_json(block_details).unwrap(),
    }))
}