use super::env::{get, get_or_else};

pub struct GoogleApiSecrets {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub response_type: String,
    pub scope: String,
    pub access_type: String,
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
    }
}
