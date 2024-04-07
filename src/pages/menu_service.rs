use dioxus::{hooks::use_signal, signals::WritableVecExt};

use crate::components::item::Item;

use super::Menu;

impl Menu {
    pub fn new() -> Self {
        return Menu {
            items: use_signal(|| vec![]),
        };
    }
    
    pub fn TotalSelectedItensFormated(&mut self) -> String {
        let total:f32 = self.items.iter_mut().map(|mut i| i.Total()).sum();
        return format!("{:.2}", total);
    }

    pub fn LoadItems(&mut self) {
        let  items = Item {
            id :"".to_string(),
            name: use_signal(|| String::from("Hambúrguer")),
            price: 17.99,
            quantity: use_signal(|| 0),
            description: String::from("Hambúrguer delicioso feito com muito amor --->"),
            image:"https://www.estadao.com.br/resizer/dixyOItHmPSgiedCSBL1iIT5lGo=/arc-anglerfish-arc2-prod-estadao/public/GUOGMQ4FRJIUPAWMYLE4WNA3SY.jpg".to_string()
        };
        let itemz = Item {
            id: "".to_string(),
            name: use_signal(|| String::from("Hambúrguer X salada")),
            price: 1.99,
            quantity: use_signal(|| 0),
            description: String::from(
                "Hambúrguer delicioso feito com muito amor e com salada --->",
            ),
            image: "https://th.bing.com/th/id/OIP.paK_UK5Hnf_fGUCkmZ-hFAHaEK?rs=1&pid=ImgDetMain"
                .to_string(),
        };

        self.items = use_signal(|| vec![items, itemz]);
        log::info!("FEZ O LOAD");
    }
}
