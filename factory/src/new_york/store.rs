use crate::store::PizzaFactory;
use crate::pizza::Pizza;

use super::pizza::{
    PizzaType,
    NewYorkCheesePizza,
    NewYorkFruitPizza
};


pub struct NyPizzaStore();
impl PizzaFactory<PizzaType> for NyPizzaStore {
    fn create_pizza(&self, pizza_type: PizzaType) -> Box<dyn Pizza> {
        match pizza_type {
            PizzaType::Cheese => NewYorkCheesePizza::with_box("Cheese"),
            PizzaType::Fruit => NewYorkFruitPizza::with_box("Fruit"),
        }
    }
}