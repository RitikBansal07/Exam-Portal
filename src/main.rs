mod structs;
mod db;
mod controller;

use actix_web::{App, HttpServer};
use controller::{get_all_topics_controller,get_topic_by_id_controller, insert_topic_controller};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Server running at http://localhost:8080");
    HttpServer::new(|| {
        App::new()
            .service(get_all_topics_controller)
            .service(get_topic_by_id_controller)
            .service(insert_topic_controller)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
