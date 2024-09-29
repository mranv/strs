use reqwest::blocking::Client;
use std::time::Instant;

const DOWNLOAD_URL: &str = "https://speed.cloudflare.com/__down?bytes=100000000";
const UPLOAD_URL: &str = "https://speed.cloudflare.com/__up";
const UPLOAD_SIZE: usize = 10_000_000; // 10 MB

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Cross-platform Rust Internet Speed Test");

    let client = Client::new();

    // Download test
    println!("\nPerforming download test...");
    let start = Instant::now();
    let response = client.get(DOWNLOAD_URL).send()?;
    let content = response.bytes()?;
    let duration = start.elapsed();
    let download_speed = content.len() as f64 / duration.as_secs_f64() / 1_000_000.0;
    println!("Download speed: {:.2} Mbps", download_speed * 8.0);

    // Upload test
    println!("\nPerforming upload test...");
    let data = vec![0u8; UPLOAD_SIZE];
    let start = Instant::now();
    let _response = client.post(UPLOAD_URL).body(data).send()?;
    let duration = start.elapsed();
    let upload_speed = UPLOAD_SIZE as f64 / duration.as_secs_f64() / 1_000_000.0;
    println!("Upload speed: {:.2} Mbps", upload_speed * 8.0);

    // Ping test
    println!("\nPerforming ping test...");
    let start = Instant::now();
    client.get("https://www.google.com").send()?;
    let ping = start.elapsed();
    println!("Ping: {} ms", ping.as_millis());

    Ok(())
}