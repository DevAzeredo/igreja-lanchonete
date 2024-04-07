use dioxus::prelude::*;
use dioxus::{core_macro::rsx, dioxus_core::Element};

use crate::components::item::Item;
use crate::components::item::ItemComponent;
use crate::components::review_item::ReviewItemComponent;

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
          div{ class:"mb-8 flex min-h-screen flex-col items-center justify-center",
          h1{ class:"mb-8 text-center text-4xl font-bold" ,
                          "Menu"},
          div{ class:"flex max-w-4xl items-center justify-center",

          ItemList{menu:props.menu.clone()},
            Footer{menu:props.menu.clone()}
        }
      }
    }
      }
}

fn ItemList(props: Props) -> Element {
    let exibir = true;

    rsx! {
            div {
           if exibir { div {ReviewItemComponent{items:props.menu.items.clone()}}}
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
}
fn Footer(mut props: Props) -> Element {
    rsx! {
        div{ class:"fixed bottom-0 left-0 right-0 flex items-center justify-center p-4",
        div{ class:"flex items-center",
          div{ class:"mr-2",
          "Total: R${props.menu.TotalSelectedItensFormated()}"}
        },
          button{ class:"rounded px-2 py-1 bg-button-200",
          " Abrir Carrinho"
          //onclick: |_| props.menu.ClearCart(),
        }
    }
    }
}
