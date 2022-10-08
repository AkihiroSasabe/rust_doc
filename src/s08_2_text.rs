// Rustの文字列は難しい
// 1. あらゆるエラーを晒す
// 2. 文字列が複雑なデータ構造
// 3. UTF-8

// Rustで文字列はstrとStringの2種類を指す
// &str:     文字列型は、文字列スライスであるstrだけ。
// String:  ただの標準ライブラリ(伸長可能、可変、所有権があってUTF-8エンコードされている)

fn main() {

    // // 新規文字列を生成
    // let mut s = String::new();

    // to_stringメソッドとString::from関数は、全く同じ
    // (1) to_string メソッドを使って、文字列リテラルからStringを生成
    // let data = "initial contents";
    // let s = data.to_string();
    // 上2行を1行で書く
    let s = "initial contents".to_string();
    println!("{}", s);

    // (2) String::from関数を使って文字列リテラルからStringを作る
    let s = String::from("initial contents");
    println!("{}", s);

    // push_strとpushで文字列に追加
    // (A)push_strメソッド
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("{}", s1);
    // push_str()は所有権を奪わない
    println!("{}", s2);

    // (B) pushメソッド
    // 引数は1文字だけ
    let mut s = String::from("lo");
    s.push('l'); // なぜかダブルクオーテーション"l"にすると落ちる
    println!("{}", s);

    // +演算子
    // fn add(self, s: &str) -> String {}と同じ
    let s1 = String::from("Hello, ");
    let s2 = String::from("World");
    let s3 = s1 + &s2; // s1は所有権を失う
    println!("{}", s3);

    // format!マクロで連結
    // 使い方はprintln!マクロと同じ
    // 第一引数の所有権も失わない
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toc");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{} {}", s, s1);

    let len = String::from("Hola").len();
    println!("{}", len); // 4が出力

    let len = String::from("Здравс").len();
    println!("{}", len); // 12が出力(6ではなく。String::fromがUTF-8に変換するから)

    // (1)バイト、(2)スカラー値、(3)書記素クラスタ
    // ヒンドウ語: “नमस्ते”
    // (1) u8値のベクタ(18バイト): [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
    // (2) Unicodeスカラー値(Rustのchar型)のベクタ(6つのchar値): ['न', 'म', 'स', '्', 'त', 'े']
    // (3) 書記素クラスタ(人間が文字と呼ぶものに一番近い): ["न", "म", "स्", "ते"]

    // 文字列スライスが欲しいときは[i..j]のように、範囲で指定する([i]で数字1つでアクセスしては駄目)
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{}", s); // Здが出力

    // charメソッド: Unicodeスカラー値単位でアクセスできる
    for c in hello.chars() {
        println!("{}", c); // 2個目の出力はд
    }

    // bytesメソッド: 各バイトをそのまま返す
    for b in hello.bytes() {
        println!("{}", b); // 1個目は151
    }



}