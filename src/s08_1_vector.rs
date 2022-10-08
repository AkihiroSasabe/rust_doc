


fn main() {
    // vectorを新規生成。型の指定が要る
    // let v: Vec<i32> = Vec::new();

    // // マクロで呼び出す(値を含むとき楽)
    // let v = vec![1, 2, 3];

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    let third: Option<&i32> = v.get(2);

    // // 存在しない要素にアクセス
    // // パニック
    // let does_not_exist = &v[100];
    // // Noneを返す(パニックして落とすよりユーザーに優しい)
    // let does_not_exist = v.get(100);

    // ベクタの各要素全て出力
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    // ベクタの各要素の値を変更する
    let mut v = vec![100, 32, 57];
    for i in &mut v{
        // 参照外し演算子(*)が、可変参照が参照している値を変更するのに必要
        *i += 50;
        println!("{}", i);
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    // vectorに異なる型を格納する方法: enum
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];



}