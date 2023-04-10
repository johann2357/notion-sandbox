use std::env;

use dotenv::dotenv;
use tokio;

mod notion;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the environment variables from the .env file
    dotenv().ok();
    let _notion_token =
        env::var("NOTION_TOKEN").expect("Environment variable NOTION_TOKEN not set");

    println!("Hello, world!");

    Ok(())
}
