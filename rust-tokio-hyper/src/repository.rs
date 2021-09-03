use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::{Error, Pool, Postgres};
use sqlx::postgres::PgPoolOptions;

#[derive(Serialize, Deserialize)]
pub struct Contact {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub phone: String,
    pub email: String,
}

#[async_trait]
pub trait Repository {
    async fn new(dsl: &str) -> Self;
    async fn get(&self, id: i32) -> Result<Contact, Error>;
    async fn save(&self, contact: &Contact) -> Result<i32, Error>;
}

pub struct PgsqlRepository {
    pool: Pool<Postgres>,
}

#[async_trait]
impl Repository for PgsqlRepository {
    async fn new(url: &str) -> Self {
        let pool = PgPoolOptions::new()
            .connect(url)
            .await
            .unwrap();
        Self { pool }
    }

    async fn get(&self, id: i32) -> Result<Contact, Error> {
        let row = sqlx::query!(r#"SELECT id, firstname, lastname, phone, email FROM contact WHERE id=$1"#, id).fetch_one(&self.pool).await?;
        Ok(Contact {
            id: row.id,
            firstname: row.firstname.unwrap_or(String::from("")),
            lastname: row.lastname,
            phone: row.phone.unwrap_or(String::from("")),
            email: row.email.unwrap_or(String::from("")),
        })
    }

    async fn save(&self, contact: &Contact) -> Result<i32, Error> {
        Ok(sqlx::query!(r#"INSERT INTO contact (id, firstname, lastname, phone, email) VALUES ($1, $2, $3, $4, $5) returning id"#,
                               contact.id, contact.firstname, contact.lastname, contact.phone, contact.email).fetch_one(&self.pool).await.unwrap().id)
    }
}

#[cfg(test)]
mod tests {
    use crate::repository::{PgsqlRepository, Repository, Contact};
    use test_context::{test_context, AsyncTestContext};
    use tokio_postgres::{NoTls};
    use async_trait::async_trait;
    use serial_test::serial;
    use sqlx::postgres::PgPoolOptions;

    struct PgContext {
        repository: PgsqlRepository,
    }

    #[async_trait]
    impl AsyncTestContext for PgContext {
        async fn setup() -> PgContext {
            let pool = PgPoolOptions::new()
                .connect("postgresql://test:test@postgresql/test")
                .await
                .unwrap();
            PgContext { repository: PgsqlRepository { pool } }
        }

        async fn teardown(self) {
            sqlx::query!(
                r#"
                DELETE FROM contact
                "#
            )
                .execute(&self.repository.pool)
                .await
                .unwrap();
        }
    }

    #[test_context(PgContext)]
    #[tokio::test]
    #[serial]
    async fn get_contact_no_contact(ctx: &PgContext) {
        assert!(ctx.repository.get(12).await.is_err(), "no results should be found")
    }

    #[test_context(PgContext)]
    #[tokio::test]
    #[serial]
    async fn save_get_contact(ctx: &PgContext) {
        let contact = Contact {
            id: 13,
            firstname: "first".to_string(),
            lastname: "second".to_string(),
            phone: "0123456789".to_string(),
            email: "e@mail.com".to_string(),
        };
        assert!(ctx.repository.save(&contact).await.is_ok(), "save should succeed");
        assert!(ctx.repository.get(13).await.is_ok(), "contact should be found")
    }

    #[test_context(PgContext)]
    #[tokio::test]
    #[serial]
    async fn save_get_contact_with_empty_fields(ctx: &PgContext) {
        let calendarid = sqlx::query!(r#"INSERT INTO contact (id, lastname) VALUES ($1,$2) returning id"#, 14, "foo").fetch_one(&ctx.repository.pool).await.unwrap().id;
        let contact = match ctx.repository.get(14).await {
            Ok(contact) => contact,
            Err(error) => { panic!("error : {:?}", error) }
        };
        assert_eq!(contact.id, 14);
        assert_eq!(contact.firstname, String::from(""));
        assert_eq!(contact.lastname, "foo");
        assert_eq!(contact.phone, String::from(""));
        assert_eq!(contact.email, String::from(""))
    }
}