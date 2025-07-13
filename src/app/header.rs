use sycamore::{futures::spawn_local_scoped, prelude::*};
use wasm_bindgen_futures::js_sys::Array;

use crate::app::{EmptyArgs, convert_file_src, invoke};

fn pick_directory() {
    let args = serde_wasm_bindgen::to_value(&EmptyArgs).unwrap();
    spawn_local_scoped(async move {
        let paths = Array::from(&invoke("pick_directory", args).await);
        paths.into_iter().map(convert_file_src).for_each(|url| {
            console_dbg!(url);
        });
    });
}

#[component]
pub fn Header() -> View {
    let styles = css_mod::get!("header.css");
    view! {
        header(class=styles["header"]) {
            button(r#type="button", on:click=|_e| { pick_directory(); }) {
                "open"
            }
        }
    }
}
