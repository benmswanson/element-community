// frontend/src/main.rs

mod app;
mod community;
mod components;
mod pages;

fn main() {
    yew::Renderer::<app::App>::new().render();
}
