use std::fs;

use tauri::Runtime;
use tauri_plugin_dialog::DialogExt;

#[tauri::command]
fn pick_directory<R: Runtime>(app: tauri::AppHandle<R>) -> Option<Vec<String>> {
    app.dialog().file().blocking_pick_folder().map(|file_path| {
        fs::read_dir(file_path.into_path().unwrap())
            .unwrap()
            .filter_map(|entry| {
                let path = entry.unwrap().path();
                path.is_file()
                    .then_some(path.into_os_string().into_string().unwrap())
            })
            .collect()
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![pick_directory])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
