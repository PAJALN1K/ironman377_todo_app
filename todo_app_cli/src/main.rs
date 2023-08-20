pub mod models;
pub mod task_parameters;
pub mod schema;
pub mod operations;

use std::io;

fn main() {
    println!("\n\n----------------------------");
    println!("TODO-CLI");
    println!("----------------------------");

    'main_loop: loop {
        println!("\nПожалуйста, введите действие");
        println!("1. Добавить задачу (на фронтенде реализовано).");
        println!("2. Удалить задачу.");
        println!("3. Переименовать задачу.");
        println!("4. Обозначить задачу, как выполненную ().");
        println!("5. Обозначить задачу, как еще не выполненную.");
        println!("6. Обозначить задачу, как отклоненную.");
        println!("7. Обозначить приоритет задачи.");
        println!("8. Обозначить сферу жизни, связанную с задачей.");
        println!("9. Отобразить все задачи.");
        println!("10. Выйти из cli.");

        let mut command = String::new();

        io::stdin()
            .read_line(&mut command)
            .expect("Error! \nLine has not been read! \nPlease, try again.");

        let command = match command.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match command {
            1 => operations::cmd_1_add::add_task(), 
            2 => operations::cmd_2_delete::delete_task(),
            3 => operations::cmd_3_rename::rename_task(),
            4 => operations::cmd_4_complete::mark_as_completed(),
            5 => operations::cmd_5_unfinish::mark_as_unfinished(), 
            6 => operations::cmd_6_reject::mark_as_rejected(),  
            7 => operations::cmd_7_importance::mark_an_importance(),
            8 => operations::cmd_8_lifesphere::mark_a_lifesphere(),
            9 => operations::cmd_9_display::display_tasks(),
            10 => break 'main_loop,
            _ => continue,
        }
    }
}
