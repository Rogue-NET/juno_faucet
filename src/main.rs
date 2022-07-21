mod app;
mod verify;
mod state;

use app::App;

fn main() {
    yew::start_app::<App>();
}
