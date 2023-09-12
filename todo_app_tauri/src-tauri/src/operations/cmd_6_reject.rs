use self::models::TaskDB;
use diesel::prelude::*;
use app::*;

pub fn mark_as_rejected(task_id: i32) {
    use self::schema::tasks::dsl::{tasks, state};
    
    let conn = &mut establish_connection();

    diesel::update(tasks.find(task_id))
        .set(state.eq("Rejected"))
        .returning(TaskDB::as_returning())
        .get_result(conn)
        .unwrap(); // 14 (db)
}
