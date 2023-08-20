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
    cmd_9_display::display_tasks
};

use tauri::Manager;

#[tauri::command(rename_all = "snake_case")]
fn display_tasks_on_frontend() -> Vec<TaskDB> {
    display_tasks()
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();

            main_window.listen("add-task-to-backend", |event| {
                if let Some(task_name) = event.payload() {
                    add_task(task_name);
                };
            });

            main_window.listen("delete-task-from-backend", |event| {
                if let Some(task_id) = event.payload() {
                    let clear_task_id = task_id.parse::<i32>().unwrap();
                    delete_task(clear_task_id);
                };
            });
            
            main_window.listen("complete-task-in-backend", |event| {
                if let Some(task_id) = event.payload() {
                    let clear_task_id = task_id.parse::<i32>().unwrap();
                    mark_as_completed(clear_task_id);
                };
            });

            main_window.listen("unfinish-task-in-backend", |event| {
                if let Some(task_id) = event.payload() {
                    let clear_task_id = task_id.parse::<i32>().unwrap();
                    mark_as_unfinished(clear_task_id);
                };
            });

            main_window.listen("reject-task-in-backend", |event| {
                if let Some(task_id) = event.payload() {
                    let clear_task_id = task_id.parse::<i32>().unwrap();
                    mark_as_rejected(clear_task_id);
                };
            });

            // main_window.listen("add-task-to-backend", |event| {
            //     if let Some(task_name) = event.payload() {
            //         add_task(task_name);
            //     };
            // });

            // main_window.listen("rename-task-in-backend", move |event| {
            //     if let Some(button_id) = event.payload() {
            //         let clear_button_id = button_id.parse::<i32>().unwrap();
            //         println!("got window event-name with payload {:?}", clear_button_id);
            //         // &app_handle.emit_all("console-show", clear_button_id + 10).unwrap();
            //     };
            // });

            // main_window.listen("mark-importance-on-backend", move |event| {
            //     if let Some(button_id) = event.payload() {
            //         let clear_button_id = button_id.parse::<i32>().unwrap();
            //         println!("got window event-name with payload {:?}", clear_button_id);
            //         // &app_handle.emit_all("console-show", clear_button_id + 10).unwrap();
            //     };
            // });
            
            // main_window.listen("mark-lifesphere-on-backend", move |event| {
            //     if let Some(button_id) = event.payload() {
            //         let clear_button_id = button_id.parse::<i32>().unwrap();
            //         println!("got window event-name with payload {:?}", clear_button_id);
            //         // &app_handle.emit_all("console-show", clear_button_id + 10).unwrap();
            //     };
            // });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            display_tasks_on_frontend
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
