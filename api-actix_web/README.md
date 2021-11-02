### Simple api server

#### Start web server
- `cargo run`

#### Endpoints:
- `GET /` - hello world (`curl localhost:8080`)
- `GET /users` - get list of all users (`curl localhost:8080/users`)
- `POST /users` - add new user (`curl -X POST localhost:8080/users`)
- `GET /users/{id}` - get user by id (`curl localhost:8080/users/1`)
- `PATCH /users/{id}` - update user (`curl -X PATCH localhost:8080/users/1`)
- `PUT /users/{id}` - replace user (`curl -X PUT localhost:8080/users/1`)
- `DELETE /users/{id}` - delete user (`curl -X DELETE localhost:8080/users/1`)