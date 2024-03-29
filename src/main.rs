
use log::{error, info, warn, debug, LevelFilter};
use log4rs::{append::{file::FileAppender, console::ConsoleAppender}, encode::pattern::PatternEncoder, Config, config::{Appender, Root}};
use test1::conf::cli::load_conf;


#[async_std::main]
async fn main() {
    let c = load_conf();
    println!("{}", c.get_log_level());

    let stdout = ConsoleAppender::builder().build();
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build("log/output.log").unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("console", Box::new(stdout)))
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("console").appender("logfile").build(LevelFilter::Debug))
        .unwrap();

    log4rs::init_config(config).unwrap();

    debug!("Hello, world!");
    info!("Hello, world!");
    warn!("Hello, world!");
    error!("Hello, world!");
}

