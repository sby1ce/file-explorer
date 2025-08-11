use sycamore::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;

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

    let range_ref = create_node_ref();
    let max_width = create_memo(move || {
        // sycamore is_ssr macros are producing clippy warnings
        #[cfg(target_arch = "wasm32")]
        let is_server = false;
        #[cfg(not(target_arch = "wasm32"))]
        let is_server = true;
        if is_server {
            f64::from(
                range_ref
                    .get()
                    .dyn_into::<Element>()
                    .map(|el: Element| el.client_width())
                    .unwrap_or(1000),
            )
        } else {
            1000.0
        }
    });

    let binds: [Signal<f64>; 4] = [
        create_signal(50.0),
        create_signal(100.0),
        create_signal(150.0),
        create_signal(200.0),
    ];
    let get_binds = move || {
        [
            binds[0].get(),
            binds[1].get(),
            binds[2].get(),
            binds[3].get(),
        ]
    };

    let input = move |idx: usize| {
        let value = binds[idx].get();
        let p = get_binds();
        let min = if idx == 0 { 0.0 } else { p[idx - 1] };
        let max = *p.get(idx + 1).unwrap_or(&max_width.get());
        binds[idx].set(f64::min(max, f64::max(min, value)));
    };

    view! {
        div(
            r#ref=range_ref,
            class=styles["multi-range"],
        ) {
            input(
                r#type="range",
                min="0",
                max=max_width.get().to_string(),
                // order of bind and on matters
                bind:valueAsNumber=binds[0],
                on:input=move |_| input(0),
                class=format!("{} {}", styles["range"], styles["from"]),
                style="--right-offset: var(--offset-1); --left-offset: var(--offset-0)",
            )
            input(
                r#type="range",
                min="0",
                max=max_width.get().to_string(),
                bind:valueAsNumber=binds[1],
                on:input=move |_| input(1),
                class=format!("{} {}", styles["range"], styles["from"]),
                style="--right-offset: var(--offset-2); --left-offset: var(--offset-1)",
            )
            input(
                r#type="range",
                min="0",
                max=max_width.get().to_string(),
                bind:valueAsNumber=binds[2],
                on:input=move |_| input(2),
                class=format!("{} {}", styles["range"], styles["from"]),
                style="--right-offset: var(--offset-3); --left-offset: var(--offset-2)",
            )
            input(
                r#type="range",
                min="0",
                max=max_width.get().to_string(),
                bind:valueAsNumber=binds[3],
                on:input=move |_| input(3),
                class=styles["range"],
                style="--right-offset: var(--offset-4); --left-offset: var(--offset-3)",
            )
        }
    }
}
