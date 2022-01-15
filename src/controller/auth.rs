use rocket::response::Redirect;
use rocket::State;

use crate::domain::user::User;
use crate::utils::googleapi::get_user_info;
use crate::utils::googleapi::GoogleApiSecrets;
use crate::utils::googleapi::GoogleAuthResponse;
use crate::Db;

#[get("/?<google_auth_response..>")]
pub async fn google_auth(
    conn: Db,
    google_api_secrets: &State<GoogleApiSecrets>,
    google_auth_response: GoogleAuthResponse,
) -> Redirect {
    let google_user = get_user_info(&google_api_secrets, &google_auth_response).await;
    match google_user {
        Ok(google_user) => {
            let user = User::from_email(google_user.email);
            load_user(&conn, &user).await
        }
        Err(_) => Redirect::to(uri!("/errors/user?status=404")),
    }
}

async fn load_user(conn: &Db, user: &User) -> Redirect {
    let result = user.load(&conn).await;
    match result {
        Some(user) => todo!("{:?}", user),
        None => create_user(&conn, &user).await
    }
}

async fn create_user(conn: &Db, user: &User) -> Redirect {
    let result = user.create(&conn).await;
    match result {
       Some(user) => todo!("{:?}", user),
       None => Redirect::to(uri!("/errors/user?status=404")),
    }
}
