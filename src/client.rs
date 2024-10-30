use websocket::{ClientBuilder, sync::Client, Message};

pub fn connect() -> Client<websocket::native_tls::TlsStream<std::net::TcpStream>> {
    let mut client = ClientBuilder::new("wss://eventsub.wss.twitch.tv/ws")
        .expect("Failed to parse WebSocket URL")
        .connect_secure(None)
        .expect("Failed to connect to WebSocket");

    // Do ping-pong
    let msg = client.recv_message()
        .expect("Didn't receive Ping message");

    Message::from(msg).into_pong()
        .expect("Failed to send Pong message");

    return client;
}
