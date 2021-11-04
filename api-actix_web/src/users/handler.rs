use actix_web::{web, HttpResponse, Responder};
use mongodb::{bson::doc, Database};
use futures::StreamExt;
use super::*;

pub async fn all(database: web::Data<Database>) -> impl Responder {
    println!("GET /users");
    let collection = database.collection::<User>("users");

    let _result = match collection.find(doc! {}, None).await {
        Ok(mut cursor) => {
            println!("successful mongo response");
            let mut result: Vec<User> = Vec::new();
            while let Some(_doc) = cursor.next().await {
                result.push(User {
                    id: String::from("1abc"),
                    name: String::from("user"),
                    role: String::from("admin"),
                })
            }
            result
        },
        Err(err) => {
            println!("unsuccessful mongo response, {}", err);
            vec![]
        },
    };

    HttpResponse::Ok().body("Get list of users")
}

pub async fn add(_database: web::Data<Database>) -> impl Responder {
    HttpResponse::Ok().body("Adds new user")
}

pub async fn get(_database: web::Data<Database>, id: web::Path<(u32,)>) -> impl Responder {
    println!("GET /users/:id");
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