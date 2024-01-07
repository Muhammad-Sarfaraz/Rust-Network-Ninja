use indicatif::{ProgressBar, ProgressStyle};
use reqwest;
use std::env;
use std::time::Instant;
use std::borrow::Cow;

mod utils{pub mod network;}
use utils::network::check_url_availability;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: cargo run -- --speed");
        println!("Usage: cargo run -- --check --url <URL>");
        return;
    }

    if args[1] == "--speed" {
        show_network_speed().await;
    } else if args[1] == "--check" {
        if args.len() < 4 || args[2] != "--url" {
            println!("Usage: cargo run -- --check --url <URL>");
            return;
        }

        let url = &args[3];
        check_url_availability(url).await;
    } else {
        println!("Invalid command");
    }
}

async fn show_network_speed() {
    let url = "https://example.com"; // Replace with a URL that you want to test download speed

    let client = reqwest::Client::new();
    let start_time = Instant::now();

    match client.get(url).send().await {
        Ok(_) => {
            let elapsed_time = start_time.elapsed().as_secs_f64();
            let download_speed = (10.0 / elapsed_time) * 8.0; // Assuming 10 MB file size
            println!("Download speed: {:.2} Mbps", download_speed);
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}


