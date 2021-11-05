use mongodb::options::FindOneAndUpdateOptions;
use crate::users::{User, InsertableUser};
use mongodb::{bson, bson::doc, bson::oid::ObjectId, Database, error::Error, results};
use futures::StreamExt;

const COLLECTION: &str = "users";

pub async fn insert(user: User, database: &Database) -> Result<ObjectId, Error> {
    let collection = database.collection::<InsertableUser>(COLLECTION);
    let data = InsertableUser::from_user(user);

    match collection.insert_one(data, None).await {
        Ok(doc) => Ok(doc.inserted_id.as_object_id().unwrap()),
        Err(err) => Err(err),
    }
}

pub async fn get(id: ObjectId, database: &Database) -> Result<Option<User>, Error> {
    let collection = database.collection::<User>(COLLECTION);

    collection.find_one(doc! { "_id": id }, None).await
}

pub async fn all(database: &Database) -> Result<Vec<User>, Error> {
    let collection = database.collection::<User>(COLLECTION);

    match collection.find(doc! {}, None).await {
        Ok(mut cursor) => {
            let mut result: Vec<User> = Vec::new();
            while let Some(doc) = cursor.next().await {
                result.push(doc.unwrap())
            }

            Ok(result)
        },
        Err(err) => Err(err),
    }
}

pub async fn delete(id: ObjectId, database: &Database) -> Result<results::DeleteResult, Error> {
    let collection = database.collection::<User>(COLLECTION);

    collection.delete_one(doc! {"_id": id}, None).await
}

pub async fn update(id: ObjectId, user: User, database: &Database) -> Result<Option<User>, Error> {
    let collection = database.collection::<User>(COLLECTION);
    let data = InsertableUser::from_user(user);

    let filter = doc!{ "_id": id };
    let update = doc!{ "$set": bson::to_bson(&data).unwrap() };
    let opt = FindOneAndUpdateOptions::builder()
        .upsert(Some(false))
        .return_document(Some(mongodb::options::ReturnDocument::After))
        .build();

    collection.find_one_and_update(filter, update, opt).await
}