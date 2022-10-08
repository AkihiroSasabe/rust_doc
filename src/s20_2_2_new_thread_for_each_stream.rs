// 各リクエストに対してスレッドを立ち上げる
use std::thread;
use std::time::Duration;
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs::File;

fn main() {
    // リッスンすべきポートに接続すること = binding to a port.
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // thread::spawnは新しいスレッドを生成し、それからクロージャ内のコードを新しいスレッドで実行します
        thread::spawn(|| {handle_connection(stream)});
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer =[0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(10));
        ("HTTP/1.1 200 OK\r\n\r\n", "sleep.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    // flushは待機し、 バイトが全て接続に書き込まれるまでプログラムが継続するのを防ぎます;
    stream.flush().unwrap();
}