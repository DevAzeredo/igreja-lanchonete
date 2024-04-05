use dioxus::{core_macro::rsx, dioxus_core::Element};
use dioxus::{prelude::*, web::launch};


#[derive(PartialEq, Clone, Props)]
pub struct Item {
    pub name: Signal<String>,
    pub price: f32,
    pub quantity: Signal<i32>,
    pub description: String,
}


pub fn ItemComponent(mut item: Item) -> Element {
    rsx!(

        div { class : "flex mb-4 max-w max-h border p-4 mt-2 mr-2 ml-2",
            div { class : "mr-4 w-5/12",
                img { src : "https://www.estadao.com.br/resizer/dixyOItHmPSgiedCSBL1iIT5lGo=/arc-anglerfish-arc2-prod-estadao/public/GUOGMQ4FRJIUPAWMYLE4WNA3SY.jpg",
                    class : "w-full h-full rounded-md object-cover",
                    alt : "Imagem do produto {item.name}",
                    } /* IMAGEM */
                }
            div { class : "flex flex-grow flex-col justify-between w-6/12",
                    div { class : "mb-auto",
                            h3 { class : "text-lg font-semibold h-1/5" ,
                                "{item.name}" /* NOME */
                               },
                                p { /* DESCRIÇÃO */
                                    class : "break-words text-sm h-4/5 mt-6",
                                            "{item.description}",

                                  },
                        },
                        div { class : "flex justify-between mt-2",
                            div { class : "flex",
                                span { class : "text-lg font-semibold",
                                        "R${item.Total()}"
                                    } /* PREÇO */
                                },

                            div { class : "flex rounded bg-gray-200 px-2 py-1 text-gray-700",
                            if (item.quantity)() > 0 {
                                button { onclick: move |event| {item.quantity -=1;} ,
                                class:"px-2 py-1",
                                        "-"
                                        }/*BUTTON - */

                            div {class:"px-2 py-1 ",
                                    "{item.quantity}"
                                       /*QUANTIDADE */
                                }
                            button { onclick: move |event| {item.quantity +=1;} ,
                                             class:"px-2 py-1 ",
                                                "+"
                                    }/*BUTTON + */
                                }

                        else {
                            button { onclick: move |event| {item.quantity +=1;} ,
                            class:"px-2 py-1 ",
                               "Adicionar"
                   }/*BUTTON ADICIONAR */
                        }
                    }
                    }
                }
            }
    )
}
