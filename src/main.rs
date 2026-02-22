use std::{error::Error, time::{Duration, Instant}};

use clap::Parser;
use hc::{Args, Statistics};

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()?;

    // watch for Ctrl-C signal
    let (stop_tx, mut stop_rx) = tokio::sync::watch::channel(());
    tokio::spawn(async move {
        if tokio::signal::ctrl_c().await.is_ok() {
            stop_tx.send(()).ok();
        }
    });

    let mut stat = Statistics::default();
    let mut interval = tokio::time::interval(Duration::from_millis(args.interval.unwrap_or(1000)));
    for i in 0..args.count.unwrap_or(std::usize::MAX) {
        tokio::select! {
            _ = interval.tick() => {},
            _ = stop_rx.changed() => break,
        }
        // interval.tick().await;

        let start = Instant::now();
        let result = client.get(&args.end_point).send().await;
        let elapsed = start.elapsed();

        match result {
            Err(e) => {
                stat.failure += 1;
                println!("#{} [Fail  ] {:?} {:?}", i, e, elapsed);
            }
            Ok(resp) if !resp.status().is_success() => {
                stat.error += 1;
                println!("#{} [Error ] {:?} {:?}", i, &resp.status(), elapsed);
            }
            Ok(_) => {
                stat.success += 1;
                println!("#{} [OK    ] {:?}", i, elapsed);
            }
        }
    }

    println!("");
    println!("success: {}", stat.success);
    println!("error: {}", stat.error);
    println!("failure: {}", stat.failure);
    println!("total: {}", stat.total());
    println!("heath rate: {:.2}%", stat.health_rate() * 100.0);
    println!("fail rate: {:.2}%", stat.fail_rate() * 100.0);

    Ok(())
}
