use rclite::Rc;
use sycamore::prelude::*;
use wasm_bindgen::{JsCast, prelude::Closure};
use web_sys::{AddEventListenerOptions, HtmlElement, MouseEvent};

use fe_types::{FileData, PickedDirectory};

#[component(inline_props)]
fn DetailsItem(
    file_data: FileData,
    select: impl Fn(u32) + Copy + 'static,
    deselect: impl Fn() + Copy + 'static,
) -> View {
    let styles = css_mod::get!("details.css");

    let file_id: u32 = file_data.id;

    view! {
        div(class=styles["table-row"], on:click=move |_e| select(file_id)) {
            p(class=styles["p"]) {
                (file_data.file_name)
            }
            p(class=styles["p"]) {
                (file_data.creation_time.format())
            }
            p(class=styles["p"]) {
                (file_data.extension)
            }
            div(on:click=move |e: MouseEvent| {
                e.stop_propagation();
                deselect();
            })
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DetailsColumn {
    FileName,
    CreatedAt,
    Extension,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SortOptions {
    FileName(bool),
    CreatedAt(bool),
    Extension(bool),
}

#[derive(Debug, Clone, Copy, Eq)]
struct ColumnProps {
    pub width: Signal<i32>,
    pub title: &'static str,
    pub sort_options: Signal<Option<SortOptions>>,
    pub column: DetailsColumn,
}

impl PartialEq for ColumnProps {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && self.title == other.title
    }
}

impl ColumnProps {
    pub fn new(
        initial_width: i32,
        title: &'static str,
        sort_options: Signal<Option<SortOptions>>,
        column: DetailsColumn,
    ) -> Self {
        Self {
            width: create_signal(initial_width),
            title,
            sort_options,
            column,
        }
    }
    pub fn dispose(self) {
        self.width.dispose();
        // sort_options belongs to the parent so we're not disposing of it here
    }
    pub fn click(&self) {
        let new = match (self.column, self.sort_options.get()) {
            (DetailsColumn::FileName, Some(SortOptions::FileName(false))) => {
                Some(SortOptions::FileName(true))
            }
            (DetailsColumn::FileName, Some(SortOptions::FileName(true))) => None,
            (DetailsColumn::FileName, _) => Some(SortOptions::FileName(false)),

            (DetailsColumn::CreatedAt, Some(SortOptions::CreatedAt(false))) => {
                Some(SortOptions::CreatedAt(true))
            }
            (DetailsColumn::CreatedAt, Some(SortOptions::CreatedAt(true))) => None,
            (DetailsColumn::CreatedAt, _) => Some(SortOptions::CreatedAt(false)),

            (DetailsColumn::Extension, Some(SortOptions::Extension(false))) => {
                Some(SortOptions::Extension(true))
            }
            (DetailsColumn::Extension, Some(SortOptions::Extension(true))) => None,
            (DetailsColumn::Extension, _) => Some(SortOptions::Extension(false)),
        };
        self.sort_options.set(new);
    }
    pub fn order(&self) -> &'static str {
        match (self.column, self.sort_options.get()) {
            (DetailsColumn::FileName, Some(SortOptions::FileName(false)))
            | (DetailsColumn::CreatedAt, Some(SortOptions::CreatedAt(false)))
            | (DetailsColumn::Extension, Some(SortOptions::Extension(false))) => "rotate: -90deg",
            (DetailsColumn::FileName, Some(SortOptions::FileName(true)))
            | (DetailsColumn::CreatedAt, Some(SortOptions::CreatedAt(true)))
            | (DetailsColumn::Extension, Some(SortOptions::Extension(true))) => "rotate: 90deg",
            _ => "display: none",
        }
    }
}

#[component(inline_props)]
fn ResizableColumn(
    width: Signal<i32>,
    title: &'static str,
    click: impl Fn() + 'static,
    icon: ReadSignal<&'static str>,
) -> View {
    let styles = css_mod::get!("details.css");

    let is_resizing = create_signal(false);
    let column = create_node_ref();
    let initial_x = create_signal(0);
    let initial_width = create_signal(width.get());
    let delta_x = create_signal(0);

    create_effect(move || {
        width.set(std::cmp::max(50, initial_width.get() + delta_x.get()));
    });

    let mouse_move: Rc<Closure<dyn FnMut(MouseEvent)>> =
        Rc::new(Closure::wrap(Box::new(move |event: MouseEvent| {
            if !is_resizing.get() {
                return;
            }
            delta_x.set(event.client_x() - initial_x.get());
        }) as Box<dyn FnMut(MouseEvent)>));
    let up_listener_options: AddEventListenerOptions = AddEventListenerOptions::new();
    up_listener_options.set_once(true);
    up_listener_options.set_passive(true);
    let move_listener_options: AddEventListenerOptions = AddEventListenerOptions::new();
    move_listener_options.set_passive(true);
    let mouse_up: Closure<dyn FnMut()> = {
        let mouse_move_clone: Rc<Closure<dyn FnMut(MouseEvent)>> = mouse_move.clone();
        Closure::wrap(Box::new(move || {
            is_resizing.set(false);

            let window = window();
            window
                .remove_event_listener_with_callback(
                    "mousemove",
                    mouse_move_clone.as_ref().as_ref().unchecked_ref(),
                )
                .unwrap();
        }) as Box<dyn FnMut()>)
    };
    let mouse_move_clone: Rc<Closure<dyn FnMut(MouseEvent)>> = mouse_move.clone();
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
            .add_event_listener_with_callback_and_add_event_listener_options(
                "mousemove",
                mouse_move_clone.as_ref().as_ref().unchecked_ref(),
                &move_listener_options,
            )
            .unwrap();
        window
            .add_event_listener_with_callback_and_add_event_listener_options(
                "mouseup",
                mouse_up.as_ref().unchecked_ref(),
                &up_listener_options,
            )
            .unwrap();
    };
    view! {
        div(
            r#ref=column,
            class=styles["column"],
        ) {
            button(on:click=move |_| click()) {
                (title)

                svg(
                    version="1.1",
                    xmlns="http://www.w3.org/2000/svg",
                    viewBox="0 0 185.343 185.343",
                    style=icon,
                ) {
                    g {
                        path(
                            fill="#FFFFFF",
                            d="M51.707,185.343c-2.741,0-5.493-1.044-7.593-3.149c-4.194-4.194-4.194-10.981,0-15.175
			l74.352-74.347L44.114,18.32c-4.194-4.194-4.194-10.987,0-15.175c4.194-4.194,10.987-4.194,15.18,0l81.934,81.934
			c4.194,4.194,4.194,10.987,0,15.175l-81.934,81.939C57.201,184.293,54.454,185.343,51.707,185.343z"
                        )
                    }
                }
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
fn TableHead(props: Signal<Vec<ColumnProps>>) -> View {
    fn column_view(props: ColumnProps) -> View {
        // prevent memory leaking when column is removed
        // https://sycamore.dev/book/introduction/rendering-lists#nested-reactivity
        on_cleanup(move || props.dispose());
        // have to wrap in reate memo because can't pass derived closures and MaybeDyn isn't workign with &str
        // https://sycamore.dev/book/introduction/adding-state#reactive-components
        let icon: ReadSignal<&'static str> = create_memo(move || props.order());
        view!(ResizableColumn(
            width = props.width,
            title = props.title,
            click = move || props.click(),
            icon = icon,
        ))
    }
    view! {
        Indexed(
            list=props,
            view=column_view,
        )
        div {}
    }
}

#[component(inline_props)]
pub fn DetailsView(directory: ReadSignal<PickedDirectory>, selected: Signal<Option<u32>>) -> View {
    let styles = css_mod::get!("details.css");

    let sort_options: Signal<Option<SortOptions>> = create_signal(None);

    let files = move || {
        let mut files = directory.get_clone().files;
        match sort_options.get() {
            None => files,
            Some(SortOptions::FileName(reverse)) => {
                files.sort_unstable_by(|file1: &FileData, file2: &FileData| {
                    file1.file_name.cmp(&file2.file_name)
                });
                if reverse {
                    files.reverse();
                }
                files
            }
            Some(SortOptions::CreatedAt(reverse)) => {
                files.sort_unstable_by_key(|file: &FileData| file.creation_time);
                if reverse {
                    files.reverse();
                }
                files
            }
            Some(SortOptions::Extension(reverse)) => {
                files.sort_unstable_by(|file1: &FileData, file2: &FileData| {
                    file1.extension.cmp(&file2.extension)
                });
                if reverse {
                    files.reverse();
                }
                files
            }
        }
    };

    // creating vec with map because `Signal` is clonable
    // so Rust clones the same signal for all elements
    let props: Signal<Vec<ColumnProps>> = create_signal(vec![
        ColumnProps::new(200, "file name", sort_options, DetailsColumn::FileName),
        ColumnProps::new(200, "created at", sort_options, DetailsColumn::CreatedAt),
        ColumnProps::new(100, "extension", sort_options, DetailsColumn::Extension),
    ]);

    let style = move || {
        let widths = props.get_clone();
        // don't forget to change in table-row
        format!(
            "grid-template-columns: {}px {}px {}px auto",
            widths[0].width.get(),
            widths[1].width.get(),
            widths[2].width.get(),
        )
    };

    view! {
        main(class=styles["main"], style=style) {
            TableHead(props=props) {}

            Keyed(
                list=files,
                view=move |file| {
                    view! {
                        DetailsItem(
                            file_data=file,
                            select=move |id| selected.set(Some(id)),
                            deselect=move || selected.set(None),
                        )
                    }
                },
                key=|file| file.id,
            )
        }
    }
}
