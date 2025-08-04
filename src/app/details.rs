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

    // from-position and to-position control the clip-path that separates the dual slider
    let from_position: Signal<f64> = create_signal(10_f64);
    let to_position: Signal<f64> = create_signal(80_f64);

    let input_style: Signal<String> = create_signal(String::new());

    let from_max = 200.0;
    let from_min = 0.0;

    let to_max = 200.0;
    let to_min = 0.0;

    let from_input = move |_e| {
        let from = from_position.get();
        let to = to_position.get();
        if from > to {
            from_position.set(to);
        }

        let range_distance = from_max - from_min;
        let from_percentage = (from_position.get() - from_min) / range_distance * 100.0;
        let to_percentage = (to - to_min) / range_distance * 100.0;

        let style: String = format!(
            "--from-position: {from_percentage}%; --to-position: {to_percentage}%"
        );
        input_style.set(style);
    };
    let to_input = move |_e| {
        let from = from_position.get();
        let to = to_position.get();
        if from > to {
            to_position.set(from);
        }

        let range_distance = from_max - from_min;
        let from_percentage = (from - from_min) / range_distance * 100.0;
        let to_percentage = (to_position.get() - to_min) / range_distance * 100.0;

        let style: String = format!(
            "--from-position: {from_percentage}%; --to-position: {to_percentage}%"
        );
        input_style.set(style);
    };

    view! {
        div(class=styles["double-range"]) {
            input(
                r#type="range", 
                min=from_min.to_string(),
                max=from_max.to_string(),
                bind:valueAsNumber=from_position, 
                class=format!("{} {}", styles["range"], styles["from"]),
                style=input_style.get_clone(),
                on:input=from_input
            )
            input(
                r#type="range", 
                min=to_min.to_string(),
                max=to_max.to_string(),
                bind:valueAsNumber=to_position, 
                class=styles["range"],
                on:input=to_input
            )
        }
    }
}
