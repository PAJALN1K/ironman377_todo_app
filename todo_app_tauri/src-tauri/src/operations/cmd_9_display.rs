use crate::models::TaskDB;
use diesel::prelude::*;
use app::*;
// use std::collections::HashMap;

pub fn display_tasks() -> Vec<TaskDB> {
    use crate::schema::tasks::dsl::*;

    let conn = &mut establish_connection();
    
    let displayed_tasks = tasks
        .filter(id.is_not_null())
        .select(TaskDB::as_select())
        .load(conn)
        .expect("Error loading tasks");
    
    return displayed_tasks;
}
