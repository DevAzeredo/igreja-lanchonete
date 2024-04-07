use dioxus::prelude::*;
use dioxus::{core_macro::rsx, dioxus_core::Element};

use crate::components::item::Item;
use crate::components::item::ItemComponent;

#[derive(PartialEq, Clone)]
pub struct Menu {
    pub items: Signal<Vec<Item>>,
}
#[derive(PartialEq, Clone, Props)]
pub struct Props {
    pub menu: Menu,
}

pub fn MenuPage(mut props: Props) -> Element {
    if props.menu.items.len() == 0 {
        props.menu.LoadItems();
    };
    rsx! {
        div { class: "menu",
        ItemList{menu:props.menu.clone()},
          Footer{menu:props.menu.clone()}}

    }
}

/* <button
  className='rounded bg-gray-200 px-2 py-1 text-gray-700'
  onClick={onClick}
>
  Abrir Carrinho
</button> */
fn ItemList(mut props: Props) -> Element {
    rsx! {
        div {
            ul {
                li {
                for i in props.menu.items.iter() {
                    ItemComponent {
                        id:&i.id,
                        name:i.name,
                        price:i.price,
                        quantity:i.quantity,
                        description:&i.description,
                        image:&i.image,
                    }
                }
            }
     }
        }
    }
}
fn Footer(mut props: Props) -> Element {
    rsx! {
        div{ class:"fixed bottom-0 left-0 right-0 flex items-center justify-center bg-gray-100 p-4",
        div{ class:"flex items-center",
          div{ class:"mr-2",
          "Total: R${props.menu.TotalSelectedItensFormated()}"}
        },
          button{ class:"rounded bg-gray-200 px-2 py-1 text-gray-700",
          " Abrir Carrinho"
          //onclick: |_| props.menu.ClearCart(),
        }
    }
    }
}
