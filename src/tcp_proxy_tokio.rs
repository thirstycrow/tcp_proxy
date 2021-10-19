use futures::future::try_join;

use tokio::{
    net::{TcpListener, TcpStream},
    runtime::Builder,
};

use common::parse_config;
use cpu_time::ThreadTime;
use std::time::Duration;

mod common;

thread_local! {
    static THREAD_TIME: ThreadTime = ThreadTime::now();
}

fn main() {
    pretty_env_logger::init();

    let config = parse_config("Tokio");

    let mut builder = Builder::new_multi_thread();
    config.nr_shards.map(|n| builder.worker_threads(n));
    let runtime = builder.enable_all().build().unwrap();

    runtime.block_on(async move {
        tokio::spawn(async move {
            let mut last = Duration::ZERO;
            for i in 0..100 {
                tokio::time::sleep(Duration::from_millis(1000)).await;
                let thread_time_elapsed = THREAD_TIME.with(|t| t.elapsed());
                println!(
                    "{}: {:?} {:?}",
                    i,
                    thread_time_elapsed - last,
                    thread_time_elapsed
                );
                last = thread_time_elapsed;
            }
        });

        let bind_address = (config.bind_address.as_str(), config.bind_port);
        let listener = TcpListener::bind(bind_address).await.unwrap();
        while let Ok((mut downstream, _)) = listener.accept().await {
            let config = config.clone();

            tokio::spawn(async move {
                let mut upstream =
                    TcpStream::connect((config.upstream_address.as_str(), config.upstream_port))
                        .await
                        .unwrap();
                let (mut rd, mut wd) = downstream.split();
                let (mut ru, mut wu) = upstream.split();
                let upstreaming = tokio::io::copy(&mut ru, &mut wd);
                let downstreaming = tokio::io::copy(&mut rd, &mut wu);
                let (_, _) = try_join(upstreaming, downstreaming).await.unwrap_or((0, 0));
            });
        }
    });
}
