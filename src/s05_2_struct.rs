// structはpythonの辞書だと思えばok

// 5-3. 構造体の練習
// 構造体を使うことで、相互の関係性を明示したり、名前による意味付けをする
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}


fn main() {
    let rect1 = Rectangle {
        width: 10,
        height: 30,
    };

    println!("The area of the rectangle is {}", area(&rect1));

    println!("rect1 is {:#?}", rect1);

}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}


