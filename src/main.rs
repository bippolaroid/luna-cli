use std::collections::HashMap;
use std::io::{self, Write};
use std::process::Command;

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
    map.insert(
        "prompt",
        serde_json::Value::String("C:\\Users\\guest\\Desktop>".to_string()),
    );
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
            let response_text = response.text().await.unwrap();
            if let Ok(parsed) = serde_json::from_str::<Value>(&response_text) {
                if let Some(response_field) = parsed.get("response") {
                    if let Value::String(ref response_str) = response_field {
                        let output = Command::new("cmd")
                            .args(&["/C"])
                            .args(&[&response_str])
                            .output()
                            .expect("Failed to execute process.");

                        if output.status.success() {
                            let stdout = String::from_utf8_lossy(&output.stdout);
                            println!("\nEXECUTING CODE: {}", &response_str);
                            println!("- - - - - - - - - -\n");
                            println!("{}", &stdout);
                            println!("- - - - - - - - - -");
                        } else {
                            let stderr = String::from_utf8_lossy(&output.stderr);
                            println!("Error: {}", stderr);
                            println!("{}", &response_str);
                        }
                    } else {
                        println!("Response field is not a string.");
                    }
                } else {
                    println!("Failed to parse response field.");
                }
            } else {
                println!("Error parsing response text.");
            }
        }
        Err(_) => {
            println!("Request error");
        }
    }
}

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}
