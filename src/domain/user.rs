use crate::Db;

#[derive(Debug)]
pub struct User {
    pub id: Option<i32>,
    pub email: String,
}

impl User {
    pub fn from_email(email: String) -> Self {
        User { id: None, email }
    }

    pub async fn load(&self, conn: &Db) -> Option<Self> {
        let email = self.email.clone();
        match conn
            .run(move |c| {
                c.query_one(
                    "SELECT id, email FROM users WHERE email = $1 LIMIT 1",
                    &[&email],
                )
            })
            .await
        {
            Ok(row) => Some(User {
                id: Some(row.get(0)),
                email: row.get(1),
            }),
            Err(_) => None,
        }
    }

    pub async fn create(&self, conn: &Db) -> Option<Self> {
        let email = self.email.clone();
        match conn
            .run(move |c| c.execute("INSERT INTO users (email) VALUES ($1)", &[&email]))
            .await
        {
            Ok(_) => self.load(&conn).await,
            Err(_) => None,
        }
    }
}
