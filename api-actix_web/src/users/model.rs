use crate::users::{User, InsertableUser};
use mongodb::{bson::doc, bson::oid::ObjectId, Database, error::Error};

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
