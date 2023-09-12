// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod models;
pub mod schema;
pub mod operations;

use models::TaskDB;
use operations::{
    cmd_1_add::add_task,
    cmd_2_delete::delete_task,
    cmd_3_rename::rename_task,
    cmd_4_complete::mark_as_completed,
    cmd_5_unfinish::mark_as_unfinished,
    cmd_6_reject::mark_as_rejected,
    cmd_7_importance::mark_an_importance,
    cmd_8_lifesphere::mark_a_lifesphere,
    cmd_9_display::display_tasks,
};

use tauri::Manager;

use std::time::Duration;
use std::thread::sleep;


fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // let task_sort = Arc::new(Mutex::new(String::new()));

            let main_window = app.get_window("main").unwrap(); // 1 (tauri)
            let window_clone = main_window.clone();

            main_window.listen("add-task-to-backend", |event| {
                if let Some(task_name) = event.payload() {
                    let task_name = task_name.trim().replace("\"", "");
                    add_task(task_name);
                };
            });

            main_window.listen("delete-task-from-backend", |event| {
                if let Some(task_id) = event.payload() {
                    let clear_task_id = task_id.parse::<i32>().unwrap(); // 2 (parse)
                    delete_task(clear_task_id);
                };
            });
            
            main_window.listen("complete-task-in-backend", |event| {
                if let Some(task_id) = event.payload() {
                    let clear_task_id = task_id.parse::<i32>().unwrap(); // 3 (parse)
                    mark_as_completed(clear_task_id);
                };
            });

            main_window.listen("unfinish-task-in-backend", |event| {
                if let Some(task_id) = event.payload() {
                    let clear_task_id = task_id.parse::<i32>().unwrap(); // 4 (parse)
                    mark_as_unfinished(clear_task_id);
                };
            });

            main_window.listen("reject-task-in-backend", |event| {
                if let Some(task_id) = event.payload() {
                    let clear_task_id = task_id.parse::<i32>().unwrap(); // 5 (parse)
                    mark_as_rejected(clear_task_id);
                };
            });

            main_window.listen("mark-importance-of-task-in-backend", |event| {
                if let Some(task_vec) = event.payload() {
                    let task_vec = &task_vec.trim()[1..task_vec.len()-1].replace("\"", "");
                    let clear_task_vec: Vec<&str> = task_vec.split(',').collect();

                    let task_id = clear_task_vec[0].parse::<i32>().unwrap(); // 6 (parse)
                    let task_importance = clear_task_vec[1].replacen(&clear_task_vec[1][0..=0], &clear_task_vec[1][0..=0].to_uppercase(), 1);

                    mark_an_importance(task_id, task_importance);
                };
            });

            main_window.listen("rename-task-in-backend", |event| {
                if let Some(task_vec) = event.payload() {
                    let task_vec = &task_vec.trim()[1..task_vec.len()-1].replace("\"", "");
                    let clear_task_vec: Vec<&str> = task_vec.split(',').collect();

                    let task_id = clear_task_vec[0].parse::<i32>().unwrap(); // 7 (parse)
                    let task_value = clear_task_vec[1].to_string();

                    rename_task(task_id, task_value);
                };
            });

            main_window.listen("mark-lifesphere-in-backend", |event| {
                if let Some(task_vec) = event.payload() {
                    let task_vec = &task_vec.trim()[1..task_vec.len()-1].replace("\"", "");
                    let clear_task_vec: Vec<&str> = task_vec.split(',').collect();

                    let task_id = clear_task_vec[0].parse::<i32>().unwrap(); // 8 (parse)
                    let task_lifesphere = clear_task_vec[1].to_string();

                    mark_a_lifesphere(task_id, task_lifesphere);
                };
            });

            main_window.listen("sort-in-backend", move |event| {
                if let Some(sort) = event.payload() {
                    sleep(Duration::from_millis(500));
                    let sort = sort.trim().replace("\"", "");
                    window_clone.emit_all("display_tasks_on_frontend", &display_tasks_on_frontend(Some(&sort))).unwrap();
                } else {
                    window_clone.emit_all("display_tasks_on_frontend", &display_tasks_on_frontend(event.payload())).unwrap();
                };
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            display_tasks_on_frontend
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application"); // 9 (tauri)
}

#[tauri::command(rename_all = "snake_case")]
fn display_tasks_on_frontend(sort: Option<&str>) -> Vec<TaskDB> {
    display_tasks(sort)
}
