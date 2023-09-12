use self::models::TaskDB;
use diesel::prelude::*;
use app::*;

pub fn mark_a_lifesphere(task_id: i32, task_lifesphere: String) {
    use self::schema::tasks::dsl::{tasks, lifesphere};

    let conn = &mut establish_connection();

    diesel::update(tasks.find(task_id))
        .set(lifesphere.eq(task_lifesphere))
        .returning(TaskDB::as_returning())
        .get_result(conn)
        .unwrap(); // 16 (db)
}
