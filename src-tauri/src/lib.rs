use std::{
    fs::{self, File},
    os::windows::io::AsRawHandle,
    path::Path,
};

use fe_types::{FileData, PickedDirectory, Timestamp};
use tauri::{Runtime, ipc::Response};
use tauri_plugin_dialog::DialogExt;
use windows::Win32::{
    Foundation::{FILETIME, HANDLE},
    Storage::FileSystem::GetFileTime,
};

// filetime ticks between 1601-01-01 and 1970-01-01
const EPOCH_DIFFERENCE: i64 = 116_444_736_000_000_000;
const FILETIME_TICKS_PER_MILLISECOND: i64 = 10_000;

fn get_creation_time(path: &Path) -> Timestamp {
    let file: File = File::open(path).unwrap();
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

    let timestamp: i64 = (filetime_value - EPOCH_DIFFERENCE) / FILETIME_TICKS_PER_MILLISECOND;
    Timestamp::new(timestamp)
}

#[tauri::command]
fn pick_directory<R: Runtime>(app: tauri::AppHandle<R>) -> Response {
    let Some(file_path) = app.dialog().file().blocking_pick_folder() else {
        return Response::new(postcard::to_allocvec::<Option<PickedDirectory>>(&None).unwrap());
    };
    let directory = file_path.into_path().unwrap();
    let files: Vec<FileData> = fs::read_dir(&directory)
        .unwrap()
        .filter_map(|entry| {
            let path = entry.unwrap().path();
            path.is_file().then_some(path)
        })
        .enumerate()
        .map(|(idx, path)| {
            let creation_time = get_creation_time(&path);
            // have to extract file name on the back end because for some reason it doesn't work on the front end
            FileData {
                id: idx as u32,
                file_name: path.file_name().unwrap().to_str().unwrap().to_owned(),
                creation_time,
                extension: path
                    .extension()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap()
                    .to_owned(),
            }
        })
        .collect();
    let picked: PickedDirectory = PickedDirectory { directory, files };
    Response::new(postcard::to_allocvec(&Some(picked)).unwrap())
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
