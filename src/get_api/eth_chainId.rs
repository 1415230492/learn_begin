use std::time::Duration;

use log::info;
use serde::{Deserialize, Serialize};
use surf::{client, http::Method, Client, Config, Url};


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

pub async fn eth_chainId() {
    let data = RequestData {
        jsonrpc: "2.0".to_string(),
        method: "eth_chainId".to_string(),
        params: Vec::new(),
        id: 1
    };

    let url = "https://eth.llamarpc.com";

    // 添加超时设置
    let client:Client = Config::new()
        .set_timeout(Some(Duration::from_millis(1)))
        .try_into().unwrap(); // 设置超时时间为 10 秒
    if let Ok(res) = client.post(url.clone()).body_json(&data) {
        println!("res: {:#?}", res);
        if let Ok(t) = res.recv_json::<ReturnData>().await{
            println!("----{:#?}", t)
        }else{
            println!("type match error")
        }
    }else{
        println!("error");
    }
    // println!("{:#?}", res)

}

#[cfg(test)]
mod test {
    use super::eth_chainId;

    #[async_std::test]
    async fn test() {
        eth_chainId().await;
    }
}