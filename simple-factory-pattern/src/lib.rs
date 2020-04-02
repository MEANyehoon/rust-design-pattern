enum ShapeType {
    Circle,
    Rectangle,
}

trait Shape {
    fn draw(&mut self);
    fn erase(&mut self);
    fn is_draw(&self) -> bool;
}

struct Circle {
    is_draw: bool
}
impl Circle {
    fn new() -> Self {
        Self { is_draw: false }
    }
}
impl Shape for Circle {
    fn draw(&mut self) {
        println!("draw circle");
        self.is_draw = true;
    }
    fn erase(&mut self) {
        println!("erase circle");
        self.is_draw = false;
    }

    fn is_draw(&self) -> bool {
        self.is_draw
    }
}

struct Rectangle {
    is_draw: bool
}
impl Rectangle {
    fn new() -> Self { Self { is_draw: false } }
}
impl Shape for Rectangle {
    fn draw(&mut self) {
        println!("draw rectangle");
        self.is_draw = true;
    }
    fn erase(&mut self) {
        println!("erase rectangle");
        self.is_draw = false;
    }
    fn is_draw(&self) -> bool {
        self.is_draw
    }
}
struct ShapeFactory;
impl ShapeFactory {
    fn create_shape(shape_type: ShapeType) -> Box<dyn Shape> {
        match shape_type {
            ShapeType::Circle => Box::new(Circle::new()),
            ShapeType::Rectangle => Box::new(Rectangle::new())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut circle = ShapeFactory::create_shape(ShapeType::Circle);
        assert_eq!(circle.is_draw(), false);
        circle.draw(); // draw circle
        assert_eq!(circle.is_draw(), true);
        circle.erase(); // erase circle
        assert_eq!(circle.is_draw(), false);

        let mut rect = ShapeFactory::create_shape(ShapeType::Rectangle);
        assert_eq!(rect.is_draw(), false);
        rect.draw(); // draw rectangle
        assert_eq!(rect.is_draw(), true);
        rect.erase();// erase rectangle
        assert_eq!(rect.is_draw(), false);
    }


}
