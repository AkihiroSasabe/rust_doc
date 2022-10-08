// リスト20-5: レスポンスの本体としてhello.htmlの中身を送る
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs::File;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer =[0; 1024];
    // streamのreadでリクエストの読み取り！
    stream.read(&mut buffer).unwrap();

    let mut file = File::open("hello.html").unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // String::from_utf8_lossy関数は、&[u8]を取り、Stringを生成
    // 名前の“lossy”の箇所は、無効なUTF-8シーケンスを目の当たりにした際のこの関数の振る舞いを示唆しています: 
    // 無効なシーケンスを�、U+FFFD REPLACEMENT CHARACTERで置き換えます
    // print!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    // レスポンスのフォーマット：
    // HTTPバージョン：                                                  HTTP/1.1
    // リクエストの結果を要約する数値ステータス・コード：                  200
    // ステータス・コードのテキスト記述を提供する理由句を含む ステータス行  OKフレーズ 
    // ヘッダ                                                           あり
    // ボディ                                                           あり
    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

    // streamのwriteでレスポンスの書き込み！
    stream.write(response.as_bytes()).unwrap();
    // flushは待機し、 バイトが全て接続に書き込まれるまでプログラムが継続するのを防ぎます;
    stream.flush().unwrap();
}



