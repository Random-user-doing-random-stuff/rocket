use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel::{prelude::*, r2d2};
use dotenv::dotenv;
use rocket::fairing::AdHoc;
use rocket::tokio::sync::OnceCell;
use std::env;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
static DB_POOL: OnceCell<DbPool> = OnceCell::const_new();

pub fn get_pool() -> &'static DbPool {
    DB_POOL.get().expect("DB pool not initialized")
}
pub fn init_pool(database_url: &str) -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .max_size(10)
        .build(manager)
        .expect("Failed to create pool.")
}

pub fn stage() -> AdHoc {
    dotenv().ok();
    AdHoc::on_ignite("Database Pool", |rocket| async {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = init_pool(&database_url);
        DB_POOL.set(pool).expect("DB pool already initialized");
        rocket
    })
}

// Returns a connection to the database
pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
