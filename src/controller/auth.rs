use rocket::State;
use rocket::response::Redirect;

use crate::utils::googleapi::GoogleApiSecrets;
use crate::utils::googleapi::GoogleAuthResponse;
use crate::utils::googleapi::get_user_info;

#[get("/?<google_auth_response..>")]
pub async fn google_auth(
    google_api_secrets: &State<GoogleApiSecrets>,
    google_auth_response: GoogleAuthResponse,
) -> Redirect {
    let google_user = get_user_info(&google_api_secrets, &google_auth_response).await;

    if google_user.is_err() {
        return Redirect::to(uri!("/"))
    }

    todo!()
}
