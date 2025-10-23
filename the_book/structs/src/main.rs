#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // associated function. (does not take self as input)
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    // method. takes self
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width >= rect.width && self.height >= rect.width
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 3,
        height: 5,
    };
    println!("{:#?}", rect1);
    let my_rect = Rectangle::square(10);
    println!("{:#?}", my_rect);
    println!("area is: {}", my_rect.area());
    println!("Hello, world!");
    println!("{}", my_rect.can_hold(&rect1));
    println!("{:#?}", rect1);
}
