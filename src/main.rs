#![allow(clippy::needless_return)]
mod client;
mod init;

use serde_json::{Map, Value};

fn main() {
    let (client_id, user_access_token) = init::get_user_access_token();

    let mut client = client::Client::new();

    // Validate token
    let validation_response = reqwest::blocking::Client::new()
        .get("https://id.twitch.tv/oauth2/validate")
        .bearer_auth(&user_access_token)
        .send()
        .expect("Failed to validate token");

    println!(
        "User token validation response: {}",
        validation_response
            .text()
            .expect("Validation response didn't contain text")
    );

    // Get app access token
    let app_access_token = init::get_app_access_token();

    println!("App token: {app_access_token}");

    let validation_response = reqwest::blocking::Client::new()
        .get("https://id.twitch.tv/oauth2/validate")
        .bearer_auth(&app_access_token)
        .send()
        .expect("Failed to validate token");

    println!(
        "App token validation response: {}",
        validation_response
            .text()
            .expect("Validation response didn't contain text")
    );

    // beun beun
    let mut eventsub_message_transport = Map::new();
    eventsub_message_transport.insert("method".to_owned(), Value::String("websocket".to_owned()));
    eventsub_message_transport.insert(
        "session_id".to_owned(),
        Value::String(client.get_session_id().unwrap()),
    );

    let mut eventsub_message_condition = Map::new();
    eventsub_message_condition.insert(
        "broadcaster_user_id".to_owned(),
        Value::String("46130270".to_owned()),
    );
    eventsub_message_condition.insert("user_id".to_owned(), Value::String("46130270".to_owned()));

    let mut eventsub_message: Map<String, Value> = Map::new();
    eventsub_message.insert(
        "transport".to_owned(),
        Value::Object(eventsub_message_transport),
    );
    eventsub_message.insert(
        "condition".to_owned(),
        Value::Object(eventsub_message_condition),
    );
    eventsub_message.insert(
        "type".to_owned(),
        Value::String("channel.chat.message".to_owned()),
    );
    eventsub_message.insert("version".to_owned(), Value::String("1".to_owned()));

    let request = reqwest::blocking::Client::new()
        .post("https://api.twitch.tv/helix/eventsub/subscriptions")
        .bearer_auth(&app_access_token)
        .header("Client-Id", client_id)
        .header("Content-Type", "application/json")
        .json(&eventsub_message)
        .send()
        .expect("Failed to send EventSub request");

    println!("EventSub response: {}", request.text().unwrap());

    loop {
        let message = client.receive_message().unwrap();

        dbg!(&message);
    }

    let _ = client.close();
}
