#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::pages::{Menu, MenuPage};
mod components;
mod pages;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    launch(App)
}

fn App() -> Element {
    const _TAILWIND_URL: &str = manganis::mg!(file(r"tailwind.css"));
    let masd: Signal<Menu> = Signal::new(Menu::new());

    rsx! {
           MenuPage{
            menu:masd()
           }
    }
}
