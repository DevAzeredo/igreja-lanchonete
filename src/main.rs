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
        div{ class:"mb-8 flex min-h-screen flex-col items-center justify-center",
        h1{ class:"mb-8 text-center text-4xl font-bold" ,
                        "Menu"},
        div{ class:"flex max-w-4xl items-center justify-center",
           MenuPage{
            menu:masd()
           }
        }}
    }
}
