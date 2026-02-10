mod commands;
mod db;
mod models;
mod services;

use db::connection::Database;
use std::sync::Mutex;
use tauri::{
    menu::{CheckMenuItem, Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager, WindowEvent,
};
use tauri_plugin_autostart::MacosLauncher;

/// 应用全局状态
pub struct AppState {
    pub db: Mutex<Database>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec![]),
        ))
        .setup(|app| {
            // 初始化数据库
            let app_dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&app_dir)?;
            let db_path = app_dir.join("ganle_me.db");
            let database = Database::new(&db_path)
                .map_err(|e| e.to_string())?;
            database.run_migrations()
                .map_err(|e| e.to_string())?;
            app.manage(AppState {
                db: Mutex::new(database),
            });

            // 创建系统托盘
            setup_tray(app)?;

            Ok(())
        })
        .on_window_event(|window, event| {
            // 点击关闭按钮时隐藏到托盘而非退出
            if let WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                let _ = window.hide();
            }
        })
        .invoke_handler(tauri::generate_handler![
            // 任务命令
            commands::task::get_all_tasks,
            commands::task::get_filtered_tasks,
            commands::task::create_task,
            commands::task::update_task,
            commands::task::toggle_task_status,
            commands::task::toggle_favorite,
            commands::task::delete_task,
            commands::task::reorder_tasks,
            commands::task::get_task_counts,
            commands::task::check_reminders,
            // 子任务命令
            commands::task::add_subtask,
            commands::task::toggle_subtask,
            commands::task::update_subtask,
            commands::task::delete_subtask,
            // 项目命令
            commands::project::get_all_projects,
            commands::project::create_project,
            commands::project::update_project,
            commands::project::delete_project,
            commands::project::get_project_task_count,
            // 标签命令
            commands::tag::get_all_tags,
            commands::tag::create_tag,
            commands::tag::update_tag,
            commands::tag::delete_tag,
            commands::tag::get_tag_task_count,
            // 设置命令
            commands::settings::get_settings,
            commands::settings::update_settings,
            // 窗口命令
            commands::window::set_window_mode,
        ])
        .run(tauri::generate_context!())
        .expect("启动应用失败");
}

/// 创建系统托盘
fn setup_tray(app: &tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let show = MenuItem::with_id(app, "show", "显示窗口", true, None::<&str>)?;
    let auto_launch = CheckMenuItem::with_id(
        app,
        "auto_launch",
        "开机自启",
        true,
        false,
        None::<&str>,
    )?;
    let separator = PredefinedMenuItem::separator(app)?;
    let quit = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;

    let menu = Menu::with_items(app, &[&show, &auto_launch, &separator, &quit])?;

    TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .tooltip("干了么")
        .menu(&menu)
        .on_menu_event(move |app, event| match event.id.as_ref() {
            "show" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        })
        .build(app)?;

    Ok(())
}
