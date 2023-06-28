
use log::{error, info, warn, debug, LevelFilter};
use log4rs::{append::file::FileAppender, encode::pattern::PatternEncoder, Config, config::{Appender, Root}};
use test1::conf::cli::load_conf;


#[async_std::main]
async fn main() {
    let c = load_conf();
    println!("{}", c.get_log_level());

    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build("log/output.log").unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder()
        .appender("logfile")
        .build(LevelFilter::Warn))
        .appender("stderr") //
        .unwrap();
    let t = config.appender(Appender::builder().filter(Box::new(ThresholdFilter::new(level))).build("stderr", Box::new(stderr)))
            .build(Root::builder().appender("logfile").appender("stderr").build(level)).unwrap();

    log4rs::init_config(config).unwrap();

    debug!("Hello, world!");
    info!("Hello, world!");
    warn!("Hello, world!");
    error!("Hello, world!");
}

