use actix_web::{web, HttpResponse, Responder};
use mongodb::{bson::oid::ObjectId, Database};
use super::*;

pub async fn all(database: web::Data<Database>) -> impl Responder {
    println!("GET /users");

    match model::all(&database).await {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

pub async fn add(data: web::Json<User>, database: web::Data<Database>) -> impl Responder {
    println!("POST /users, json: {:?}", data);

    match model::insert(data.into_inner(), &database).await {
        Ok(id) => {
            match model::get(id, &database).await {
                Ok(doc) => HttpResponse::Ok().json(doc),
                Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
            }

        },
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

pub async fn get(id: web::Path<(String,)>, database: web::Data<Database>) -> impl Responder {
    println!("GET /users/{}", id.as_ref().0);
    let object_id = ObjectId::parse_str(id.into_inner().0).unwrap();

    match model::get(object_id, &database).await {
        Ok(Some(doc)) => HttpResponse::Ok().json(doc),
        Ok(None) => HttpResponse::NotFound().json(format!("No user found with id {}", object_id)),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

pub async fn replace(id: web::Path<(u32,)>) -> impl Responder {
    HttpResponse::Ok().json(format!("Replaces user by ID: {}", id.into_inner().0))
}

pub async fn update(id: web::Path<(u32,)>) -> impl Responder {
    HttpResponse::Ok().json(format!("Updates user by ID: {}", id.into_inner().0))
}

pub async fn delete(id: web::Path<(String,)>, database: web::Data<Database>) -> impl Responder {
    println!("DELETE /users/{}", id.as_ref().0);
    let object_id = ObjectId::parse_str(id.into_inner().0).unwrap();

    match model::delete(object_id, &database).await {
        Ok(doc) => {
            if doc.deleted_count == 1 {
                HttpResponse::Ok().json("User deleted")
            } else {
                HttpResponse::NotFound().json(format!("No user found with id {}", object_id))
            }
        }
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}