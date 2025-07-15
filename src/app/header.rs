use sycamore::{futures::spawn_local_scoped, prelude::*};
use wasm_bindgen_futures::js_sys::Array;

use crate::app::{EmptyArgs, invoke};
use fe_types::FileData;

fn pick_directory(set_files: Signal<Vec<FileData>>) {
    let args = serde_wasm_bindgen::to_value(&EmptyArgs).unwrap();
    spawn_local_scoped(async move {
        let paths = Array::from(&invoke("pick_directory", args).await);
        set_files.set(
            paths
                .into_iter()
                .enumerate()
                .map(FileData::from)
                .collect(),
        );
    });
}

#[component(inline_props)]
pub fn Header(set_files: Signal<Vec<FileData>>) -> View {
    let styles = css_mod::get!("header.css");
    view! {
        header(class=styles["header"]) {
            button(r#type="button", class=styles["button"], on:click=move |_e| { pick_directory(set_files); }) {
                "open"
            }
        }
    }
}
