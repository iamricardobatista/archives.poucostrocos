use rocket::State;
use rocket::response::Redirect;

use crate::domain::user::User;
use crate::utils::googleapi::GoogleApiSecrets;
use crate::utils::googleapi::GoogleAuthResponse;
use crate::utils::googleapi::get_user_info;

#[get("/?<google_auth_response..>")]
pub async fn google_auth(
    google_api_secrets: &State<GoogleApiSecrets>,
    google_auth_response: GoogleAuthResponse,
) -> Redirect {
    let google_user = get_user_info(&google_api_secrets, &google_auth_response).await;
    match google_user {
        Ok(google_user) => login_user(User::from_email(google_user.email)),
        Err(_) => Redirect::to(uri!("/errors/user?status=404")),
    }
}

fn login_user(user: User) -> Redirect {
    todo!("{:?}", user)
}
