mod user;

use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::{Config, NoTls};

pub struct PgService {
  pool: Pool<PostgresConnectionManager<NoTls>>,
}

impl PgService {
  fn get_config() -> Config {
    let username = std::env::var("POSTGRES_USERNAME")
        .expect("FATAL: Env variable POSTGRES_USERNAME missing.");

    let password = std::env::var("POSTGRES_PASSWORD")
        .expect("FATAL: Env variable POSTGRES_PASSWORD missing.");

    let host = std::env::var("POSTGRES_HOST")
        .expect("FATAL: Env variable POSTGRES_HOST missing.");

    let port = std::env::var("POSTGRES_PORT")
        .expect("FATAL: Env variable POSTGRES_PORT missing.")
        .parse::<u16>()
        .expect("FATAL: Env variable POSTGRES_PORT was not parseable into a u16 as required.");

    let db_name = std::env::var("POSTGRES_DB").expect(
      "FATAL: Env variable POSTGRES_DB missing.");

    Config::new()
        .user(&username)
        .password(password)
        .host(&host)
        .dbname(&db_name)
        .port(port)
        .to_owned()
  }

  pub async fn new() -> Self {
    let config = PgService::get_config();
    let manager = PostgresConnectionManager::new(config, NoTls);
    let pool = Pool::builder().build(manager).await.unwrap();
    PgService { pool }
  }

  pub async fn get(&self) -> Result<bb8::PooledConnection<'_, PostgresConnectionManager<NoTls>>, bb8::RunError<tokio_postgres::Error>> {
    self.pool.get().await
  }
}

impl Clone for PgService {
  fn clone(&self) -> Self {
    Self {
      pool: self.pool.clone()
    }
  }
}
