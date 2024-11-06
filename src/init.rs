const ENV_NAME_ID: &str = "TWITCH_CLIENT_ID";
const ENV_NAME_SECRET: &str = "TWITCH_CLIENT_SECRET";
const ENV_NAME_TOKEN: &str = "TWITCH_USER_ACCESS_TOKEN";

const TWITCH_ACCESS_TOKEN_URL: &str = "https://id.twitch.tv/oauth2/token";

pub fn get_user_access_token() -> (String, String) {
    dotenv::dotenv().expect("Could not find .env file");

    // TODO: Not hardcode error message, or just fix error handling xdd
    let id = std::env::var(ENV_NAME_ID).expect("Could not find `TWITCH_CLIENT_ID` variable");
    let token =
        std::env::var(ENV_NAME_TOKEN).expect("Could not find `TWITCH_USER_ACCESS_TOKEN` variable");

    return (id, token);
}

fn load_secret_from_env() -> (String, String) {
    dotenv::dotenv().expect("Could not find .env file");

    // TODO: Not hardcode error message, or just fix error handling xdd
    let id = std::env::var(ENV_NAME_ID).expect("Could not find `TWITCH_CLIENT_ID` variable");
    let secret =
        std::env::var(ENV_NAME_SECRET).expect("Could not find `TWITCH_CLIENT_SECRET` variable");

    return (id, secret);
}

#[derive(serde::Deserialize)]
struct ApiKeyResponse {
    access_token: String,
}

pub fn get_app_access_token() -> String {
    let (id, secret) = load_secret_from_env();

    // TODO: I have to get a user access token, not an app access token
    // I think
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
        .expect("Failed to get OAuth App Access Token")
        .json::<ApiKeyResponse>()
        .expect("Response wasn't in expected format");

    return response.access_token;
}
