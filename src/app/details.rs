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
        p(class=styles["p"]) {
            (file_data.extension)
        }
        div {}
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColumnProps {
    pub width: Signal<i32>,
    pub title: &'static str,
}

impl ColumnProps {
    pub fn new(initial_width: i32, title: &'static str) -> Self {
        Self {
            width: create_signal(initial_width),
            title,
        }
    }
    pub fn dispose(self) {
        self.width.dispose();
    }
}

#[component(inline_props)]
fn ResizableColumn(width: Signal<i32>, title: &'static str) -> View {
    let styles = css_mod::get!("details.css");

    let is_resizing = create_signal(false);
    let column = create_node_ref();
    let initial_x = create_signal(0);
    let initial_width = create_signal(width.get());
    let delta_x = create_signal(0);

    create_effect(move || {
        width.set(std::cmp::max(50, initial_width.get() + delta_x.get()));
    });

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
        initial_width.set(
            column
                .get()
                .dyn_into::<HtmlElement>()
                .unwrap()
                .offset_width(),
        );
        delta_x.set(0);
        initial_x.set(event.client_x());
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
            class=styles["column"],
        ) {
            span {
                (title)
            }
            div(
                on:mousedown=mouse_down,
                class=styles["handle"],
            ) {
                span {}
            }
        }
    }
}

#[component(inline_props)]
pub fn TableHead(props: Signal<Vec<ColumnProps>>) -> View {
    fn column_view(props: ColumnProps) -> View {
        // prevent memory leaking when column is removed
        // https://sycamore.dev/book/introduction/rendering-lists#nested-reactivity
        on_cleanup(move || props.dispose());
        view!(ResizableColumn(width = props.width, title = props.title))
    }
    view! {
        Indexed(
            list=props,
            view=column_view,
        )
        div {}
    }
}
