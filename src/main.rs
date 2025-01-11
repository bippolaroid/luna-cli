use std::collections::HashMap;
use std::io::{self, Write};
use std::process::Command;
use tokio::sync::Mutex;

use serde_json::Value;

#[macro_use]
extern crate rocket;

lazy_static::lazy_static! {
    static ref PREVIOUS_COMMAND: Mutex<String> = Mutex::new(String::from("None"));
    static ref ROOT: Mutex<String> = Mutex::new(String::from("C:\\Users\\guest\\Desktop"));
    static ref PATH: Mutex<String> = Mutex::new(String::from("C:\\Users\\guest\\Desktop"));
    static ref PREV_OUTPUT: Mutex<String> = Mutex::new(String::from(""));
}

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

        let mut choice: String = String::new();
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

    let prev_output: tokio::sync::MutexGuard<'_, String> = PREV_OUTPUT.lock().await;
    let path: tokio::sync::MutexGuard<'_, String> = PATH.lock().await;

    map.insert(
        "prompt",
        serde_json::Value::String(format!("{}\n{}", &*prev_output, &path)),
    );
    map.insert("stream", serde_json::Value::Bool(false));

    let json_body: String = serde_json::to_string(&map).expect("Failed to serialize");
    println!("{}", json_body);

    let client: reqwest::Client = reqwest::Client::new();
    let res: Result<reqwest::Response, reqwest::Error> = client
        .post("http://localhost:11434/api/generate")
        .json(&map)
        .send()
        .await;

    match res {
        Ok(response) => {
            let response_text: String = response.text().await.unwrap();
            if let Ok(parsed) = serde_json::from_str::<Value>(&response_text) {
                if let Some(response_field) = parsed.get("response") {
                    if let Value::String(ref response_str) = response_field {
                        let output: std::process::Output = Command::new("cmd")
                            .args(&["/C"])
                            .args(&[response_str])
                            .output()
                            .expect("Failed to execute process.");

                        if output.status.success() {
                            let stdout: std::borrow::Cow<'_, str> =
                                String::from_utf8_lossy(&output.stdout);
                                let mut prev_output: tokio::sync::MutexGuard<'_, String> = PREV_OUTPUT.lock().await;
                                *prev_output = stdout.to_string();
                            println!("\nEXECUTING CODE: {}", response_str);
                            println!("- - - - - - - - - -\n");
                            println!("{}", stdout);
                            println!("- - - - - - - - - -");
                            push_command(response_str).await;
                            let previous_command: tokio::sync::MutexGuard<'_, String> =
                                PREVIOUS_COMMAND.lock().await;
                            println!("Previous: {:?}", *previous_command);
                        } else {
                            let stderr: std::borrow::Cow<'_, str> =
                                String::from_utf8_lossy(&output.stderr);
                            println!("Error: {}", stderr);
                            println!("{}", response_str);
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

async fn push_command(string: &str) {
    let mut previous_command: tokio::sync::MutexGuard<'_, String> = PREVIOUS_COMMAND.lock().await;
    *previous_command = String::from(string);
}

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}
