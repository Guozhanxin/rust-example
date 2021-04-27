
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }
}

fn main() {
    let rectangle = Rectangle {width : 30, height : 50};

    println!("The rectangle:{:?}'s area is {}", rectangle, rectangle.area());
}
