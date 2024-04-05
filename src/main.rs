#![allow(non_snake_case, unused)]
use dioxus::{prelude::*, web::launch};
mod item;
use item::ItemComponent;
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
    let mut itemz = Item {
        name: use_signal(|| String::from("Hambúrguer X salada")),
        price: 1.99,
        quantity: use_signal(|| 0),
        description: String::from("Hambúrguer delicioso feito com muito amor e com salada --->"),
    };

    let array: Vec<Item> = vec![items, itemz];

    rsx! {
        ul {
            for (i) in array.iter() {
                ItemComponent {
                    name:i.name,
                    price:i.price,
                    quantity:i.quantity,
                    description:&i.description,
                }
            }
        }
    }
}
