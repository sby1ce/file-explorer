mod details;
mod header;

use sycamore::prelude::*;
use wasm_bindgen::prelude::*;

use crate::app::details::{ColumnProps, DetailsItem, TableHead};
use fe_types::PickedDirectory;

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
    let files = move || directory.get_clone().files;

    // creating vec with map because `Signal` is clonable
    // so Rust clones the same signal for all elements
    let props: Signal<Vec<ColumnProps>> = create_signal(vec![
        ColumnProps::new(200, "file name"),
        ColumnProps::new(200, "created at"),
        ColumnProps::new(100, "extension"),
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
