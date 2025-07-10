use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
use dotenvy::dotenv;
use std::env;
use crate::structs::Topic;
use crate::structs::Top;

/// Get the correct database URL based on the ENV mode.
pub fn get_database_url() -> String {
    let environment = env::var("ENV").unwrap_or_else(|_| "local".to_string());

    match environment.as_str() {
        "local" => env::var("DATABASE_URL").expect("DATABASE_URL not set"),
        "docker" => env::var("DATABASE_URL_DOCKER").expect("DATABASE_URL_DOCKER not set"),
        "deploy" => env::var("DATABASE_URL_DOCKER_DEPLOYMENT").expect("DATABASE_URL_DOCKER_DEPLOYMENT not set"),
        other => panic!("❌ Unknown ENV value: {}", other),
    }
}

pub async fn get_data_pool() -> MySqlPool {
    dotenv().ok(); // Load .env if not already loaded

    let db_url = get_database_url(); // 👈 Get based on ENV

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

pub async fn get_all_topics(pool: &MySqlPool) -> Result<Vec<Top>, sqlx::Error> {
    let topics = sqlx::query_as!(
        Top,
        "SELECT * FROM topics"
    )
    .fetch_all(pool)
    .await?;

    Ok(topics)
}

