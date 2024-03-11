use reqwest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let message = "Hello from Rocket client";
    let response = reqwest::Client::new()
        .post("http://localhost:8000/")
        .body(message.to_string())
        .send()
        .await?;

    println!("{}", response.text().await?);
    Ok(())
}
