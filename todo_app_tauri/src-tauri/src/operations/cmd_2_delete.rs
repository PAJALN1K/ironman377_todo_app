use diesel::prelude::*;
use app::*;

pub fn delete_task(task_id: i32) {
    use self::schema::tasks::dsl::*;

    let conn = &mut establish_connection();
    
    diesel::delete(tasks.find(task_id))
        .execute(conn)
        .expect("Error deleting tasks"); // 10 (db)
}

