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
        name: use_signal(|| String::from("Hambúrguer")),
        price: 17.99,
        quantity: use_signal(|| 0),
        description: String::from("Hambúrguer delicioso feito com muito amor --->"),
    };
    let mut count = use_signal(|| 0);
    rsx! { Item {item:items} }

}
