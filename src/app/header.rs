use sycamore::{futures::spawn_local_scoped, prelude::*};

use crate::app::{EmptyArgs, invoke};

fn pick_directory() {
    let args = serde_wasm_bindgen::to_value(&EmptyArgs).unwrap();
    spawn_local_scoped(async move {
        invoke("pick_directory", args).await;
    });
}

#[component]
pub fn Header() -> View {
    let css = css_mod::get!("header.css");
    view! {
        header(class=css["header"]) {
            button(r#type="button", on:click=|_e| { pick_directory(); }) {
                "open"
            }
        }
    }
}
