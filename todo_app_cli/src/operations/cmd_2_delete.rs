use std::io::stdin;
use diesel::prelude::*;
use todo_app::*;

pub fn delete_task() {
    use self::schema::tasks::dsl::*;

    let mut cli_string = String::new();
    
    println!("\nInsert a value in order to delete all tasks whose name contains it:");
    stdin().read_line(&mut cli_string).expect("Input error");
    let pattern = format!("%{}%", cli_string.trim());

    let conn = &mut establish_connection();
    let num_deleted = diesel::delete(tasks.filter(name.like(pattern)))
        .execute(conn)
        .expect("Error  deleting tasks");

    println!("Deleted {} tasks", num_deleted);
}

