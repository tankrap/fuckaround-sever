use std::path::Path;

use crate::models::{Config, StructToString};
use anyhow::Result;
use redis::{AsyncCommands, Client, aio::ConnectionManager};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Postgres, migrate::Migrator, postgres::PgRow};

pub struct Database
{
    pub inner: Pool<Postgres>,
    pub redis: ConnectionManager,
}

impl Database
{
    pub async fn init(dsn: &str, migrations: &str, redis: &str) -> Result<Self>
    {
        let _current_env =
            std::env::var("CURRENT_ENV").unwrap_or("dev".to_string());
        let pool = Pool::connect(dsn).await.unwrap();
        let migrator = Migrator::new(Path::new(migrations)).await.unwrap();
        //migrator.run(&pool).await.unwrap();
        Ok(Self {
            inner: pool,
            redis: ConnectionManager::new(Client::open(redis).unwrap())
                .await?,
        })
    }
    pub async fn update_config<T: Serialize + for<'a> Deserialize<'a>>(
        &self,
        key: &str,
        val: T,
    ) -> Result<T>
    {
        sqlx::query::<_>(r#"INSERT INTO goit.config ("key", "value") VALUES ($2, $1) ON CONFLICT ("key") DO UPDATE SET value = ($1)"#)
            .bind(&serde_json::to_value(&val)?)
            .bind(key)
            .fetch_all(&self.inner)
            .await?;
        let mut reids_conn = self.redis.clone();
        let table_name = Config::get_db_name();
        let redis_key: String = format!("goit-{table_name}-key");
        reids_conn.del::<&str, String>(&redis_key).await?;
        return self.get_config(key).await;
    }
    pub async fn get_config<T: Serialize + for<'a> Deserialize<'a>>(
        &self,
        key: &str,
    ) -> Result<T>
    {
        let value_from_db = self.simple_keyed_get::<Config>("key", key).await?;
        let deserialized = serde_json::from_value::<T>(value_from_db.value)?;
        Ok(deserialized)
    }
    pub fn assemble_redis_key<T: StructToString>(key: &str) -> String
    {
        let table_name = T::get_db_name();
        let redis_key: String = format!("goit-{table_name}-{key}");
        redis_key
    }
    pub async fn reset_cache<T: StructToString>(&self, key_val: &str) -> bool
    {
        let mut reids_conn = self.redis.clone();
        let redis_key = Database::assemble_redis_key::<T>(key_val);
        return reids_conn.del::<&str, ()>(&redis_key).await.is_ok();
    }
    pub async fn simple_keyed_get<
        T: StructToString
            + Serialize
            + for<'a> Deserialize<'a>
            + for<'a> FromRow<'a, PgRow>
            + Unpin
            + Send,
    >(
        &self,
        key_name: &str,
        key: &str,
    ) -> Result<T>
    {
        let table_name = T::get_db_name();
        let mut reids_conn = self.redis.clone();
        let redis_key = Database::assemble_redis_key::<T>(key);
        let from_redis =
            reids_conn.get::<&str, Option<String>>(&redis_key).await?;
        if let Some(from_redis) = from_redis {
            return Ok(serde_json::from_str::<T>(&from_redis)?);
        }
        let data = sqlx::query_as::<_, T>(&format!(
            r#"SELECT * FROM goit.{table_name} WHERE {key_name} = ($1)"#
        ))
        .bind(key)
        .fetch_one(&self.inner)
        .await?;
        reids_conn
            .set_ex::<&str, String, Option<String>>(
                &redis_key,
                serde_json::to_string(&data)?,
                3600,
            )
            .await?;
        return Ok(data);
    }
}
