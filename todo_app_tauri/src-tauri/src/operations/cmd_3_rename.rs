use self::models::TaskDB;
use diesel::prelude::*;
use app::*;

pub fn rename_task(task_id: i32, task_value: String) {
    use self::schema::tasks::dsl::{tasks, name};

    let conn = &mut establish_connection();

    diesel::update(tasks.find(task_id))
        .set(name.eq(task_value))
        .returning(TaskDB::as_returning())
        .get_result(conn)
        .unwrap(); // 11 (db)
}
