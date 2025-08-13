use std::cell::RefCell;

use rclite::Rc;
use sycamore::prelude::*;
use wasm_bindgen::{JsCast, prelude::Closure};
use web_sys::{HtmlElement, MouseEvent};

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
fn ResizableColumn() -> View {
    let styles = css_mod::get!("details.css");

    let is_resizing = create_signal(false);
    let column = create_node_ref();
    let initial_x = create_signal(0);
    let initial_width = create_signal(100);
    let delta_x = create_signal(0);

    let style = move || format!("width: {}px", initial_width.get() + delta_x.get());

    let mouse_move = Rc::new(Closure::wrap(Box::new(move |event: MouseEvent| {
        if !is_resizing.get() {
            return;
        }
        delta_x.set(event.client_x() - initial_x.get());
    }) as Box<dyn FnMut(MouseEvent)>));
    let mouse_up = {
        // can't recurse closures in rust so we have this indirection
        let closure: Rc<RefCell<Closure<dyn FnMut()>>> =
            Rc::new(RefCell::new(Closure::new(Box::new(|| {}))));
        let clone = closure.clone();
        let mouse_move_clone = mouse_move.clone();
        let mouse_up = move || {
            is_resizing.set(false);

            let window = window();
            window
                .remove_event_listener_with_callback(
                    "mousemove",
                    mouse_move_clone.as_ref().as_ref().unchecked_ref(),
                )
                .unwrap();
            window
                .remove_event_listener_with_callback(
                    "mouseup",
                    clone.borrow().as_ref().unchecked_ref(),
                )
                .unwrap();
        };
        closure.replace(Closure::wrap(Box::new(mouse_up) as Box<dyn FnMut()>));
        closure
    };
    let mouse_move_clone = mouse_move.clone();
    let mouse_up_clone = mouse_up.clone();
    let mouse_down = move |event: MouseEvent| {
        is_resizing.set(true);
        initial_x.set(event.client_x());
        initial_width.set(
            column
                .get()
                .dyn_into::<HtmlElement>()
                .unwrap()
                .offset_width(),
        );
        event.prevent_default();

        let window = window();
        window
            .add_event_listener_with_callback(
                "mousemove",
                mouse_move_clone.as_ref().as_ref().unchecked_ref(),
            )
            .unwrap();
        window
            .add_event_listener_with_callback(
                "mouseup",
                mouse_up_clone.borrow().as_ref().unchecked_ref(),
            )
            .unwrap();
    };
    view! {
        div(
            r#ref=column,
            style=style,
            class=styles["column"],
        ) {
            span {
                "file name"
            }
            div(
                on:mousedown=mouse_down,
                class=styles["handle"],
            )
        }
    }
}

#[component]
pub fn OkBruh() -> View {
    view! {
        ResizableColumn {}
    }
}
