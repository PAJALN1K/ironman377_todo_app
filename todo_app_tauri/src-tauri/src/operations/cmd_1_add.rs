use app::*;

pub fn add_task(task_name: &str) {
    let conn = &mut establish_connection();

    create_task(conn, task_name);
}
