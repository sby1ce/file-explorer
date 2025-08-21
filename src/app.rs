mod context_menu;
mod details;
mod grid;
mod header;

use sycamore::prelude::*;
use wasm_bindgen::prelude::*;

use crate::app::{context_menu::ItemView, details::DetailsView, grid::GridView};
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

#[component(inline_props)]
fn Main(
    item_view: ReadSignal<ItemView>,
    directory: ReadSignal<PickedDirectory>,
    selected: Signal<Option<u32>>,
) -> View {
    // .get() is not reactive in just plain function, need to wrap it
    let main_ = move || match item_view.get() {
        ItemView::Grid => view!(GridView(directory = directory, selected = selected)),
        ItemView::Details => view!(DetailsView(directory = directory, selected = selected)),
    };
    view!((main_()))
}

#[component]
pub fn App() -> View {
    let directory: Signal<PickedDirectory> = create_signal(PickedDirectory::default());

    let selected: Signal<Option<u32>> = create_signal(None);

    let item_view: Signal<ItemView> = create_signal(ItemView::Details);
    provide_context(item_view);

    create_effect(move || {
        console_dbg!(selected.get());
    });

    view! {
        header::Header(set_files=directory)

        Main(
            item_view=*item_view,
            directory=*directory,
            selected=selected
        )

        context_menu::ContextMenu {}
    }
}
