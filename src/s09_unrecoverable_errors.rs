fn main() {
    // マクロでpanic呼出し
    // panic!("crash and burn") // クラッシュ

    // バグでpanic呼び出し: バッファー外読み出し
    let v = vec![1, 2, 3];
    v[99];

}