use rocket::{response::Redirect, State};

use crate::utils::googleapi::GoogleApiSecrets;

#[get("/")]
pub fn index(google_api_secrets: &State<GoogleApiSecrets>) -> Redirect {
    let url = format!(
        "https://accounts.google.com/o/oauth2/v2/auth?client_id={}&redirect_uri={}&response_type={}&scope={}&access_type={}",
        google_api_secrets.client_id,
        google_api_secrets.redirect_uri,
        google_api_secrets.response_type,
        google_api_secrets.scope,
        google_api_secrets.access_type
    );
    Redirect::to(url)
}

#[cfg(test)]
mod tests {
    use rocket::{response::Redirect, State};

    use crate::utils::googleapi::GoogleApiSecrets;

    use super::index;

    #[test]
    fn index_redirect() {
        let google_api_secrets = GoogleApiSecrets {
            client_id: String::from("client_id"),
            client_secret: String::from("client_secret"),
            redirect_uri: String::from("redirect_uri"),
            response_type: String::from("response_type"),
            scope: String::from("scope"),
            access_type: String::from("access_type"),
        };

        let state = State::from(&google_api_secrets);
        let url = format!(
            "https://accounts.google.com/o/oauth2/v2/auth?client_id={}&redirect_uri={}&response_type={}&scope={}&access_type={}",
            google_api_secrets.client_id,
            google_api_secrets.redirect_uri,
            google_api_secrets.response_type,
            google_api_secrets.scope,
            google_api_secrets.access_type
        );

        let redirect = index(state);

        assert_eq!(
            format!("{:?}", redirect),
            format!("{:?}", Redirect::to(url))
        )
    }
}
