use std::time::Duration;

use log::info;
use serde::{Deserialize, Serialize};
use surf::{Client, Config};


#[derive(Deserialize, Serialize)]
struct RequestData {
    jsonrpc: String,
    method: String,
    params: Vec<String>,
    id: i8
}
#[derive(Deserialize, Serialize, Debug)]
struct ReturnData {
    jsonrpc: String,
    id: i8,
    result: String
}

pub async fn eth_chain_id() {
    let data = RequestData {
        jsonrpc: "2.0".to_string(),
        method: "eth_chainId".to_string(),
        params: Vec::new(),
        id: 1
    };

    let url = "https://eth.llamarpc.com";

    // 添加超时设置
    let client:Client = Config::new()
        .set_timeout(Some(Duration::from_millis(10)))
        .try_into().unwrap(); // 设置超时时间为 10 秒
    match client.post(url.clone()).body_json(&data) {
        Ok(res) => {
            // println!("res: {:#?}", res);
            match res.recv_json::<ReturnData>().await {
                Ok(return_data) => {
                    info!("{:#?}", return_data)
                },
                Err(err) => {
                    info!("err: {:#?}", err);
                }
            }
        },
        Err(err) => {
            info!("err: {:#?}", err);
        }
    }
}

#[cfg(test)]
mod test {
    use super::eth_chain_id;

    #[async_std::test]
    async fn test() {
        eth_chain_id().await;
    }
}