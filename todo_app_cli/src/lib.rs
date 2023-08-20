use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub mod models;

pub mod schema;
use models::*;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set"); 
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))  
}

pub fn create_task(connection: &mut PgConnection, task_name: &str) -> TaskDB {
    let new_task = NewTask {
        name: task_name,
        state: "Unfinished",
        importance: "Unmarked",
        lifesphere: "Unmarked",
    };

    diesel::insert_into(tasks::table)
        .values(&new_task)
        .returning(TaskDB::as_returning())
        .get_result(connection)
        .expect("Error saving a new task")      
}
