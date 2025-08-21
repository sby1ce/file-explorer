use sycamore::prelude::*;
use wasm_bindgen::{JsCast, prelude::Closure};
use web_sys::{HtmlElement, MouseEvent, SubmitEvent};

const ITEM_VIEW_SELECT: &str = "item-view-select";

#[derive(Debug, Clone, Copy)]
pub enum ItemView {
    Details,
    Grid,
}

#[component(inline_props)]
pub fn ContextMenu() -> View {
    let styles = css_mod::get!("context_menu.css");
    let item_view: Signal<ItemView> = use_context::<Signal<ItemView>>();

    let popover = create_node_ref();

    let abs_x: Signal<i32> = create_signal(0);
    let abs_y: Signal<i32> = create_signal(0);
    let position = move || format!(
        "top: {}px; left: {}px", abs_y.get(), abs_x.get(),
    );

    let window = window();
    let on_context = Closure::wrap(Box::new(move |event: MouseEvent| {
        event.prevent_default();
        abs_x.set(event.page_x());
        abs_y.set(event.page_y());
        popover.get().dyn_into::<HtmlElement>().unwrap().show_popover().unwrap();
    }) as Box<dyn FnMut(MouseEvent)>);
    window
        .add_event_listener_with_callback("contextmenu", on_context.as_ref().unchecked_ref())
        .unwrap();
    // leaking closure otherwise it gets dropped
    on_context.forget();

    let submit = move |event: SubmitEvent| {
        event.prevent_default();
        popover.get().dyn_into::<HtmlElement>().unwrap().hide_popover().unwrap();
    };

    view! {
        form(
            r#ref=popover,
            class=styles["dialog"], 
            style=position, 
            on:submit=submit,
            popover="",
        ) {
            button(
                r#type="button",
                "popovertarget"=ITEM_VIEW_SELECT,
                "popovertargetaction"="show",
                style="anchor-name: --item-view-select",
            ) { 
                "item view" 
            }
            fieldset(
                id=ITEM_VIEW_SELECT,
                popover="",
                class=styles["fieldset"],
                style="position-anchor: --item-view-select; inset-block-start: anchor(--item-view-select top); inset-inline-start: anchor(--item-view-select right)"
            ) {
                button(
                    r#type="submit",
                    on:click=move |_e| item_view.set(ItemView::Details),
                ) { "details" }
                button(
                    r#type="submit",
                    on:click=move |_e| item_view.set(ItemView::Grid),
                ) { "grid" }
            }
        }
    }
}
