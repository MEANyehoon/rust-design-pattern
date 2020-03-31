use super::pizza::Pizza;

pub trait OrderPizza<PT> {
    fn order_pizza(&self, pizza_type: PT) -> Box<dyn Pizza>;
}
pub trait PizzaFactory<PT> {
    fn create_pizza(&self, pizza_type: PT) -> Box<dyn Pizza>;
}

impl<T, PT> OrderPizza<PT> for T
    where T: PizzaFactory<PT>
{
    fn order_pizza(&self, pizza_type: PT) -> Box<dyn Pizza> {
        let pizza = self.create_pizza(pizza_type);
        pizza.prepare();
        pizza.bake();
        pizza.cut();
        pizza
    }
}