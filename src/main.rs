mod app;
mod components;
mod domain;
mod functions;
mod pages;
mod partials;
mod routes;

fn main() {
    yew::Renderer::<app::App>::new().render();
}
