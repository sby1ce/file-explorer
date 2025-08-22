use sycamore::prelude::*;
use wasm_bindgen::{JsCast, prelude::Closure};
use web_sys::{HtmlElement, MouseEvent, SubmitEvent};

#[derive(Debug, Clone, Copy)]
pub enum ItemView {
    Details,
    Grid,
}

#[component(inline_props)]
fn ContextMenuOption(label: &'static str, click: impl Fn() + 'static) -> View {
    view!(button(r#type="submit", on:click=move |_e| click()) {
        (label)
    })
}

#[component(inline_props)]
fn ContextMenuItem(node_ref: NodeRef, children: Children, id: &'static str) -> View {
    let styles = css_mod::get!("context_menu.css");
    // have for styles unforuntately because can't pass generic static strings
    let fieldset_style = format!(
        "position-anchor: --{id}; inset-block-start: anchor(--{id} top); inset-inline-start: anchor(--{id} right)"
    );
    view! {
        button(
            r#type="button",
            "popovertarget"=id,
            "popovertargetaction"="show",
            style=format!("anchor-name: --{id}"),
        ) {
            "item view"
        }
        fieldset(
            id=id,
            popover="",
            r#ref=node_ref,
            class=styles["fieldset"],
            style=fieldset_style,
        ) {
            (children)
        }
    }
}

#[component(inline_props)]
pub fn ContextMenu() -> View {
    let styles = css_mod::get!("context_menu.css");
    let item_view: Signal<ItemView> = use_context::<Signal<ItemView>>();

    let popover = create_node_ref();

    let abs_x: Signal<i32> = create_signal(0);
    let abs_y: Signal<i32> = create_signal(0);
    let position = move || format!("top: {}px; left: {}px", abs_y.get(), abs_x.get(),);

    let window = window();
    let on_context = Closure::wrap(Box::new(move |event: MouseEvent| {
        event.prevent_default();
        abs_x.set(event.page_x());
        abs_y.set(event.page_y());
        popover
            .get()
            .dyn_into::<HtmlElement>()
            .unwrap()
            .show_popover()
            .unwrap();
    }) as Box<dyn FnMut(MouseEvent)>);
    window
        .add_event_listener_with_callback("contextmenu", on_context.as_ref().unchecked_ref())
        .unwrap();
    // leaking closure otherwise it gets dropped
    on_context.forget();

    let submit = move |event: SubmitEvent| {
        event.prevent_default();
        popover
            .get()
            .dyn_into::<HtmlElement>()
            .unwrap()
            .hide_popover()
            .unwrap();
    };

    let menu_items: [NodeRef; 1] = [create_node_ref()];

    view! {
        form(
            r#ref=popover,
            class=styles["dialog"],
            style=position,
            on:submit=submit,
            popover="",
        ) {
            ContextMenuItem(node_ref=menu_items[0], id="item-view-select") {
                ContextMenuOption(
                    label="details",
                    click=move || item_view.set(ItemView::Details),
                )
                ContextMenuOption(
                    label="grid",
                    click=move || item_view.set(ItemView::Grid),
                )
            }
        }
    }
}
