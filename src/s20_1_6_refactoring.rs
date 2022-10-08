// リクエストにバリデーションをかけ、選択的にレスポンスを返す
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

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    // レスポンスのフォーマット：
    // HTTPバージョン：                                                  HTTP/1.1
    // リクエストの結果を要約する数値ステータス・コード：                  200
    // ステータス・コードのテキスト記述を提供する理由句を含む ステータス行  OKフレーズ 
    // ヘッダ                                                           あり
    // ボディ                                                           あり
    let response = format!("{}{}", status_line, contents);

    // streamのwriteでレスポンスの書き込み！
    stream.write(response.as_bytes()).unwrap();
    // flushは待機し、 バイトが全て接続に書き込まれるまでプログラムが継続するのを防ぎます;
    stream.flush().unwrap();
}