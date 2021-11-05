use actix_web::{web, HttpResponse, Responder};
use mongodb::{bson::doc, bson::oid::ObjectId, Database};
use futures::StreamExt;
use super::*;

pub async fn all(database: web::Data<Database>) -> impl Responder {
    println!("GET /users");
    let collection = database.collection::<User>("users");

    let _result = match collection.find(doc! {}, None).await {
        Ok(mut cursor) => {
            let mut result: Vec<User> = Vec::new();
            while let Some(_doc) = cursor.next().await {
                result.push(User {
                    id: Some(ObjectId::new()),
                    name: String::from("user"),
                    role: Some(String::from("admin")),
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

pub async fn add(database: web::Data<Database>, data: web::Json<User>) -> impl Responder {
    println!("POST /users, body: {:?}", data);

    match model::insert(data.into_inner(), &database).await {
        Ok(id) => {
            match model::get(id, &database).await {
                Ok(doc) => HttpResponse::Ok().json(doc),
                Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
            }

        },
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn get(database: web::Data<Database>, id: web::Path<(String,)>) -> impl Responder {
    println!("GET /users/{}", id.as_ref().0);
    let object_id = ObjectId::parse_str(id.into_inner().0).unwrap();

    match model::get(object_id, &database).await {
        Ok(Some(doc)) => HttpResponse::Ok().json(doc),
        Ok(None) => HttpResponse::NotFound().json(format!("No user found with id {}", object_id)),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
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