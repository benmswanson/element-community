// frontend/src/main.rs

mod app;
mod components;
mod pages;

fn main() {
    yew::Renderer::<app::App>::new().render();
}
