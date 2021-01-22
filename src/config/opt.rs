use structopt::StructOpt;

//Copy trait 是给编译器用的，告诉编译器这个类型默认采用 copy 语义，而不是 move 语义。Clone trait 是给程序员用的，我们必须手动调用clone方法，它才能发挥作用。
//

/// A basic example
#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "qtun-rs")]
pub struct Config {
    // A flag, true if used in the command line. Note doc comment will
    // be used for the help message of the flag. The name of the
    // argument will be, by default, based on the name of the field.
    /// Activate debug mode
    #[structopt(short, long)]
    server: bool,

    #[structopt(short, long)]
    client: bool,

    #[structopt(short, long, default_value = "0.0.0.0")]
    listen: String,

    #[structopt(short, long)]
    key: String,

    #[structopt(short, long)]
    remote_addr: String,

    #[structopt(short, long)]
    ip: String,

    #[structopt(long, default_value = "info")]
    log_level: String,

    #[structopt(short, long, default_value = "3")]
    transport_threads: u32,
}

// let CONFIG : &Config;
// static mut cfg: Option<Config> = None;
lazy_static! {
    static ref CFG: Config = Config::from_args();
}

// pub fn parse_args() ->() {
//     unsafe{
//         cfg = Some(Config::from_args());
//     }
// }

pub fn get_config() -> Config {
    // let tmp = cfg.as_ref().unwarp();
    Config{
        server: CFG.server,
        client: CFG.client,
        listen: CFG.listen.clone(),
        key: CFG.key.clone(),
        remote_addr: CFG.remote_addr.clone(),
        ip: CFG.ip.clone(),
        log_level: CFG.log_level.clone(),
        transport_threads: CFG.transport_threads,
    }
}