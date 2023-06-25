extern crate toml;

use std::fs::File;
use std::io::Read;
use std::ops::Deref;
use std::path::PathBuf;
use std::process::exit;

use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[clap(name = "get_con_mets")]
#[clap(version = "1.0.0")]
#[clap(about = "Scan Block")]
#[clap(arg_required_else_help = true)]

pub struct Cli {
    #[clap(short, long, value_name = "FILE")]//, parse(from_os_str),
    pub config: Option<PathBuf>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Config {
    log_info: LogInfo,
    server: Server,
    database: Database
}

#[derive(Clone, Deserialize, Serialize, Debug)]
struct LogInfo {
    level: String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
struct Server {
    ip: String,
    port: String
}

#[derive(Clone, Deserialize, Serialize, Debug)]
struct Database {
    host: String,
    username: String,
    password: String,
    database_name: String,
}

impl Config {
    pub fn new(file_path: &str) -> Self {
        if let Ok(mut f) = File::open(file_path) {
            let mut buff = String::new();
            if let Ok(_) = f.read_to_string(&mut buff) {
                match toml::from_str(buff.deref()) {
                    Ok(c) => { return c; }
                    Err(e) => {
                        panic!("🚨 Error: converting to toml! {:?}", e)
                    }
                }
            }
        }
        panic!("🚨 Error: Read Config Wrong!");
    }

    pub fn get_log_level(&self) -> String {
        self.log_info.level.clone()
    }

    
}

pub fn load_conf() -> Config {
    let c = Cli::parse();
    if let Some(conf_file) = c.config.as_deref() {
        if let Some(f) = conf_file.to_str() {
            let conf = Config::new(f);
            return conf;
        }
    }
    exit(1)
}
