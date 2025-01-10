use std::collections::HashMap;

use serde_json::Value;

#[macro_use]
extern crate rocket;

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[launch]
async fn rocket() -> _ {
    let mut map: HashMap<&str, serde_json::Value> = HashMap::new();

    map.insert(
        "model",
        serde_json::Value::String("bippy/luna1".to_string()),
    );
    map.insert("prompt", serde_json::Value::String("Hello!".to_string()));
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

    rocket::build().mount("/", routes![hello])
}
