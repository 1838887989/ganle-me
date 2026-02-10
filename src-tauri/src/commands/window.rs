use tauri::{AppHandle, Manager};

/// 切换窗口模式（标准/极简）
#[tauri::command]
pub fn set_window_mode(app: AppHandle, mode: String) -> Result<(), String> {
    let window = app
        .get_webview_window("main")
        .ok_or("找不到主窗口")?;

    match mode.as_str() {
        "standard" => {
            window.set_min_size(Some(tauri::LogicalSize::new(600.0, 500.0)))
                .map_err(|e| e.to_string())?;
            window.set_size(tauri::LogicalSize::new(900.0, 700.0))
                .map_err(|e| e.to_string())?;
        }
        "minimal" => {
            window.set_min_size(Some(tauri::LogicalSize::new(320.0, 480.0)))
                .map_err(|e| e.to_string())?;
            window.set_size(tauri::LogicalSize::new(400.0, 600.0))
                .map_err(|e| e.to_string())?;
        }
        _ => return Err(format!("未知模式: {}", mode)),
    }

    Ok(())
}
