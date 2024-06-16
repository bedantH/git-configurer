// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;
use auto_launch::*;
use tauri::{AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, Window, Wry};
use std::env::current_exe;

#[tauri::command]
fn run_config_command(email: String, username: String) {
  let email_output: std::process::Output = Command::new("git")
    .args(["config", "--global", "user.email", &format!("{}", email)])
    .output()
    .expect("failed to execute process");

  let username_output: std::process::Output = Command::new("git")
    .args(["config", "--global", "user.name", &format!("{}", username)])
    .output()
    .expect("failed to execute process");

  println!("{:?}", email_output);
  println!("{:?}", username_output);
}

fn show_window(app: &AppHandle<Wry>) {
  match app.get_window("main") {
      Some(window) => {
          if !window.is_visible().unwrap_or(false) {
              window.show().unwrap();
          }
          window
              .set_focus().unwrap();
      }
      _ => {
          //Error
      }
  };
}


fn create_tray() -> SystemTray {
  // Define menu items for the system tray
  let show = CustomMenuItem::new("show".to_string(), "Show Window");

  // Create the tray menu with the defined menu items
  let tray_menu = SystemTrayMenu::new().add_item(show);

  SystemTray::new().with_menu(tray_menu)
}
pub fn tray_event_handler(app: &AppHandle, event: SystemTrayEvent) {
  match event {
      SystemTrayEvent::DoubleClick { .. } => show_window(app),
      SystemTrayEvent::MenuItemClick { id, .. } => {
          match id.as_str() {
              "show" => show_window(app),
              _ => {}
          }
      }
      _ => {}
  }
}

fn main() {
  tauri::Builder::default()
    .on_window_event(|event| match event.event() {
      tauri::WindowEvent::CloseRequested { api, .. } => {
        event.window().hide().unwrap();
        api.prevent_close();
      }
      _ => {}
    })
    .system_tray(create_tray())
    .on_system_tray_event(tray_event_handler)
    .invoke_handler(tauri::generate_handler![run_config_command])
    .setup(|app| {
      let app_name = &app.package_info().name;
      let current_exe = current_exe().unwrap();

      let auto_start = AutoLaunchBuilder::new()
                                      .set_app_name(&app_name)
                                      .set_app_path(&current_exe.to_str().unwrap())
                                      .set_use_launch_agent(true)
                                      .build()
                                      .unwrap();
      
      auto_start.enable().unwrap();
      println!("is autostart {}", auto_start.is_enabled().unwrap());

      Ok(())
  })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
