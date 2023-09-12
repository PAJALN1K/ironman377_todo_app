use crate::models::TaskDB;
use diesel::prelude::*;
use app::*;



pub fn display_tasks(sort: Option<&str>) -> Vec<TaskDB> {
    use crate::schema::tasks::dsl::{tasks, id, name, importance, lifesphere, state};

    let conn = &mut establish_connection();
    
    if let Some(inc_column) = sort {
        match inc_column {
            "name" => {
                let displayed_tasks = tasks
                    .filter(id.is_not_null())
                    .select(TaskDB::as_select())
                    .order_by(name)
                    .load(conn)
                    .expect("Error loading tasks"); // 15 (db)
            
                return displayed_tasks;
            },
            "importance" => {
                let displayed_tasks = tasks
                    .filter(id.is_not_null())
                    .select(TaskDB::as_select())
                    .order_by(importance)
                    .load(conn)
                    .expect("Error loading tasks"); // 15 (db)
            
                return displayed_tasks;
            },
            "lifesphere" => {
                let displayed_tasks = tasks
                    .filter(id.is_not_null())
                    .select(TaskDB::as_select())
                    .order_by(lifesphere)
                    .load(conn)
                    .expect("Error loading tasks"); // 15 (db)
            
                return displayed_tasks;
            },
            "state" => {
                let displayed_tasks = tasks
                    .filter(id.is_not_null())
                    .select(TaskDB::as_select())
                    .order_by(state)
                    .load(conn)
                    .expect("Error loading tasks"); // 15 (db)
            
                return displayed_tasks;
            },
            _ => {
                let displayed_tasks = tasks
                    .filter(id.is_not_null())
                    .select(TaskDB::as_select())
                    .load(conn)
                    .expect("Error loading tasks"); // 15 (db)
            
                return displayed_tasks;
            },
        }
    } else {
        let displayed_tasks = tasks
            .filter(id.is_not_null())
            .select(TaskDB::as_select())
            .load(conn)
            .expect("Error loading tasks"); // 15 (db)
    
        return displayed_tasks;
    }

    // let displayed_tasks = tasks
    //     .filter(id.is_not_null())
    //     .select(TaskDB::as_select())
    //     .order_by(importance)
    //     .load(conn)
    //     .expect("Error loading tasks"); // 15 (db)

    // return displayed_tasks;
}
