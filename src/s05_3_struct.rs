// 5-3. メソッド記法
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}


fn main() {
    let rect1 = Rectangle {
        width: 10,
        height: 30,
    };

    println!("The area of the rectangle is {}", rect1.area());

    println!("rect1 is {:#?}", rect1);

}



