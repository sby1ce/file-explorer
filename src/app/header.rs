use sycamore::{futures::spawn_local_scoped, prelude::*};
use wasm_bindgen_futures::js_sys::Uint8Array;

use crate::app::invoke;
use fe_types::PickedDirectory;

fn pick_directory(set_files: Signal<PickedDirectory>) {
    spawn_local_scoped(async move {
        let constructor_arg = invoke("pick_directory").await;
        let Some(paths) = postcard::from_bytes::<Option<PickedDirectory>>(
            &Uint8Array::new(&constructor_arg).to_vec(),
        )
        .unwrap() else {
            return;
        };
        set_files.set(paths);
    });
}

#[component(inline_props)]
pub fn Header(set_files: Signal<PickedDirectory>, item_view: Signal<bool>) -> View {
    let styles = css_mod::get!("header.css");
    view! {
        header(class=styles["header"]) {
            button(
                r#type="button",
                class=styles["button"],
                on:click=move |_e| pick_directory(set_files)
            ) { "open" }

            button(
                r#type="button",
                class=styles["button"],
                on:click=move |_e| item_view.set_fn(|iv| !iv),
            ) { "view" }
        }
    }
}
