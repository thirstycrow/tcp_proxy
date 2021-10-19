use std::{collections::HashSet, time::Duration};

use futures_lite::{
    future::try_zip,
    io::{copy, split},
};
use glommio::{
    enclose,
    net::{TcpListener, TcpStream},
    prelude::*,
    spawn_local, CpuSet,
};

use common::parse_config;
use cpu_time::ThreadTime;

mod common;

fn main() {
    pretty_env_logger::init();

    let config = parse_config("Glommio");

    let cpuset = {
        let mut selected = HashSet::new();
        CpuSet::online()
            .unwrap()
            .filter(|loc| {
                config.use_hyper_thread || selected.insert((loc.numa_node, loc.package, loc.core))
            })
            .filter(|loc| config.numa_node.map_or(true, |node| loc.numa_node == node))
    };

    LocalExecutorPoolBuilder::new(config.nr_shards.unwrap())
        .placement(Placement::MaxSpread(Some(cpuset)))
        .spin_before_park(Duration::from_millis(config.spin_before_park as _))
        .on_all_shards(enclose!((config) move || async move {
            let tq = executor().create_task_queue(Shares::Static(1000), Latency::Matters(Duration::from_micros(500)), "default");

            spawn_local_into(async move {
                let thread_time = ThreadTime::now();
                spawn_local(async move {
                    let mut last = Duration::ZERO;
                    loop {
                        glommio::timer::sleep(Duration::from_millis(1000)).await;
                        let elapsed = thread_time.elapsed();
                        println!("{:?}", elapsed - last);
                        last = elapsed;
                    }
                }).detach();

                let bind_address = (config.bind_address.as_str(), config.bind_port);
                let listener = TcpListener::bind(bind_address).unwrap();
                while let Ok(downstream) = listener.accept().await {
                    spawn_local(enclose!((config) async move {
                        let upstream = TcpStream::connect((config.upstream_address.as_str(), config.upstream_port)).await.unwrap();
                        let (mut rd, mut wd) = split(downstream);
                        let (mut ru, mut wu) = split(upstream);
                        let upstreaming = copy(&mut rd, &mut wu);
                        let downstreaming = copy(&mut ru, &mut wd);
                        let (_, _) = try_zip(downstreaming, upstreaming).await.unwrap_or((0, 0));
                    })).detach();
                }
            }, tq).unwrap().await;
        }))
        .unwrap()
        .join_all();
}
