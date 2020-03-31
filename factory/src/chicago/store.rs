use crate::store::PizzaFactory;
use crate::pizza::Pizza;

use super::pizza::{
    PizzaType,
    ChicagoCheesePizza,
    ChicagoFruitPizza
};


pub struct ChicagoPizzaStore();
impl PizzaFactory<PizzaType> for ChicagoPizzaStore {
    fn create_pizza(&self, pizza_type: PizzaType) -> Box<dyn Pizza> {
        match pizza_type {
            PizzaType::Cheese => ChicagoCheesePizza::with_box("Chicago Cheese"),
            PizzaType::Fruit => ChicagoFruitPizza::with_box("Chicago Fruit"),
        }
    }
}