# Simple api server using actix_web framework

#### Start web server
- `cargo run` - starts app
- `docker run --rm -p 27000:27017 -e MONGO_INITDB_ROOT_USERNAME=root -e MONGO_INITDB_ROOT_PASSWORD=password -e MONGO_INITDB_DATABASE=actixdb -d mongo:latest` - starts mongoDb

#### Endpoints:
- `GET /` - hello world (`curl localhost:8080`)
- `GET /users` - get list of all users (`curl localhost:8080/users`)
- `POST /users` - add new user (`curl -X POST localhost:8080/users`)
- `GET /users/{id}` - get user by id (`curl localhost:8080/users/1`)
- `PATCH /users/{id}` - update user (`curl -X PATCH localhost:8080/users/1`)
- `PUT /users/{id}` - replace user (`curl -X PUT localhost:8080/users/1`)
- `DELETE /users/{id}` - delete user (`curl -X DELETE localhost:8080/users/1`)

#### Links
- [actix_web](https://actix.rs/docs)
- [rustlang-rocket-mongodb (rocket+mongodb)](https://github.com/louis030195/rustlang-rocket-mongodb/)
- [actix+mongo](https://github.com/actix/examples/tree/master/database_interactions/mongodb)