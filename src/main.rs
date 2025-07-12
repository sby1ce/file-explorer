mod app;

use app::App;

fn main() {
    console_error_panic_hook::set_once();
    css_mod::init!();
    sycamore::render(App);
}
