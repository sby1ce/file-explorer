mod header;

use serde::{Deserialize, Serialize};
use sycamore::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    pub async fn invoke(cmd: &str, args: JsValue) -> JsValue;
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = convertFileSrc)]
    pub fn convert_file_src(file_path: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
pub struct EmptyArgs;

#[component]
pub fn App() -> View {
    let files = create_signal(Vec::new());
    let styles = css_mod::get!("app.css");
    view! {
        header::Header(set_files=files) {}
        main(class=styles["main"]) {
            ul(class=styles["ul"]) {
                Keyed(
                    list=files,
                    view=|file| view! {
                        li { (file.path) }
                    },
                    key=|file| file.id,
                )
            }
        }
    }
}
