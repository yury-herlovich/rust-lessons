use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::{NormalizePath, TrailingSlash};
mod users;
use mongodb::{Client};

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_client = Client::with_uri_str("mongodb://localhost:27000").await.expect("failed to connect");
    let database = db_client.database("actixdb");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(database.clone()))
            .wrap(NormalizePath::new(TrailingSlash::Trim))
            .route("/", web::get().to(hello))
            .service(
                web::scope("/users")
                    .route("", web::get().to(users::handler::all))
                    .route("", web::post().to(users::handler::add))
                    .route("/{id}", web::get().to(users::handler::get))
                    .route("/{id}", web::patch().to(users::handler::update))
                    .route("/{id}", web::put().to(users::handler::replace))
                    .route("/{id}", web::delete().to(users::handler::delete)),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
