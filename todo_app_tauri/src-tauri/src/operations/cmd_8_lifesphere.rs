use std::io::stdin;

use self::models::TaskDB;
use diesel::prelude::*;
use app::*;
use std::cell::RefCell;

pub fn mark_a_lifesphere() {
    use self::schema::tasks::dsl::{tasks, lifesphere};

    let mut task_id = String::new();

    println!("\nInput the ID of the needed task:");
    stdin().read_line(&mut task_id).expect("Input error");
    let task_id = task_id.trim().parse::<i32>().expect("Invalid ID");

    let conn = &mut establish_connection();

    let old_lifesphere = RefCell::new(String::new());
    
    let old_tasks = tasks
        .find(task_id)
        .select(TaskDB::as_select())
        .first(conn)
        .optional();
    match old_tasks {
        Ok(Some(task)) => {
            *old_lifesphere.borrow_mut() = task.lifesphere;
            println!("You have chosen the task \"{}\" that belongs to lifesphere {}", task.name, *old_lifesphere.borrow());
        },
        Ok(None) => println!("Unable to find task {}", task_id),
        Err(_) => println!("An error occured while fetching task {}", task_id),
    }

    let mut desired_lifesphere = String::new();

    println!("\nInput a new lifesphere of the needed task:");
    stdin().read_line(&mut desired_lifesphere).expect("Input error");
    let desired_lifesphere = desired_lifesphere.trim().to_string();

    let task = diesel::update(tasks.find(task_id))
        .set(lifesphere.eq(desired_lifesphere))
        .returning(TaskDB::as_returning())
        .get_result(conn)
        .unwrap();
    println!("The task with the name \"{}\" is now is now related to the {}!", task.name, task.lifesphere);
}
