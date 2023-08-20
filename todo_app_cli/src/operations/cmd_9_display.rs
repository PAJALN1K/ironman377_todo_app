use crate::models::*;
use diesel::prelude::*;
use todo_app::*;

pub fn display_tasks() {
    use crate::schema::tasks::dsl::*;

    let conn = &mut establish_connection();
    
    let displayed_tasks = tasks
        .filter(id.is_not_null())
        .select(TaskDB::as_select())
        .load(conn)
        .expect("Error loading tasks");

    println!(
        "\n\n| {0: ^5} | {1: ^50} | {2: ^15} | {3: ^15} | {4: ^15} |", 
        "ID", 
        "Name of a task", 
        "Lifesphere",
        "Importance",
        "State"
    );
    println!(
        "{0:-<8}{0:-<53}{0:-<18}{0:-<18}{0:-<18}{0}", 
        "+"
    );
    for task in displayed_tasks {
        println!(
            "| {0: >5} | {1: <50} | {2: <15} | {3: <15} | {4: <15} |",
            task.id, 
            task.name, 
            task.lifesphere, 
            task.importance,
            task.state
        );
    }
}
