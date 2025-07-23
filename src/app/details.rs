use sycamore::prelude::*;

use fe_types::FileData;

#[component]
pub fn DetailsItem(file_data: FileData) -> View {
    let styles = css_mod::get!("details.css");
    view! {
        p(class=styles["p"]) {
            (file_data.file_name)
        }
        p(class=styles["p"]) {
            (file_data.creation_time.format())
        }
        div {}
    }
}

#[component]
pub fn MultiRange() -> View {
    let styles = css_mod::get!("details.css");
    view! {
        div(class=styles["double-range"]) {
            input(r#type="range", class=format!("{} {}", styles["range"], styles["from"]))
            input(r#type="range", class=styles["range"])
        }
    }
}
