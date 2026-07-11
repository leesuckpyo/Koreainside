mod repository;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let result = tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![repository::select_repository])
        .run(tauri::generate_context!());

    if let Err(error) = result {
        eprintln!("Korea Inside Admin 실행 오류: {error}");
    }
}
