use sycamore::prelude::*;

#[component]
pub fn Header() -> View {
    let css = css_mod::get!("my-component.css");
    view! {
        header(class=css["header"]) {
            "header"
        }
    }
}
