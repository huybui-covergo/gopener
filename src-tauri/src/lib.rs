mod commands;
mod google;
mod utils;

use tauri::Emitter;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            // Check for file arguments passed via CLI or file association
            let args: Vec<String> = std::env::args().collect();
            if args.len() > 1 {
                let file_path = &args[1];
                // Validate it's a file path (not a flag)
                if !file_path.starts_with('-') && std::path::Path::new(file_path).exists() {
                    let app_handle = app.handle().clone();
                    let path = file_path.clone();
                    // Emit event to frontend after window is ready
                    tauri::async_runtime::spawn(async move {
                        // Small delay to ensure frontend is ready
                        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                        let _ = app_handle.emit("file-opened", path);
                    });
                }
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Auth commands
            commands::auth::get_auth_url,
            commands::auth::exchange_code,
            commands::auth::refresh_token,
            commands::auth::sign_out,
            commands::auth::check_auth,
            // Upload commands
            commands::upload::upload_file,
            commands::upload::get_upload_progress,
            // Storage commands
            commands::storage::get_settings,
            commands::storage::save_settings,
            commands::storage::add_recent_file,
            commands::storage::clear_recent_files,
            // OAuth config commands
            commands::oauth_config::get_oauth_config,
            commands::oauth_config::save_oauth_config,
            commands::oauth_config::clear_oauth_config,
            // File association commands
            commands::fileassoc::register_file_associations,
            // Google Drive commands
            google::drive::list_folders,
            google::drive::create_folder,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
