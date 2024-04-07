use dioxus::prelude::*;
use dioxus::{core_macro::rsx, dioxus_core::Element};

use crate::components::item::{Item, ItemComponent};

#[derive(PartialEq, Clone, Props)]
pub struct SelectedItems {
    pub items: Signal<Vec<Item>>,
}

pub fn ReviewItemComponent(mut selectedItems: SelectedItems) -> Element {
    rsx! {
      div {
        class:"bg-blur fixed inset-0 flex items-center justify-center bg-gray-900 bg-opacity-50",
        div {
          class:"flex flex-col items-center rounded bg-white p-4",
          div{
            class:"flex h-96 w-96 flex-col overflow-y-scroll",
            div {
              ul {
                  li {
                  for i in selectedItems.items.iter() {
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
    }
}
