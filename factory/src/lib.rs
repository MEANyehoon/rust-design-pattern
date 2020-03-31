
mod pizza;
mod store;
mod new_york;
mod chicago;

use store::OrderPizza;
use new_york:: {
    NyPizzaStore,
    PizzaType as NyPizzaType
};
use chicago:: {
    ChicagoPizzaStore,
    PizzaType as ChicagoPizzaType
};


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_something() {
        let ny_pizza_store = NyPizzaStore();
        ny_pizza_store.order_pizza(NyPizzaType::Cheese);
        ny_pizza_store.order_pizza(NyPizzaType::Fruit);

        let chicago_pizza_store = ChicagoPizzaStore();
        chicago_pizza_store.order_pizza(ChicagoPizzaType::Cheese);
        chicago_pizza_store.order_pizza(ChicagoPizzaType::Fruit);

        assert_eq!(1, 2);
    }
}






