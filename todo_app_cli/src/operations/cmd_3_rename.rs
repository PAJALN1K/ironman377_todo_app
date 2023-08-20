use std::io::stdin;

use self::models::TaskDB;
use diesel::prelude::*;
use todo_app::*;
use std::cell::RefCell;

pub fn rename_task() {
    use self::schema::tasks::dsl::{tasks, name};

    let mut task_id = String::new();

    println!("\nInput the ID of the needed task:");
    stdin().read_line(&mut task_id).expect("Input error");
    let task_id = task_id.trim().parse::<i32>().expect("Invalid ID");

    let conn = &mut establish_connection();

    let old_name = RefCell::new(String::new());
    
    let old_tasks = tasks
        .find(task_id)
        .select(TaskDB::as_select())
        .first(conn)
        .optional();
    match old_tasks {
        Ok(Some(task)) => {
            *old_name.borrow_mut() = task.name;
            println!("You have chosen task {}: \"{}\"", task.id, *old_name.borrow());
        },
        Ok(None) => println!("Unable to find task {}", task_id),
        Err(_) => println!("An error occured while fetching task {}", task_id),
    }

    let mut desired_name = String::new();

    println!("\nInput a new name of the needed task:");
    stdin().read_line(&mut desired_name).expect("Input error");
    let desired_name = desired_name.trim().to_string();

    let task = diesel::update(tasks.find(task_id))
        .set(name.eq(desired_name))
        .returning(TaskDB::as_returning())
        .get_result(conn)
        .unwrap();
    println!("The task with the name \"{}\" was renamed to \"{}\"", *old_name.borrow(), task.name);
}
