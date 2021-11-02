use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::{NormalizePath, normalize};
pub mod users;

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(NormalizePath::new(normalize::TrailingSlash::Trim))
            .route("/", web::get().to(hello))
            .service(
                web::scope("/users")
                    .route("/", web::get().to(users::handler::all))
                    .route("/", web::post().to(users::handler::add))
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
