

trait Hamburger {
    fn get_description(&self) -> String;
    fn cost(&self) -> f32;
}
struct SimpleHamburger();
impl SimpleHamburger {
    const UNIT_PRICE: f32 = 1.0;
}
impl Hamburger for SimpleHamburger {
    fn get_description(&self) -> String {
        "Simple Hamburger".into()
    }

    fn cost(&self) -> f32 { Self::UNIT_PRICE }
}

trait Decorator: Hamburger {
    fn new(hamburger: Box<dyn Hamburger>) -> Box<dyn Hamburger>;
}

struct FilletDecorator {
    component: Box<dyn Hamburger>
}
impl FilletDecorator {
    const UNIT_PRICE: f32 = 2.0;
}
impl Hamburger for FilletDecorator {
    fn get_description(&self) -> String {
        format!("Fillet and {}", self.component.get_description())
    }

    fn cost(&self) -> f32 {
        self.component.cost() + Self::UNIT_PRICE
    }
}
impl Decorator for FilletDecorator {
    fn new(component: Box<dyn Hamburger>) -> Box<dyn Hamburger> {
        Box::new(Self {  component })
    }
}

struct FruitDecorator {
    component: Box<dyn Hamburger>
}
impl FruitDecorator {
    const UNIT_PRICE: f32 = 3.0;
}
impl Decorator for FruitDecorator {
    fn new(component: Box<dyn Hamburger>) -> Box<dyn Hamburger> {
        Box::new(Self { component })
    }
}

impl Hamburger for FruitDecorator {
    fn get_description(&self) -> String {
        format!("Fruit and {}", self.component.get_description())
    }

    fn cost(&self) -> f32 {
        self.component.cost() + Self::UNIT_PRICE
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let h = Box::new(SimpleHamburger());
        assert_eq!(&h.get_description(), "Simple Hamburger");
        assert_eq!(h.cost(), 1.0);
        let h = FilletDecorator::new(h);
        assert_eq!(&h.get_description(), "Fillet and Simple Hamburger");
        assert_eq!(h.cost(), 3.0);
        let h = FruitDecorator::new(h);
        assert_eq!(&h.get_description(), "Fruit and Fillet and Simple Hamburger");
        assert_eq!(h.cost(), 6.0);
    }
}
