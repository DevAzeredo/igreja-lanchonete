#![allow(non_snake_case, unused)]
use dioxus::{prelude::*, web::launch};
mod item;
use item::Item;
fn main() {
    launch(App)
}

fn App() -> Element {
    const _TAILWIND_URL: &str = manganis::mg!(file(r"tailwind.css"));
    rsx! { Item {} }
}