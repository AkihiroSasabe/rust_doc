// ハッシュマップはキーの型が自由な辞書。

use std::collections::HashMap;

fn main() {

    // ハッシュマップの新規生成
    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);

    // 2つのベクトルからハッシュマップを生成する方法
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    // collectメソッドをタプルのベクタに使うと、ハッシュマップになる(?)
    // zipメソッドでタプルのベクタを作っている
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();


    // // ハッシュマップと所有権 (何がしたいのかよくわからんw)
    // let field_name = String::from("Favorite color");
    // let field_value = String::from("Blue");
    // let mut map = HashMap::new();
    // map.insert(field_name, field_value);
    
    // ハッシュマップの値にアクセスする
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("{:?}", score); // getで取得するのはSomeなので、{:?}つけないと取得できない

    for (key, value) in &scores {
        println!("{} {}", key, value);
    }

    // ハッシュマップを更新
    // 値を上書き
    scores.insert(String::from("Blue"), 10000); // 10を10000に上書き
    println!("{:?}", scores); // 10ではなく、10000が出力される

    // キーに値が無かった場合にのみ、挿入する
    // entryメソッドの返り値: Entryと呼ばれるenum. 存在or不在を表す
    scores.entry(String::from("Red")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50); // Blueは既に居るので、挿入されない
    println!("{:?}", scores);

    // 古い値に基づいて更新する
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); //そのキーの可変参照を返す
        *count += 1; // *は参照外し
    }

    println!("{:?}", map);

}