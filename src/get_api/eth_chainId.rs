use serde::{Deserialize, Serialize};

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

    let uri = "http://eth.node.diagonalley.xyz/";
    if let Ok(res) = surf::post(uri).body_json(&data) {
        if let Ok(t) = res.recv_json::<ReturnData>().await{
            println!("{:#?}", t)
        }
        
    }
}

#[cfg(test)]
mod test {
    use super::eth_chainId;

    #[async_std::test]
    async fn test() {
        eth_chainId().await;
    }
}