#![allow(clippy::needless_return)]
mod client;
mod init;

fn main() {
    let token = init::request_api_key();

    let mut client = client::Client::new();

    let _ = client.close();
}
