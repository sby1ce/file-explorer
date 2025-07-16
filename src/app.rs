mod details;
mod header;

use fe_types::PickedDirectory;
use sycamore::prelude::*;
use wasm_bindgen::prelude::*;

use crate::app::details::DetailsItem;

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
    view! {
        header::Header(set_files=directory) {}
        main(class=styles["main"]) {
            div(class=styles["div"]) {
                Keyed(
                    list=files,
                    view=DetailsItem,
                    key=|file| file.id,
                )
            }
        }
    }
}
