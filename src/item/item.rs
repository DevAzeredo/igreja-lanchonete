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
    rsx!(
        div { class : "flex mb-4 max-w-md max-h-md border p-4",
            div { class : "mr-4 w-7/12",
                img { src : "https://th.bing.com/th/id/OIP.tAWdQ-CvasZX5IXaADwI8wHaDc?rs=1&pid=ImgDetMain",
                    class : "w-full h-full rounded-md object-cover",
                    } /* IMAGEM */
                }
            div { class : "flex flex-grow flex-col justify-between w-5/12",
                    div { class : "mb-auto h-20",
                            h3 { class : "text-lg font-semibold h-1/5" ,
                                "TESTE H3" /* NOME */
                               },
                                p { /* DESCRIÇÃO */
                                    class : "break-words text-sm h-4/5",
                                            "TESTE DESCRICAOBBA
                                            AAAAAAAAAAAAAAAAAAAAAAAAAAAAA
                                            AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA 
                                              AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA 
                                                AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA  
                                                 AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA 
                                                   AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA 
                                                     AAAAAAAAAAAAAAAAAAAAAAAAAAABBBS",

                                  },
                        },
                        div { class : "flex justify-between",
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
                    }
                }
            }
    )
}
