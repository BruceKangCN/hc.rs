use std::{
    error::Error,
    time::{Duration, Instant},
};

use clap::Parser;
use colored::Colorize;
use hc::{Args, Statistics};
use tracing::info;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn Error>> {
    let appender = RollingFileAppender::new(Rotation::DAILY, ".", "hc.log");
    let (nb_file, _file_guard) = tracing_appender::non_blocking(appender);
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(nb_file)
        .json()
        .init();

    let args = Args::parse();

    let client = reqwest::Client::builder()
        .timeout(Duration::from_millis(args.timeout.unwrap_or(5000)))
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

        let start = Instant::now();
        let result = client.get(&args.end_point).send().await;
        let rtt = start.elapsed();

        match result {
            Err(e) => {
                stat.failure += 1;

                let error = Some(&e);
                let status = Option::<reqwest::StatusCode>::None;
                let healthy = false;

                let msg = format!("#{} [Fail  ] {:?} {:?}", i, &e, rtt);
                println!("{}", msg.as_str().red());

                info!(i, ?rtt, ?error, ?status, healthy);
            }
            Ok(resp) if !resp.status().is_success() => {
                stat.error += 1;

                let error = Option::<reqwest::Error>::None;
                let status = Some(&resp.status());
                let healthy = false;

                let msg = format!("#{} [Error ] {:?} {:?}", i, &resp.status(), rtt);
                println!("{}", msg.as_str().yellow());

                info!(i, ?rtt, ?error, ?status, healthy);
            }
            Ok(resp) => {
                stat.success += 1;

                let error = Option::<reqwest::Error>::None;
                let status = Some(&resp.status());
                let healthy = true;

                let msg = format!("#{} [OK    ] {:?}", i, rtt);
                println!("{}", msg.as_str().green());

                info!(i, ?rtt, ?error, ?status, healthy);
            }
        }
    }

    println!("{}", "=".repeat(80));
    println!("success: {}", stat.success);
    println!("error: {}", stat.error);
    println!("failure: {}", stat.failure);
    println!("total: {}", stat.total());
    println!("health rate: {:.2}%", stat.health_rate() * 100.0);
    println!("fail rate: {:.2}%", stat.fail_rate() * 100.0);

    Ok(())
}
