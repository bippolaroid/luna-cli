use std::collections::HashMap;
use std::io::{self, Write};

use serde_json::Value;

#[macro_use]
extern crate rocket;

#[launch]
async fn rocket() -> _ {
    show_menu().await;
    rocket::build().mount("/", routes![hello])
}

async fn show_menu() {
    loop {
        println!("\nLunaCLI Menu");
        println!("------------------------");
        println!("1. Generate");
        println!("2. Exit");
        println!("------------------------");
        print!("Please enter your choice: ");

        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line.");

        match choice.trim() {
            "1" => {
                println!("Generating...");
                generate().await;
            }
            "2" => {
                println!("Exiting...");
                return;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }
}

async fn generate() {
    let mut map: HashMap<&str, serde_json::Value> = HashMap::new();

    map.insert(
        "model",
        serde_json::Value::String("bippy/cli-tool".to_string()),
    );
    map.insert("prompt", serde_json::Value::String("C:\\>".to_string()));
    map.insert("stream", serde_json::Value::Bool(false));

    let json_body = serde_json::to_string(&map).expect("Failed to serialize");
    println!("{}", json_body);

    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:11434/api/generate")
        .json(&map)
        .send()
        .await;

    match res {
        Ok(response) => {
            let response_text = response.text().await.unwrap_or_default();
            if let Ok(parsed) = serde_json::from_str::<Value>(&response_text) {
                if let Some(response_field) = parsed.get("response") {
                    println!("{}", response_field);
                } else {
                    println!("failed to parse");
                }
            } else {
                println!("error fetching")
            }
        }
        Err(_) => {
            println!("Error");
        }
    }
}

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}
