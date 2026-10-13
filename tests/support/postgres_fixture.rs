use humanizar_nucleo::infrastructure::config::{DatabaseConfig, DatabaseSettings};
use std::time::Duration;
use tokio_postgres::{Client, NoTls};
use uuid::Uuid;

pub struct PostgresFixture {
    pub database: DatabaseConfig,
    pub client: Client,
    connection: tokio::task::JoinHandle<()>,
    schema: String,
}

impl PostgresFixture {
    pub async fn start() -> Self {
        let url = std::env::var("NUCLEO_TEST_DB_URL")
            .expect("executar scripts/run_checks.py para criar recursos isolados");
        let username = std::env::var("NUCLEO_TEST_DB_USERNAME").expect("usuario fixture");
        let password = std::env::var("NUCLEO_TEST_DB_PASSWORD").expect("senha fixture");
        assert_eq!(
            url::Url::parse(&url).expect("url").host_str(),
            Some("127.0.0.1")
        );
        let mut config = url.parse::<tokio_postgres::Config>().expect("config");
        config.user(&username).password(&password);
        let (client, connection) = config.connect(NoTls).await.expect("PostgreSQL isolado");
        let connection = tokio::spawn(async move {
            let _ = connection.await;
        });
        let schema = format!("nucleo_test_{}", Uuid::new_v4().simple());
        client
            .batch_execute(&format!(
                "CREATE SCHEMA {schema}; SET search_path TO {schema}"
            ))
            .await
            .expect("schema fixture");
        client
            .batch_execute(include_str!("../../migrations/0001_initial_schema.sql"))
            .await
            .expect("schema inicial");
        let mut database_url = url::Url::parse(&url).expect("url");
        database_url
            .query_pairs_mut()
            .append_pair("options", &format!("-c search_path={schema}"));
        let settings = DatabaseSettings::new(
            database_url.as_str().replace('+', "%20"),
            username,
            password,
        )
        .expect("settings")
        .with_pool_policy(
            1,
            10,
            Duration::from_secs(5),
            Duration::from_secs(30),
            Duration::from_secs(60),
        )
        .expect("pool settings");
        let database = DatabaseConfig::initialize(settings)
            .await
            .expect("pool fixture");
        Self {
            database,
            client,
            connection,
            schema,
        }
    }

    pub async fn close(self) {
        self.client
            .batch_execute(&format!("DROP SCHEMA {} CASCADE", self.schema))
            .await
            .expect("cleanup schema");
        self.connection.abort();
    }
}
