use dioxus::{core_macro::rsx, dioxus_core::Element};
use dioxus::{prelude::*, web::launch};

struct Item {
    name: String,
    price: f32,
    quantity: i32,
    description: String,
}

pub fn Item() -> Element {
    let item = Item {
        name: String::from("Item"),
        price: 0.0,
        quantity: 0,
        description: String::from(""),
    };

    rsx!( div {class:"text-red-500", "nome" }
    div { "Preço" }
    div { "Quantidade" }
    div { "Descrição" } )
}
