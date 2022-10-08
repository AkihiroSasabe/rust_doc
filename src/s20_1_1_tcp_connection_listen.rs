use std::net::TcpListener;

fn main() {
    // リッスンすべきポートに接続すること = binding to a port.
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // incoming()は一連のストリームを与えるイテレータを返す
    // このforループは各接続を順番に処理
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // 接続が確立しました
        println!("Connection established!!");
    }
}