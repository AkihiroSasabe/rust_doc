
fn main() {
    // 関数
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("the largest number is {result}");


    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest(&number_list);
    println!("the largest number is {result}");

    // 構造体
    let integer = Point {x: 5, y: 10};
    let float = Point {x: 5.0, y: 10.1};
    let mix = Point2 {x: 5, y: 10.1};
    println!("{:?}", integer);
    println!("{:?}", float);
    // println!("{:?}", mix);

    // メソッド
    let p = Point {x: 5, y: 10};
    println!("p.x = {}", p.x());

    let p = Point {x: 5.0, y: 10.0};
    println!("distance = {}", p.distance_from_origin());

    let p1 = Point2 {x:5, y:10.4};
    let p2 = Point2 {x: "Hello", y: 'c'};
    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y)

}


fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest{
            largest = item;
        }
    }
    largest
}


// ジェネリックな型引数の定義
// 関数名の後ろに<>でくくったTをつける
// このコードは比較トレイとstd::cmp::PartialOrdによりエラーを吐く
// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest{
//             largest = item;
//         }
//     }
//     largest
// }


// ジェネリックな構造体定義
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// 複数の型を定義できる
#[derive(Debug)]
struct Point2<T, U> {
    x: T,
    y: U,
}

// ジェネリックなenum
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

// ジェネリックなメソッド定義
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// 特定の型にだけメソッドを追加
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<T, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y
        }
    }
}