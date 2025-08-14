mod details;
mod header;

use sycamore::prelude::*;
use wasm_bindgen::prelude::*;

use crate::app::details::{DetailsItem, TableHead};
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
    let widths: Signal<Vec<Signal<i32>>> = create_signal(vec![
        create_signal(400),
        create_signal(200),
        create_signal(100),
    ]);

    let style = move || {
        let widths = widths.get_clone();
        format!(
            "grid-template-columns: {}px {}px {}px auto",
            widths[0].get(),
            widths[1].get(),
            widths[2].get(),
        )
    };

    view! {
        header::Header(set_files=directory) {}

        main(class=styles["main"], style=style) {
            TableHead(widths=widths) {}

            Keyed(
                list=files,
                view=DetailsItem,
                key=|file| file.id,
            )
        }
    }
}
