use actix_web::{web, HttpResponse, Responder};

pub async fn all() -> impl Responder {
    HttpResponse::Ok().body("Returns all users")
}

pub async fn add() -> impl Responder {
    HttpResponse::Ok().body("Adds new user")
}

pub async fn get(id: web::Path<(u32,)>) -> impl Responder {
    HttpResponse::Ok().body(format!("Returns user by ID: {}", id.into_inner().0))
}

pub async fn replace(id: web::Path<(u32,)>) -> impl Responder {
    HttpResponse::Ok().body(format!("Replaces user by ID: {}", id.into_inner().0))
}

pub async fn update(id: web::Path<(u32,)>) -> impl Responder {
    HttpResponse::Ok().body(format!("Updates user by ID: {}", id.into_inner().0))
}

pub async fn delete(id: web::Path<(u32,)>) -> impl Responder {
    HttpResponse::Ok().body(format!("Deletes user by ID: {}", id.into_inner().0))
}