mod app;
mod components;
mod domain;
mod functions;

fn main() {
    yew::Renderer::<app::App>::new().render();
}
