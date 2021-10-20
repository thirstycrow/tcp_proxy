use std::time::{Duration, Instant};

use glommio::prelude::*;

use cpu_time::ThreadTime;
use tokio::runtime::Builder;

thread_local! {
    static THREAD_TIME: ThreadTime = ThreadTime::now();
}

fn main() {
    LocalExecutorBuilder::new()
        .pin_to_cpu(0)
        .spawn(|| async move {
            println!("Glommio:");
            let start = Instant::now();

            THREAD_TIME.with(|_| {});

            for i in 1..=10000 {
                glommio::timer::sleep(Duration::from_millis(1)).await;
                if i % 1000 == 0 {
                    let elapsed = THREAD_TIME.with(|t| t.elapsed());
                    println!(
                        "repeated: {}, thread time used {:?}, avg: {:?}",
                        i,
                        elapsed,
                        elapsed / i
                    );
                }
            }

            println!("Total time: {:?}", start.elapsed());
        })
        .unwrap()
        .join()
        .unwrap();

    let runtime = Builder::new_multi_thread()
        .enable_all()
        .worker_threads(1)
        .build()
        .unwrap();

    runtime.block_on(async move {
        println!("Tokio:");
        let start = Instant::now();

        THREAD_TIME.with(|_| {});

        for i in 1..=10000 {
            tokio::time::sleep(Duration::from_millis(1)).await;
            if i % 1000 == 0 {
                let elapsed = THREAD_TIME.with(|t| t.elapsed());
                println!(
                    "repeated: {}, thread time used {:?}, avg: {:?}",
                    i,
                    elapsed,
                    elapsed / i
                );
            }
        }

        println!("Total time: {:?}", start.elapsed());
    });
}
