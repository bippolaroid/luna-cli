use rocket::{get, launch, routes};
use std::io::{self, Write};
use std::process::Command;

#[launch]
async fn rocket() -> _ {
    display_menu().await;
    rocket::build().mount("/", routes![hello])
}

async fn display_menu() {
    loop {
        println!("\nLunaCLI Menu\n------------------------");
        println!("1. Generate\n2. Exit\n------------------------");
        print!("Please enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice: String = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line.");
        match choice.trim() {
            "1" => {
                println!("\nGenerating...");
                send_request().await;
            }
            "2" => {
                println!("\nExiting...");
                return;
            }
            _ => println!("\nInvalid choice. Please try again."),
        }
    }
}

async fn send_request() {
    let client: reqwest::Client = reqwest::Client::new();

    let body: serde_json::Value = serde_json::json!({
        "model": "bippy/cli-tool",
        "messages": [{
            "role": "system",
            "content": "You are an assistant that directly executes in a Windows 11 command prompt.

            ASSISTANT KEYWORDS:
            The role 'user' is the user's request.
            The role 'assistant' is your previous response.
            The role 'output' is the output response of the command prompt.

            ASSISTANT POLICY:
            You must only respond with unformatted Windows 11 Command Prompt commands or scripts.
            You must pay specific attention to capitalization and syntax.
            You must not simulate or emulate execution or command prompt output.
            You must only respond one line at a time.
            You must adhere to these rules at all times - no exceptions.",
        },
        {
            "role": "output",
            "content": "C:\\Users\\mdeez>"

        },
        {
            "role": "user",
            "content": "List the directories."
        }],
        "stream": false
    });
    println!("\nUSER INPUT: {body}");

    let response: Result<reqwest::Response, reqwest::Error> = client
        .post("http://localhost:11434/api/chat")
        .json(&body)
        .send()
        .await;
    match response {
        Ok(res) => match res.text().await {
            Ok(text) => {
                let json: Result<serde_json::Value, serde_json::Error> =
                    serde_json::from_str(&text);
                match json {
                    Ok(json_response) => {
                        if let Some(response_value) = json_response.get("message") {
                            if let Some(content_value) = response_value.get("content") {
                                println!("ASSISTANT OUTPUT: {}", content_value);
                            }
                        } else {
                            println!("Response not found.");
                            println!("{}", json_response);
                        }
                    }
                    Err(e) => eprintln!("\nError parsing: {}", e),
                }
            }
            Err(e) => eprintln!("\nError reading response: {}", e),
        },
        Err(e) => eprintln!("\nError sending request: {}", e),
    }
}

fn execute_command(command: &str) {
    println!("Command: {}", command);
    let output = Command::new("cmd")
        .args(&["/C", command])
        .output()
        .expect("Failed to execute command");
    println!(
        "Command output: {}",
        String::from_utf8_lossy(&output.stdout)
    );
    println!("Command error: {}", String::from_utf8_lossy(&output.stderr));
}

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}
