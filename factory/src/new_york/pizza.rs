use crate::pizza::Pizza;

pub enum PizzaType {
    Cheese,
    Fruit
}

pub struct NewYorkCheesePizza {
    name: String
}

impl NewYorkCheesePizza {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into()
        }
    }
    pub fn with_box(name: &str) -> Box<Self> {
        Box::new(Self::new(name))
    }
}

impl Pizza for NewYorkCheesePizza {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn prepare(&self) {
        println!("NewYorkCheesePizza prepare {} ", self.get_name());
    }
    fn bake(&self) {
        println!("NewYorkCheesePizza bake {} ", self.get_name());
    }
    fn cut(&self) {
        println!("NewYorkCheesePizza cut {} ", self.get_name());
    }
}

pub struct NewYorkFruitPizza {
    name: String
}
impl NewYorkFruitPizza {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into()
        }
    }

    pub fn with_box(name: &str) -> Box<Self> {
        Box::new(Self::new(name))
    }
}
impl Pizza for NewYorkFruitPizza {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn prepare(&self) {
        println!("NewYorkFruitPizza prepare {} ", self.get_name());
    }
    fn bake(&self) {
        println!("NewYorkFruitPizza bake {} ", self.get_name());
    }
    fn cut(&self) {
        println!("NewYorkFruitPizza cut {} ", self.get_name());
    }
}