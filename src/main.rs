mod app;
mod buttons;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
