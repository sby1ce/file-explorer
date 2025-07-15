use std::path::Path;

use sycamore::prelude::*;

use fe_types::FileData;

#[component]
pub fn DetailsItem(file_data: FileData) -> View {
    let styles = css_mod::get!("details.css");
    let filename = Path::new(&file_data.path).file_name().unwrap().display().to_string();
    view! {
        p(class=styles["p"]) {
            (filename)
        }
        p(class=styles["p"]) {
            (file_data.creation_time.format())
        }
    }
}
