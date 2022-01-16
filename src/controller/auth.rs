use rocket::http::Cookie;
use rocket::http::CookieJar;
use rocket::response::Redirect;
use rocket::State;

use crate::domain::user::User;
use crate::utils::googleapi::get_user_info;
use crate::utils::googleapi::GoogleApiSecrets;
use crate::utils::googleapi::GoogleAuthResponse;
use crate::Db;

#[get("/?<google_auth_response..>")]
pub async fn google_auth(
    cookies: &CookieJar<'_>,
    conn: Db,
    google_api_secrets: &State<GoogleApiSecrets>,
    google_auth_response: GoogleAuthResponse,
) -> Redirect {
    let google_user = get_user_info(&google_api_secrets, &google_auth_response).await;
    match google_user {
        Ok(google_user) => {
            let user = User::from_email(google_user.email);
            load_user(&cookies, &conn, &user).await
        }
        Err(_) => Redirect::to(uri!("/errors/user?status=404")),
    }
}

#[get("/logout")]
pub fn logout(cookies: &CookieJar<'_>) -> Redirect {
    cookies.remove_private(Cookie::named("poucostrocos"));
    Redirect::to(uri!("/"))
}

async fn load_user(cookies: &CookieJar<'_>, conn: &Db, user: &User) -> Redirect {
    let result = user.load(&conn).await;
    match result {
        Some(user) => sign_in_user(&cookies, &user),
        None => create_user(&cookies, &conn, &user).await,
    }
}

async fn create_user(cookies: &CookieJar<'_>, conn: &Db, user: &User) -> Redirect {
    let result = user.create(&conn).await;
    match result {
        Some(user) => sign_in_user(&cookies, &user),
        None => Redirect::to(uri!("/errors/user?status=404")),
    }
}

fn sign_in_user(cookies: &CookieJar<'_>, user: &User) -> Redirect {
    let cookie = Cookie::build("poucostrocos", user.email.clone())
        .path("/")
        .secure(true)
        .http_only(true)
        .finish();

    cookies.add_private(cookie);
    Redirect::to(uri!("/"))
}
