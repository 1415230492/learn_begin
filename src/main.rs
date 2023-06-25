
use test1::conf::cli::load_conf;


fn main() {
    // 打开文件
    // let mut file = File::open("config.toml").unwrap();
    // let mut contents = String::new();

    // // 读取文件内容
    // file.read_to_string(&mut contents).unwrap();

    // // 解析 TOML 并将其转换为 Config 结构体
    // let config: ConfigToml = toml::from_str(&contents).unwrap();

    // // 访问配置参数
    // println!("Server IP: {}", config.server.ip);
    // println!("Server port: {}", config.server.port);
    // println!("Database host: {}", config.database.host);
    // println!("Database username: {}", config.database.username);


    let c = load_conf();
    println!("{}", c.get_log_level())
    

    // let logfile = FileAppender::builder()
    //     .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
    //     .build("log/output.log").unwrap();

    // let config = Config::builder()
    //     .appender(Appender::builder().build("logfile", Box::new(logfile)))
    //     .build(Root::builder()
    //                .appender("logfile")
    //                .build(LevelFilter::Warn)).unwrap();

    // log4rs::init_config(config).unwrap();

    // debug!("Hello, world!");
    // info!("Hello, world!");
    // warn!("Hello, world!");
    // error!("Hello, world!");
}

