mod details;
mod header;

use fe_types::PickedDirectory;
use sycamore::prelude::*;
use wasm_bindgen::prelude::*;

use crate::app::details::{DetailsItem, MultiRange};

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

    let widths: [Signal<String>; 2] = [create_signal("250px".to_owned()); 2];
    let template_columns = create_memo(move || {
        format!(
            "grid-template-columns: {} {} auto",
            widths[0].get_clone(),
            widths[1].get_clone(),
        )
    });

    view! {
        header::Header(set_files=directory) {}
        main(class=styles["main"]) {
            MultiRange {}
            div(class=styles["thead"], style=template_columns.get_clone()) {
                p(class=styles["th"]) {
                    "file name"
                    div(class=styles["slider"]) {}
                }
                p(class=styles["th"]) {
                    "creation time"
                    div(class=styles["slider"])
                }
                div {}
            }
            div(class=styles["tbody"], style=template_columns.get_clone()) {
                Keyed(
                    list=files,
                    view=DetailsItem,
                    key=|file| file.id,
                )
            }
        }
    }
}
