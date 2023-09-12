use self::models::TaskDB;
use diesel::prelude::*;
use app::*;

pub fn mark_an_importance(task_id: i32, task_importance: String) {
    use self::schema::tasks::dsl::{tasks, importance};

    let conn = &mut establish_connection();

    diesel::update(tasks.find(task_id))
        .set(importance.eq(task_importance))
        .returning(TaskDB::as_returning())
        .get_result(conn)
        .unwrap(); // 15 (db)
}
