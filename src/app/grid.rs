use sycamore::prelude::*;

use fe_types::{FileData, PickedDirectory};

#[component(inline_props)]
fn GridItem(file_data: FileData, select: impl Fn(u32) + 'static) -> View {
    let styles = css_mod::get!("grid.css");

    view! {
        div(
            on:click=move |_e| select(file_data.id),
            aria-role="button",
            tabindex="0",
            class=styles["item"],
        ) {
            (file_data.file_name)
        }
    }
}

#[component(inline_props)]
pub fn GridView(directory: ReadSignal<PickedDirectory>, selected: Signal<Option<u32>>) -> View {
    let styles = css_mod::get!("grid.css");

    let files = move || directory.get_clone().files;

    view! {
        main(class=styles["main"]) {
            Keyed(
                list=files,
                view=move |file| view!(GridItem(
                    file_data=file,
                    select=move |id| selected.set(Some(id)),
                )),
                key=|file| file.id,
            )
        }
    }
}
