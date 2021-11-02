// use r2d2::PooledConnection;
// use r2d2_mongodb::{ConnectionOptions, MongodbConnectionManager};

// type Pool = r2d2::Pool<MongodbConnectionManager>;

// pub struct Conn(pub PooledConnection<MongodbConnectionManager>);

// /*
//     create a connection pool of mongodb connections to allow a lot of users to modify db at same time.
// */
// pub fn init_pool() -> Pool {
//     let manager = MongodbConnectionManager::new(
//         ConnectionOptions::builder()
//             .with_host("localhost", 27017)
//             .with_db("actixdb")
//             .with_auth("root", "password")
//             .build(),
//     );

//     match Pool::builder().max_size(64).build(manager) {
//         Ok(pool) => pool,
//         Err(e) => panic!("Error: failed to create mongodb pool {}", e),
//     }
// }