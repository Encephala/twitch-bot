#![allow(clippy::needless_return)]
mod init;

fn main() {
    let token = init::request_api_key();

    println!("Got token '{token}'");
}
