fn main() -> Result<(), Box<dyn std::error::Error>> {
    tauri_build::try_build(tauri_build::Attributes::new().app_manifest(
        tauri_build::AppManifest::new().commands(&[
            "select_repository",
            "disconnect_repository",
            "preview_repository_export",
            "export_repository_inventory",
            "save_vercel_access_token",
            "get_vercel_connection_status",
            "delete_vercel_access_token",
            "test_vercel_analytics_connection",
            "get_vercel_analytics_summary",
            "get_site_status_report",
            "get_search_console_client_status",
            "save_search_console_client_id",
            "delete_search_console_client_id",
            "start_search_console_oauth",
            "disconnect_search_console",
            "test_search_console_connection",
        ]),
    ))?;

    Ok(())
}
