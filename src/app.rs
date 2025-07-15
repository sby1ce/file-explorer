mod details;
mod header;

use serde::{Deserialize, Serialize};
use sycamore::prelude::*;
use wasm_bindgen::prelude::*;

use crate::app::details::DetailsItem;

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
            table(class=styles["table"]) {
                colgroup {
                    col {}
                    col {}
                }
                thead {
                    tr {
                        th(class=styles["th"]) {}
                        th(class=styles["th"]) { "path" }
                    }
                }
                tbody {
                    Keyed(
                        list=files,
                        view=DetailsItem,
                        key=|file| file.id,
                    )
                }
            }
        }
    }
}
