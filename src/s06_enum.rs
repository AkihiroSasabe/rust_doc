// 列挙型enum
// 構造体よりもenumを使う方がいいことがある
enum IpAddrKind {
    V4,
    V6
}

fn route(ip_type: IpAddrKind) {}


// IpAddrKind ver1
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// IpAddrKind ver2: (1)余計な構造体不要, (2)列挙子が異なる型を紐付けてok
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String)
}
// 列挙子が異なる型を持っていてok
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

// Rustにnullはない！
// nullか、そうでないか
// Rustにおける値が存在するか不在かという概念を表すコード
// enum Option<T> {
//     Some(T),
//     None,
// }
// 初期化処理(prelude)に含まれているので、SomeとNoneをOption::の接頭辞なしに使える

impl Message {
    fn call(&self) {
        println!("tekito");
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // ver1
    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1")
    // };

    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };

    // ver2: 余計な構造体不要
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    
    let some_number = Some(5);
    let some_string = Some(String::from("a string"));
    let absent_number: Option<i32> = None;


}