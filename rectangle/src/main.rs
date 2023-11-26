#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    println!("Hello, world!");
    let rect = Rectangle{
        width: 10,
        height: 20
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

    println!("{}", rect.width)
}

