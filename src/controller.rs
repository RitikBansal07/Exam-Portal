use actix_web::{get, post, web, HttpResponse,Responder};
use crate::structs::{NewTopic};
use crate::db::{get_data_pool, get_topic_by_id, insert_topic, get_all_topics};

#[get("/{id}")]
pub async fn get_topic_by_id_controller(
    path: web::Path<i32>,
) -> impl Responder {
    let id = path.into_inner();

    // get pool from helper
    let pool = get_data_pool().await;

    match get_topic_by_id(&pool, id).await {
        Ok(Some(topic)) => HttpResponse::Ok().json(topic),
        Ok(None) => HttpResponse::NotFound().body("Topic not found"),
        Err(err) => {
            eprintln!("❌ DB error: {}", err);
            HttpResponse::InternalServerError().body("DB error")
        }
    }

}#[post("/topic")]
pub async fn insert_topic_controller(msg: web::Json<NewTopic>) -> impl Responder {
     let pool = get_data_pool().await;

     match insert_topic(&pool, msg.topic.clone()).await {
         Ok(_) => HttpResponse::Created().body("✅ Topic inserted"),
         Err(err) => {
             eprintln!("❌ Insert error: {}", err);
             HttpResponse::InternalServerError().body("Failed to insert topic")
         }
     }
 }

 #[get("/topics")]
 pub async fn get_all_topics_controller() -> impl Responder {
     // Get the database connection pool
     let pool = get_data_pool().await;

     // Call a function to get all topics from DB
     match get_all_topics(&pool).await {
         Ok(topics) => HttpResponse::Ok().json(topics),  // send all topics as JSON
         Err(err) => {
             eprintln!("❌ DB error: {}", err);
             HttpResponse::InternalServerError().body("DB error")
         }
     }
 }