use crate::pizza::Pizza;

pub enum PizzaType {
    Cheese,
    Fruit
}

pub struct ChicagoCheesePizza {
    name: String
}

impl ChicagoCheesePizza {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into()
        }
    }
    pub fn with_box(name: &str) -> Box<Self> {
        Box::new(Self::new(name))
    }
}

impl Pizza for ChicagoCheesePizza {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn prepare(&self) {
        println!("ChicagoCheesePizza prepare {} ", self.get_name());
    }
    fn bake(&self) {
        println!("ChicagoCheesePizza bake {} ", self.get_name());
    }
    fn cut(&self) {
        println!("ChicagoCheesePizza cut {} ", self.get_name());
    }
}

pub struct ChicagoFruitPizza {
    name: String
}
impl ChicagoFruitPizza {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into()
        }
    }

    pub fn with_box(name: &str) -> Box<Self> {
        Box::new(Self::new(name))
    }
}
impl Pizza for ChicagoFruitPizza {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn prepare(&self) {
        println!("ChicagoFruitPizza prepare {} ", self.get_name());
    }
    fn bake(&self) {
        println!("ChicagoFruitPizza bake {} ", self.get_name());
    }
    fn cut(&self) {
        println!("ChicagoFruitPizza cut {} ", self.get_name());
    }
}