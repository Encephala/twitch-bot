const ENV_NAME_ID: &str = "TWITCH_CLIENT_ID";
const ENV_NAME_SECRET: &str = "TWITCH_CLIENT_SECRET";
const TWITCH_ACCESS_TOKEN_URL: &str = "https://id.twitch.tv/oauth2/token";

fn load_env_variables() -> (String, String) {
    dotenv::dotenv().expect("Could not find .env file");

    let id = std::env::var(ENV_NAME_ID).expect("Could not find `TWITCH_API_KEY` variable");
    let secret = std::env::var(ENV_NAME_SECRET).expect("Could not find `TWITCH_API_KEY` variable");

    return (id, secret);
}

#[derive(serde::Deserialize)]
struct ApiKeyResponse {
    access_token: String,
}

pub fn request_api_key() -> String {
    let (id, secret) = load_env_variables();

    let params = [
        ("client_id", id.as_str()),
        ("client_secret", secret.as_str()),
        ("grant_type", "client_credentials"),
    ];

    let client = reqwest::blocking::Client::new();

    let response = client
        .post(TWITCH_ACCESS_TOKEN_URL)
        .form(&params)
        .send()
        .expect("Failed to get OAuth token")
        .json::<ApiKeyResponse>()
        .expect("Response didn't contain an access token");

    return response.access_token;
}
