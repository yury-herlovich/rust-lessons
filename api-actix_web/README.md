# Simple api server using actix_web framework

#### Start web server
- `cargo run` - starts app
- `docker run --rm -p 27000:27017 -e MONGO_INITDB_DATABASE=actixdb -d mongo:latest` - starts mongoDb (`... -e MONGO_INITDB_ROOT_USERNAME=root -e MONGO_INITDB_ROOT_PASSWORD=password...` to add authentication)

#### Endpoints:
- `GET /` - hello world (`curl localhost:8080`)
- `GET /users` - get list of all users (`curl localhost:8080/users`)
- `POST /users` - add new user (`curl -X POST localhost:8080/users/6184259364fd1b6a3ce45270 -d "{\"name\":\"new name\", \"role\":\"user\"}" -H "Content-Type: application/json"`)
- `GET /users/{id}` - get user by id (`curl localhost:8080/users/6184259364fd1b6a3ce45270`)
- `PATCH /users/{id}` - update user, not really a PATCH request, it replaces full db record for now (`curl -X PATCH localhost:8080/users/6184259364fd1b6a3ce45270 -d "{\"name\":\"new name\", \"role\":\"user\"}" -H "Content-Type: application/json"`)
- `PUT /users/{id}` - replace user (`curl -X PUT localhost:8080/users/6184259364fd1b6a3ce45270 -d "{\"name\":\"new name\", \"role\":\"user\"}" -H "Content-Type: application/json"`)
- `DELETE /users/{id}` - delete user (`curl -X DELETE localhost:8080/users/6184259364fd1b6a3ce45270`)

#### Links
- [actix_web](https://actix.rs/docs)
- [rustlang-rocket-mongodb (rocket+mongodb)](https://github.com/louis030195/rustlang-rocket-mongodb/)
- [actix+mongo](https://github.com/actix/examples/tree/master/database_interactions/mongodb)
