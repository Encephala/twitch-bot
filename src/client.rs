use std::{any::Any, net::TcpStream};

use websocket::{self as ws, ClientBuilder};

pub struct Client {
    client: ws::sync::Client<ws::native_tls::TlsStream<TcpStream>>,
    session_info: Option<SessionInfo>,
}

impl Client {
    pub fn new() -> Self {
        let client = ClientBuilder::new("wss://eventsub.wss.twitch.tv/ws")
            .expect("Failed to parse WebSocket URL")
            .connect_secure(None)
            .expect("Failed to connect to WebSocket");

        let mut client = Client {
            client,
            session_info: None,
        };

        client.do_ping_pong();

        client.process_welcome_message();

        dbg!(&client.session_info);

        return client;
    }

    fn do_ping_pong(&mut self) {
        let message = self
            .client
            .recv_message()
            .expect("Didn't receive Ping message");

        let mut response = ws::Message::from(message);
        response.into_pong().expect("Received message wasn't Ping");

        self.client
            .send_message(&response)
            .expect("Failed to send Pong message");
    }

    // TODO error handling
    fn process_welcome_message(&mut self) {
        let message = self.receive_message().unwrap();

        let payload = message.payload.get("session").unwrap();

        let session_info: SessionInfo = serde_json::from_value(payload.clone())
            .expect("welcome message payload didn't contain session info");

        self.session_info = Some(session_info);
    }

    // TODO Fix return type (error)
    // TODO better error handling
    pub fn receive_message(&mut self) -> Result<Message, Box<dyn Any>> {
        let raw_message = self.client.recv_message().expect("Failed to get message");

        let message: Message = serde_json::from_str(
            String::from_utf8(raw_message.take_payload())
                .expect("Response wasn't contain valid UTF-8")
                .as_str(),
        )
        .expect("Failed to deserialize message");

        return Ok(message);
    }

    pub fn close(&mut self) -> Result<(), Box<dyn Any>> {
        self.client.shutdown().unwrap();

        return Ok(());
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct Message {
    metadata: Metadata,
    payload: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, serde::Deserialize)]
struct Metadata {
    message_id: String,
    message_type: MessageType,
    message_timestamp: RFCTimestamp,

    // These are only used in Notification and Revocation messages
    subscription_type: Option<String>,
    subscription_version: Option<String>,
}

#[derive(Debug)]
enum MessageType {
    Welcome,
    Notification,
    Keepalive,
    Reconnect,
    Revocation,
}

impl<'de> serde::Deserialize<'de> for MessageType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        return deserializer.deserialize_str(MessageTypeVisitor);
    }
}

struct MessageTypeVisitor;
impl<'de> serde::de::Visitor<'de> for MessageTypeVisitor {
    type Value = MessageType;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        return formatter.write_str("a string describing a Twitch API message type");
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        return match value {
            "session_welcome" => Ok(MessageType::Welcome),
            "notification" => Ok(MessageType::Notification),
            "session_keepalive" => Ok(MessageType::Keepalive),
            "session_reconnect" => Ok(MessageType::Reconnect),
            "revocation" => Ok(MessageType::Revocation),
            _ => Err(E::invalid_value(
                serde::de::Unexpected::Str(value),
                &"a valid message type",
            )),
        };
    }
}

#[derive(Debug, serde::Deserialize)]
struct RFCTimestamp(#[serde(with = "time::serde::rfc3339")] time::OffsetDateTime);

#[derive(Debug, serde::Deserialize)]
struct SessionInfo {
    id: String,
    connected_at: ISOTimestamp,
    keepalive_timeout_seconds: usize,
}

#[derive(Debug, serde::Deserialize)]
struct ISOTimestamp(#[serde(with = "time::serde::iso8601")] time::OffsetDateTime);
