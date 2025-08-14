mod details;
mod header;

use sycamore::prelude::*;
use wasm_bindgen::prelude::*;

use crate::app::details::{ColumnProps, DetailsColumn, DetailsItem, SortOptions, TableHead};
use fe_types::{FileData, PickedDirectory};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    pub async fn invoke(cmd: &str) -> JsValue;
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
    pub async fn invoke_with(cmd: &str, args: JsValue) -> JsValue;
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = convertFileSrc)]
    pub fn convert_file_src(file_path: JsValue) -> JsValue;
}

#[component]
pub fn App() -> View {
    let directory = create_signal(PickedDirectory::default());
    let styles = css_mod::get!("app.css");

    let sort_options: Signal<Option<SortOptions>> = create_signal(None);

    let files = move || {
        let mut files = directory.get_clone().files;
        match sort_options.get() {
            None => files,
            Some(SortOptions::FileName(reverse)) => {
                files.sort_unstable_by(|file1: &FileData, file2: &FileData| {
                    file1.file_name.cmp(&file2.file_name)
                });
                if reverse {
                    files.reverse();
                }
                files
            }
            Some(SortOptions::CreatedAt(reverse)) => {
                files.sort_unstable_by_key(|file: &FileData| file.creation_time);
                if reverse {
                    files.reverse();
                }
                files
            }
            Some(SortOptions::Extension(reverse)) => {
                files.sort_unstable_by(|file1: &FileData, file2: &FileData| {
                    file1.extension.cmp(&file2.extension)
                });
                if reverse {
                    files.reverse();
                }
                files
            }
        }
    };

    // creating vec with map because `Signal` is clonable
    // so Rust clones the same signal for all elements
    let props: Signal<Vec<ColumnProps>> = create_signal(vec![
        ColumnProps::new(200, "file name", sort_options, DetailsColumn::FileName),
        ColumnProps::new(200, "created at", sort_options, DetailsColumn::CreatedAt),
        ColumnProps::new(100, "extension", sort_options, DetailsColumn::Extension),
    ]);

    let style = move || {
        let widths = props.get_clone();
        format!(
            "grid-template-columns: {}px {}px {}px auto",
            widths[0].width.get(),
            widths[1].width.get(),
            widths[2].width.get(),
        )
    };

    view! {
        header::Header(set_files=directory) {}

        main(class=styles["main"], style=style) {
            TableHead(props=props) {}

            Keyed(
                list=files,
                view=DetailsItem,
                key=|file| file.id,
            )
        }
    }
}
