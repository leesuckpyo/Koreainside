mod analytics;
mod credentials;
mod export;
mod repository;
mod site_status;

use repository::RepositorySessionState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let result = tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(RepositorySessionState::default())
        .invoke_handler(tauri::generate_handler![
            repository::select_repository,
            repository::disconnect_repository,
            export::preview_repository_export,
            export::export_repository_inventory,
            credentials::save_vercel_access_token,
            credentials::get_vercel_connection_status,
            credentials::delete_vercel_access_token,
            analytics::test_vercel_analytics_connection,
            analytics::get_vercel_analytics_summary,
            site_status::get_site_status_report,
        ])
        .run(tauri::generate_context!());

    if let Err(error) = result {
        eprintln!("Korea Inside Admin 실행 오류: {error}");
    }
}
