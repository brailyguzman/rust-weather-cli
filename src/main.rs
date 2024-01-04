use reqwest::Error;
use std::io;

#[tokio::main]
async fn main() {
    println!("Welcome to the Rust::Weather::CLI");
    eprint!("[+] Enter Location: ");
    let mut location = String::new();
    io::stdin().read_line(&mut location).unwrap();
    location = location.trim().to_string();

    match get_weather_data(&location).await {
        Ok(data) => println!("Weather: {}", data),
        Err(error) => println!("Error: {}", error),
    };
}

async fn get_weather_data(location: &str) -> Result<String, Error> {
    let url = format!("https://wttr.in/{}?format=%C+%t", location);
    let body = reqwest::get(&url).await?.text().await?;
    Ok(body)
}
