use crate::app::App;

mod app;
mod engine;
mod model;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
