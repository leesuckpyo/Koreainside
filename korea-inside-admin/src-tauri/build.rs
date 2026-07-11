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
        ]),
    ))?;

    Ok(())
}
