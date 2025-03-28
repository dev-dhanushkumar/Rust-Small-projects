use clap::Parser;
use reqwest::blocking::Client;
use std::time::Instant;
use colored::Colorize;

#[derive(Parser)]
#[command(name = "internet_speed_test", about = "A simple internet speed tester")]
struct Args {
    #[arg(short, long)]
    test: bool,
}

fn measure_download_speed() -> Result<f64, Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = "https://speedtest.ftp.otenet.gr/files/test100kb";

    println!("Testing download speed...");
    let start = Instant::now();
    let response = client.get(url).send()?;
    let bytes = response.bytes()?;
    let duration = start.elapsed().as_secs_f64();

    let bytes_per_second = bytes.len() as f64 / duration;
    let mbps = (bytes_per_second * 8.0) / 1_000_000.0;
    Ok(mbps)
}

fn measure_ping() -> Result<f64, Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = "https://www.google.com";

    println!("Testing ping...");
    let start = Instant::now();
    client.get(url).send()?;
    let ping_ms = start.elapsed().as_millis() as f64;
    Ok(ping_ms)
}

fn main() {
    let args = Args::parse();

    if  args.test {
        match measure_download_speed() {
            Ok(speed) => println!("Download Speed: {:.2} Mbps", speed.to_string().green()),
            Err(e) => eprint!("Error measuring download speed: {}", e),
        }

        match measure_ping() {
            Ok(ping) => println!("Ping: {:} ms", ping.to_string().yellow()),
            Err(e) => eprintln!("Error measuring ping: {}", e),
        }

        println!("Upload Speed: Not implemented yet");
    } else {
        println!("Use --test to run the speed test");
    }
}