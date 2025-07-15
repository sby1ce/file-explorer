use sycamore::prelude::*;

use fe_types::FileData;

#[component]
pub fn DetailsItem(file_data: FileData) -> View {
    let styles = css_mod::get!("details.css");
    view! {
        tr {
            td(class=styles["td"]) {
                (file_data.id)
            }
            td(class=styles["td"]) {
                (file_data.path)
            }
        }
    }
}
