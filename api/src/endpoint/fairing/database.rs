use log::error;
use rocket::{
    fairing::{self, Fairing, Info, Kind},
    http::Status,
    outcome::Outcome,
    request::FromRequest,
    Build, Orbit, Rocket,
};
use serde::Deserialize;
use sqlx::Postgres;

pub type PostgresPool = sqlx::Pool<Postgres>;

/// Provides access to the postgres database client
#[derive(Debug, Clone)]
pub struct PostgresDb(pub PostgresPool);

pub struct PostgresFairing;

#[derive(Debug, Clone, Deserialize)]
pub struct PostgresConfig {
    url: String,
}

impl PostgresFairing {
    pub fn new() -> Self {
        Self {}
    }
}

#[rocket::async_trait]
impl Fairing for PostgresFairing {
    fn info(&self) -> Info {
        Info {
            name: "Database Fairing",
            kind: Kind::Ignite | Kind::Shutdown,
        }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> fairing::Result {
        let config: PostgresConfig = rocket.figment().focus("database").extract().unwrap();
        match sqlx::PgPool::connect(&config.url).await {
            Ok(pool) => {
                sqlx::migrate!("./migrations").run(&pool).await.unwrap();
                Ok(rocket.manage(pool))
            }
            Err(err) => {
                error!("Failed to initialize postgres database client: {}", err);
                Err(rocket)
            }
        }
    }

    async fn on_shutdown(&self, rocket: &Rocket<Orbit>) {
        if let Some(pool) = rocket.state::<PostgresPool>() {
            pool.close().await;
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for PostgresDb {
    type Error = crate::endpoint::v1::error::Error;

    async fn from_request(
        request: &'r rocket::Request<'_>,
    ) -> rocket::request::Outcome<Self, Self::Error> {
        if let Some(pool) = request.rocket().state::<PostgresPool>() {
            // Usually we want to avoid clones at all cost but here this is totally fine.
            // This clone statement results in a `Arc::clone` which is basically a free operation
            // as it just copies the pointer and increments the reference count.
            return Outcome::Success(Self(pool.clone()));
        }

        error!(
            "The postgres pool seems to be uninitialized, did the database client initialize properly?"
        );
        Outcome::Error((Status::InternalServerError, Self::Error::DatabaseError))
    }
}
