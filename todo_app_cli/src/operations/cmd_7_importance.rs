use std::io::stdin;

use self::models::TaskDB;
use diesel::prelude::*;
use todo_app::*;
use std::cell::RefCell;

pub fn mark_an_importance() {
    use self::schema::tasks::dsl::{tasks, importance};

    let mut task_id = String::new();

    println!("\nInput the ID of the needed task:");
    stdin().read_line(&mut task_id).expect("Input error");
    let task_id = task_id.trim().parse::<i32>().expect("Invalid ID");

    let conn = &mut establish_connection();

    let old_importance = RefCell::new(String::new());
    
    let old_tasks = tasks
        .find(task_id)
        .select(TaskDB::as_select())
        .first(conn)
        .optional();
    match old_tasks {
        Ok(Some(task)) => {
            *old_importance.borrow_mut() = task.importance;
            println!("You have chosen the task \"{}\" with {} importance", task.name, *old_importance.borrow());
        },
        Ok(None) => println!("Unable to find task {}", task_id),
        Err(_) => println!("An error occured while fetching task {}", task_id),
    }

    let mut desired_importance = String::new();

    println!("\nInput a new importance of the needed task:");
    stdin().read_line(&mut desired_importance).expect("Input error");
    let desired_importance = desired_importance.trim().to_string();

    let task = diesel::update(tasks.find(task_id))
        .set(importance.eq(desired_importance))
        .returning(TaskDB::as_returning())
        .get_result(conn)
        .unwrap();
    println!("The importance of task with name \"{}\" is now considered as {}!", task.name, task.importance);
}
