mod app;
use app::App;
use console_error_panic_hook;
use leptos::mount_to_body;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
