use super::Item;

impl Item {
    pub fn Total(&mut self) -> f32 {
        let total = self.price * ((self.quantity)()) as f32;
        return (total * 100.0).round() / 100.0;
    }
}
