use super::env::get;
use super::env::get_or_else;

use reqwest::Client;
use reqwest::Error;

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug)]
pub struct GoogleApiSecrets {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub response_type: String,
    pub scope: String,
    pub access_type: String,
    pub authorization_type: String,
}

#[derive(FromForm, Debug)]
pub struct GoogleAuthResponse {
    pub code: String,
    pub scope: String,
    pub authuser: u8,
    pub prompt: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleAuthToken {
    pub access_token: String,
    pub expires_in: u16,
    pub token_type: String,
    pub scope: String,
    pub refresh_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleUser {
    pub id: String,
    pub email: String,
    pub verified_email: bool,
    pub picture: String,
}

pub fn load() -> GoogleApiSecrets {
    GoogleApiSecrets {
        client_id: get(String::from("GOOGLE_CLIENT_ID")),
        client_secret: get(String::from("GOOGLE_CLIENT_SECRET")),
        redirect_uri: get(String::from("GOOGLE_REDIRECT_URL")),
        response_type: get_or_else(String::from("GOOGLE_RESPONSE_TYPE"), String::from("code")),
        scope: get_or_else(
            String::from("GOOGLE_SCOPE"),
            String::from("https://www.googleapis.com/auth/userinfo.email"),
        ),
        access_type: get_or_else(String::from("GOOGLE_ACCESS_TYPE"), String::from("online")),
        authorization_type: get_or_else(
            String::from("GOOGLE_AUTHORIZATION_TYPE"),
            String::from("authorization_code"),
        ),
    }
}

pub async fn get_user_info(
    google_api_secrets: &GoogleApiSecrets,
    google_auth_response: &GoogleAuthResponse
) -> Result<GoogleUser, Error> {
    let api_url = "https://oauth2.googleapis.com/token";
    let params = [
        ("code", &google_auth_response.code),
        ("client_id", &google_api_secrets.client_id),
        ("client_secret", &google_api_secrets.client_secret),
        ("redirect_uri", &google_api_secrets.redirect_uri),
        ("grant_type", &google_api_secrets.authorization_type),
    ];

    let client = Client::new();
    client
        .post(api_url)
        .form(&params)
        .send()
        .await?
        .json::<GoogleAuthToken>()
        .await?
        .get_user_info()
        .await
}

impl GoogleAuthToken {
    async fn get_user_info(&self) -> Result<GoogleUser, Error> {
        let client = Client::new();
        let api_url = format!(
            "https://www.googleapis.com/oauth2/v1/userinfo?access_token={}",
            self.access_token
        );

        client.get(api_url).send().await?.json::<GoogleUser>().await
    }
}
