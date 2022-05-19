
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    let rectangle = Rectangle {
        width: width1,
        height: height1,
    };
    println!("{:#?}", rectangle);

    println!("{:#?}", rectangle.area());

    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!("{:#?}", Rectangle::square(3));



    println!("The area of the rectangle is {} square pixels.", area(&rectangle));
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
    






