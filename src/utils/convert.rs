use log::error;
use rlp::RlpStream;
use sha3::{Keccak256, Digest};
use chrono::{Utc, TimeZone};

// "0x80" -> 128
pub fn hex2num(hex: String) -> Option<i128> {
    match i128::from_str_radix(&hex[2..], 16) {
        Ok(num) => Some(num),
        Err(_e) => {
            error!("Invalid hex: {} msg: {:?}", hex, _e);
            None
        }
    }
}
// 128 -> "0x80"
pub fn num2hex(num: i128) -> String {
    format!("0x{:X}", num).to_ascii_lowercase()
}
// timetstamp -> %Y-%m-%d %H:%M:%S
pub fn timestamp2date(timestamp: String) -> String {
    let timestamp = hex2num(timestamp).unwrap();
    let utc = Utc.timestamp_opt(timestamp.try_into().unwrap(), 0).unwrap();
    let utc = utc.format("%Y-%m-%d %H:%M:%S").to_string();
    utc
}
// 根据from和nince计算合约地址
pub fn from_nonce2address(from: String, nonce: String) -> String {
    let from = hex::decode(&from[2..]).unwrap();
    let nonce = hex2num(nonce).unwrap() as u64;
    let rlp_data = {
        let mut stream = RlpStream::new();
        stream.begin_list(2);
        stream.append(&from);
        stream.append(&nonce);
        stream.out().to_vec()
    };
    let keccak = Keccak256::digest(&rlp_data);
    let address = &keccak[12..];
    format!("0x{}", hex::encode(address))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{}", num2hex(12));
        println!("{:?}", hex2num(String::from("0x12")));
        println!("{}", timestamp2date(String::from("0x5a1cb0b1")));

        let from_address = "0x36928500bc1dcd7af6a2b4008875cc336b927d57".to_string();
        let nonce = "0x6".to_string();
        let contract = from_nonce2address(from_address, nonce);
        println!("Contract Address: {:?}", contract);
    }
}