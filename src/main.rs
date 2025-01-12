use once_cell::sync::Lazy;
use serde_json::{json, Value};
use std::io::{self, Write};
use std::process::Command;
use tokio::sync::Mutex;

pub static BODY: Lazy<Mutex<Value>> = Lazy::new(|| {
    Mutex::new(json!({
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
        You must respond very explicitly with what the user requests - nothing more or less.
        You must only respond one line at a time.
        You must adhere to these rules at all times - no exceptions.",
        },
        {
            "role": "output",
            "content": "C:\\Users\\mdeez>"

        }],
        "stream": false
    }))
});

#[tokio::main]
async fn main() {
    display_menu().await;
}

async fn display_menu() {
    println!("\nLunaCLI is now running...");
    println!("\nType 'exit' to exit.");

    loop {
        print!("\n[LunaCLI] C:\\> ");
        io::stdout().flush().unwrap();

        let mut choice: String = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line.");
        match choice.trim() {
            "exit" => {
                println!("\nExiting...\n");
                return;
            }
            _ => {
                println!("\nGenerating...");
                send_request(choice).await;
            }
        }
    }
}

async fn send_request(user_request: String) {
    let client: reqwest::Client = reqwest::Client::new();

    let mut body: tokio::sync::MutexGuard<'_, Value> = BODY.lock().await;

    if let Some(Value::Array(messages)) = body.get_mut("messages") {
        messages.push(json!({
            "role": "user",
            "content": user_request,
        }));
    }
    println!("\nUSER INPUT: {body}");

    let response: Result<reqwest::Response, reqwest::Error> = client
        .post("http://localhost:11434/api/chat")
        .json(&*body)
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
                                println!("\nASSISTANT OUTPUT: {}", content_value);
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
