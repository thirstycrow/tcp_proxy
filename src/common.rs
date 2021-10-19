use clap::{App, Arg};

#[derive(Clone)]
pub struct Config {
    pub bind_address: String,
    pub bind_port: u16,
    pub upstream_address: String,
    pub upstream_port: u16,
    pub numa_node: Option<usize>,
    pub use_hyper_thread: bool,
    pub nr_shards: Option<usize>,
    pub spin_before_park: usize,
}

pub fn parse_config(name: &str) -> Config {
    let matches = App::new(format!("Tcp Proxy ({})", name))
        .arg(
            Arg::with_name("bind_address")
                .long("bind-address")
                .takes_value(true)
                .use_delimiter(false)
                .required(false)
                .default_value("0.0.0.0"),
        )
        .arg(
            Arg::with_name("bind_port")
                .long("bind-port")
                .takes_value(true)
                .use_delimiter(false)
                .required(false)
                .default_value("8080"),
        )
        .arg(
            Arg::with_name("upstream_address")
                .long("upstream-address")
                .takes_value(true)
                .use_delimiter(false)
                .required(true),
        )
        .arg(
            Arg::with_name("upstream_port")
                .long("upstream-port")
                .takes_value(true)
                .use_delimiter(false)
                .required(true),
        )
        .arg(
            Arg::with_name("numa_node")
                .help("not available for `tcp_proxy_tokio`")
                .long("numa-node")
                .takes_value(true)
                .use_delimiter(false)
                .required(false),
        )
        .arg(
            Arg::with_name("use_hyper_thread")
                .help("not available for `tcp_proxy_tokio`")
                .long("use-hyper-thread")
                .takes_value(true)
                .use_delimiter(false)
                .required(false)
                .default_value("true"),
        )
        .arg(
            Arg::with_name("nr_shards")
                .help("not available for `tcp_proxy_tokio`")
                .long("nr-of-shards")
                .takes_value(true)
                .use_delimiter(false)
                .required(false),
        )
        .arg(
            Arg::with_name("spin_before_park")
                .help("not available for `tcp_proxy_tokio`")
                .long("spin-before-park")
                .takes_value(true)
                .use_delimiter(false)
                .required(false)
                .default_value("0"),
        )
        .get_matches();

    Config {
        bind_address: matches.value_of("bind_address").unwrap().into(),
        bind_port: matches.value_of("bind_port").unwrap().parse().unwrap(),
        upstream_address: matches.value_of("upstream_address").unwrap().into(),
        upstream_port: matches.value_of("upstream_port").unwrap().parse().unwrap(),
        numa_node: matches.value_of("numa_node").map(|v| v.parse().unwrap()),
        use_hyper_thread: matches
            .value_of("use_hyper_thread")
            .unwrap()
            .parse()
            .unwrap(),
        nr_shards: matches.value_of("nr_shards").map(|v| v.parse().unwrap()),
        spin_before_park: matches
            .value_of("spin_before_park")
            .unwrap()
            .parse()
            .unwrap(),
    }
}
