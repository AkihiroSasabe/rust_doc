

fn main() {
    let some_u8_value = Some(0u8);

    // 冗長
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (), // ()はユニット値。何もしない
    };

    // if letは楽(matchへの糖衣構文)
    if let Some(3) = some_u8_value {
        println!("three")
    }

    let mut count = 0;
    match coin {
        Coin::Quater(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    };

    



}


