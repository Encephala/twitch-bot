#![allow(clippy::needless_return)]
use core::str;

mod init;
mod client;

fn main() {
    let token = init::request_api_key();

    println!("Got token '{token}'");
}
