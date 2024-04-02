use dioxus::{core_macro::rsx, dioxus_core::Element};
use dioxus::{prelude::*, web::launch};
#[derive(PartialEq, Clone, Props)]
pub struct Prop {
    pub asd: Item,
}

#[derive(PartialEq, Clone, Props)]
pub struct Item {
    pub name: Signal<String>,
    pub price: f32,
    pub quantity: Signal<i32>,
    pub description: String,
}

pub fn Item(mut props: Prop) -> Element {
    rsx!(
        div { class : "flex mb-4 max-w-md max-h-md border p-4",
            div { class : "mr-4 w-7/12",
                img { src : "https://th.bing.com/th/id/OIP.tAWdQ-CvasZX5IXaADwI8wHaDc?rs=1&pid=ImgDetMain",
                    class : "w-full h-full rounded-md object-cover",
                    } /* IMAGEM */
                }
            div { class : "flex flex-grow flex-col justify-between w-5/12",
                    div { class : "mb-auto",
                            h3 { class : "text-lg font-semibold h-1/5" ,
                                "TESTE H3" /* NOME */
                               },
                                p { /* DESCRIÇÃO */
                                    class : "break-words text-sm h-4/5",
                                            "TESTE DESCRICAOBBAAAAAAAAAAAAAAAAAAAAAAAAAAAAsdfsdfsdfsdfs
                                            AAAAAAAAAAAAAAAAAAAAAAAAAAAAA
                                            AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA 
                                          ",

                                  },
                        },
                        div { class : "flex justify-between mt-2",
                    div { class : "flex",
                           span { class : "text-lg font-semibold",
                                        "R$0.00"
                                } /* PREÇO */
                        },

                    div { class : "flex",
                         button { class : "rounded bg-gray-200 px-2 py-1 text-gray-700",
                                      "+"
                                }/*BUTTON ADICIONAR */
                        }
                        div { class : "flex",
                         button { class : "rounded bg-gray-200 px-2 py-1 text-gray-700",
                         "{props.asd.quantity }"
                                }/*BUTTON ADICIONAR */
                        }

                          button { onclick: move |event| {
                            props.asd.quantity -=1;
                             log::info!("Clicked! Event:")} ,
                            class : "rounded bg-gray-200 px-2 py-1 text-gray-700",
                            "click me!"

                                }/*BUTTON ADICIONAR */

                    }
                }
            }
    )
}
