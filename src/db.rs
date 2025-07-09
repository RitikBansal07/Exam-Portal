use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
use dotenvy::dotenv;
use std::env;
use crate::structs::Topic;


pub async fn get_data_pool() -> MySqlPool {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");

    MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("❌ Failed to connect to DB")
}


pub async fn get_topic_by_id(pool: &MySqlPool, id: i32) -> Result<Option<Topic>, sqlx::Error> {
    let row = sqlx::query!(
        "SELECT id, topic FROM topics WHERE id = ?",
        id
    )
    .fetch_optional(pool) // ✅ Use this instead of `execute`
    .await?;

    if let Some(row) = row {
        println!("✅ Found topic: {}", row.topic.as_deref().unwrap_or("NULL"));
        Ok(Some(Topic {
            id: row.id as i32,
            topic: row.topic.unwrap(),
        }))
    } else {
        println!("❌ Topic not found for ID {}", id);
        Ok(None)
    }
}

pub async fn insert_topic(pool: &MySqlPool, msg: String) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO topics (topic) VALUES (?)",
        msg
    )
    .execute(pool)
    .await?;

    println!("✅ Inserted topic: {}", msg);
    Ok(())
}

