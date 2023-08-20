use todo_app::*;
use std::io::stdin;

pub fn add_task() {
    let conn = &mut establish_connection();

    let mut task_name = String::new();

    println!("\nInput the needed task:");
    stdin().read_line(&mut task_name).expect("Input error");
    let task_name = task_name.trim();

    let task = create_task(conn, task_name);
    println!("Task \"{}\" with id {} was added to the list", &task_name, task.id);
}
