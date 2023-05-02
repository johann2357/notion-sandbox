use serde_json::json;
use std::env;

use dotenv::dotenv;
use tokio;

mod notion;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let notion_token = env::var("NOTION_TOKEN").expect("Environment variable NOTION_TOKEN not set");
    let database_id = "c6c99573-b3ad-4b1d-ac6e-d8c7f7f43007".to_string();
    let body = json!({
        "filter": {
            "or": [
                {
                    "property": "Name",
                    "rich_text": {
                        "contains": "Notas 2023-03-24"
                    }
                },
                {
                    "property": "Fecha",
                    "date": {
                        "equals": "2023-03-24"
                    }
                }
            ]
        },
        "sorts": [
            {
                "property": "Created",
                "direction": "ascending"
            }
        ]
    });

    let response = notion::database::find_pages(&database_id, body, &notion_token)
        .await
        .expect("Failed to get pages");

    for page in response.results {
        println!("{:#?}", page);
    }

    Ok(())
}
