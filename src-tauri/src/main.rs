// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::path::Path;
use tauri::{
    image::Image,
    tray::{TrayIconBuilder, TrayIconEvent},
    Manager,
};
use tauri_plugin_positioner::{Position, WindowExt};
mod api;
use dotenv::dotenv;

fn main() {
	dotenv().ok();
    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .setup(|app| {
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            TrayIconBuilder::new()
                .icon(Image::from_path(Path::new("./icons/icon.png")).expect("REASON"))
                .on_tray_icon_event(|app, event| {
                    tauri_plugin_positioner::on_tray_event(app.app_handle(), &event);
                    match event {
                        TrayIconEvent::Click {
                            button,
                            button_state,
							..
                        } => match button {
                            tauri::tray::MouseButton::Left => match button_state {
                                tauri::tray::MouseButtonState::Up => {
                                    let window =
                                        app.app_handle().get_webview_window("main").unwrap();
                                    let position = Position::TrayCenter;

                                    if window.is_visible().unwrap_or(false) {
                                        window.hide().unwrap();
                                    } else {
                                        window.move_window(position).unwrap();
                                        window.show().unwrap();
                                        window.set_focus().unwrap();
                                    }
                                }
                                tauri::tray::MouseButtonState::Down => {}
                            },
                            tauri::tray::MouseButton::Right => todo!(),
                            tauri::tray::MouseButton::Middle => todo!(),
                        },
                        _ => {}
                    }
                })
                .build(app)?;
            Ok(())
        })
		.on_window_event(|window, event| match event {
			tauri::WindowEvent::Focused(focused) => {
			  if !focused {
				window.hide().unwrap();
			  }
			}
			_ => {}
		})
		.invoke_handler(tauri::generate_handler![api::authorize, api::get_projects, api::get_tasks, api::start_timer, api::stop_timer, api::get_timer])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
