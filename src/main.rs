#![allow(non_snake_case, unused)]
use dioxus::{prelude::*, web::launch};
mod item;
use item::Item;
fn main() {
    launch(App)
}

fn App() -> Element {
    const _TAILWIND_URL: &str = manganis::mg!(file(r"tailwind.css"));
    let mut items = Item {
        name: use_signal(|| String::from("Item")),
        price: 0.0,
        quantity: use_signal(|| 9),
        description: String::from(""),
    };
    let mut count = use_signal(|| 0);
    rsx! { Item {asd:items} }

    //name:items.name,price:items.price, quantity:items.quantity,description: items.description
}
