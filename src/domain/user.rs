#[derive(Debug)]
pub struct User {
    pub id: Option<i32>,
    pub email: String
}

impl User {
    pub fn from_email(email: String) -> Self {
        User {
            id: None,
            email,
        }
    }
}
