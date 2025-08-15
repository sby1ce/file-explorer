mod details;
mod header;

use sycamore::prelude::*;
use wasm_bindgen::prelude::*;

use crate::app::details::DetailsView;
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
    let directory: Signal<PickedDirectory> = create_signal(PickedDirectory::default());

    view! {
        header::Header(set_files=directory)

        DetailsView(directory=directory)
    }
}
