use self::models::TaskDB;
use diesel::prelude::*;
use app::*;

pub fn mark_as_unfinished(task_id: i32) {
    use self::schema::tasks::dsl::{tasks, state};

    let conn = &mut establish_connection();

    diesel::update(tasks.find(task_id))
        .set(state.eq("Unfinished"))
        .returning(TaskDB::as_returning())
        .get_result(conn)
        .unwrap();
}
