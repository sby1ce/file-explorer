use std::{
    fs::{self, File, OpenOptions},
    os::windows::io::AsRawHandle,
    path::Path,
};

use fe_types::FileData;
use tauri::Runtime;
use tauri_plugin_dialog::DialogExt;
use windows::Win32::{
    Foundation::{FILETIME, HANDLE},
    Storage::FileSystem::GetFileTime,
};

// Seconds between 1601-01-01 and 1970-01-01
const EPOCH_DIFFERENCE: i64 = 11_644_473_600;
const NANOSECONDS_PER_SECOND: i64 = 10_000_000;

fn get_creation_time(path: &Path) -> i64 {
    let file: File = OpenOptions::new().open(&path).ok().unwrap();
    let handle: HANDLE = HANDLE(file.as_raw_handle());
    let mut filetime: FILETIME = FILETIME {
        dwLowDateTime: 0,
        dwHighDateTime: 0,
    };
    // SAFETY:
    // it's a windows API, god knows what it's doing
    unsafe { GetFileTime(handle, Some(&mut filetime), None, None) }.unwrap();
    let filetime_value: i64 = {
        let high: i64 = i64::from(filetime.dwHighDateTime) << 32;
        let low: i64 = i64::from(filetime.dwLowDateTime);
        high | low
    };

    let timestamp: i64 = filetime_value / NANOSECONDS_PER_SECOND - EPOCH_DIFFERENCE;
    timestamp
}

#[tauri::command]
fn pick_directory<R: Runtime>(app: tauri::AppHandle<R>) -> Option<Vec<FileData>> {
    app.dialog().file().blocking_pick_folder().map(|file_path| {
        fs::read_dir(file_path.into_path().unwrap())
            .unwrap()
            .enumerate()
            .filter_map(|(idx, entry)| {
                let path = entry.unwrap().path();
                path.is_file()
                    .then_some(
                        FileData {
                            id: idx,
                            path: path.into_os_string().into_string().unwrap()
                            creation_time: get_creation_time(&path),
                        }
                    )
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
