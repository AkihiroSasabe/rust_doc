use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

// リクエストを読み取る
fn handle_connection(mut stream: TcpStream) {
    let mut buffer =[0; 1024];
    // streamのreadでリクエストの読み取り！
    stream.read(&mut buffer).unwrap();

    // String::from_utf8_lossy関数は、&[u8]を取り、Stringを生成
    // 名前の“lossy”の箇所は、無効なUTF-8シーケンスを目の当たりにした際のこの関数の振る舞いを示唆しています: 
    // 無効なシーケンスを�、U+FFFD REPLACEMENT CHARACTERで置き換えます
    print!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}


// asasabe@asasabe_open3d_210825_user:/raid/data/asasabe/rust_prog_intro/rust_doc/sr
// c$ cargo run --bin s20_1_2_read_request
//    Compiling rust_doc v0.1.0 (/raid/data/asasabe/rust_prog_intro/rust_doc)
//     Finished dev [unoptimized + debuginfo] target(s) in 0.68s
//      Running `/raid/data/asasabe/rust_prog_intro/rust_doc/target/debug/s20_1_2_read_request`

// "Request行"は、クライアントが要求しているものが何なのかを示す
// Request: GET / HTTP/1.1
// "GET"はメソッドで、どのようにクライアントがこの要求を行なっているかを記述する。"POST"もメソッド。
// "/"はURI (=URL = Uniform Resource Identifier = Uniform Resource Locator)
// "HTTP/1.1"はクライアントのHTTPのバージョン

// 残りはヘッダ。GETリクエストには、本体（訳注:message-bodyのこと）がありません。
// Host: 127.0.0.1:7878
// User-Agent: Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:99.0) Gecko/20100101 Firefox/99.0
// Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8
// Accept-Language: en-US,en;q=0.5
// Accept-Encoding: gzip, deflate, br
// Connection: keep-alive
// Cookie: _xsrf=2|8e0e40f3|7e9114c690a4aea4951d9a8855447131|1651756279
// Upgrade-Insecure-Requests: 1
// Sec-Fetch-Dest: document
// Sec-Fetch-Mode: navigate
// Sec-Fetch-Site: none
// Sec-Fetch-User: ?1

// Request: GET / HTTP/1.1
// Host: 127.0.0.1:7878
// User-Agent: Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:99.0) Gecko/20100101 Firefox/99.0
// Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8
// Accept-Language: en-US,en;q=0.5
// Accept-Encoding: gzip, deflate, br
// Connection: keep-alive
// Cookie: _xsrf=2|8e0e40f3|7e9114c690a4aea4951d9a8855447131|1651756279
// Upgrade-Insecure-Requests: 1
// Sec-Fetch-Dest: document
// Sec-Fetch-Mode: navigate
// Sec-Fetch-Site: none
// Sec-Fetch-User: ?1

// Request: GET / HTTP/1.1
// Host: 127.0.0.1:7878
// User-Agent: Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:99.0) Gecko/20100101 Firefox/99.0
// Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8
// Accept-Language: en-US,en;q=0.5
// Accept-Encoding: gzip, deflate, br
// Connection: keep-alive
// Cookie: _xsrf=2|8e0e40f3|7e9114c690a4aea4951d9a8855447131|1651756279
// Upgrade-Insecure-Requests: 1
// Sec-Fetch-Dest: document
// Sec-Fetch-Mode: navigate
// Sec-Fetch-Site: none
// Sec-Fetch-User: ?1

// Request: GET / HTTP/1.1
// Host: 127.0.0.1:7878
// User-Agent: Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:99.0) Gecko/20100101 Firefox/99.0
// Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8
// Accept-Language: en-US,en;q=0.5
// Accept-Encoding: gzip, deflate, br
// Connection: keep-alive
// Cookie: _xsrf=2|8e0e40f3|7e9114c690a4aea4951d9a8855447131|1651756279
// Upgrade-Insecure-Requests: 1
// Sec-Fetch-Dest: document
// Sec-Fetch-Mode: navigate
// Sec-Fetch-Site: none
// Sec-Fetch-User: ?1

// Request: GET / HTTP/1.1
// Host: 127.0.0.1:7878
// User-Agent: Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:99.0) Gecko/20100101 Firefox/99.0
// Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8
// Accept-Language: en-US,en;q=0.5
// Accept-Encoding: gzip, deflate, br
// Connection: keep-alive
// Cookie: _xsrf=2|8e0e40f3|7e9114c690a4aea4951d9a8855447131|1651756279
// Upgrade-Insecure-Requests: 1
// Sec-Fetch-Dest: document
// Sec-Fetch-Mode: navigate
// Sec-Fetch-Site: none
// Sec-Fetch-User: ?1

// Request: GET / HTTP/1.1
// Host: 127.0.0.1:7878
// User-Agent: Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:99.0) Gecko/20100101 Firefox/99.0
// Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8
// Accept-Language: en-US,en;q=0.5
// Accept-Encoding: gzip, deflate, br
// Connection: keep-alive
// Cookie: _xsrf=2|8e0e40f3|7e9114c690a4aea4951d9a8855447131|1651756279
// Upgrade-Insecure-Requests: 1
// Sec-Fetch-Dest: document
// Sec-Fetch-Mode: navigate
// Sec-Fetch-Site: none
// Sec-Fetch-User: ?1

// Request: GET / HTTP/1.1
// Host: 127.0.0.1:7878
// User-Agent: Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:99.0) Gecko/20100101 Firefox/99.0
// Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8
// Accept-Language: en-US,en;q=0.5
// Accept-Encoding: gzip, deflate, br
// Connection: keep-alive
// Cookie: _xsrf=2|8e0e40f3|7e9114c690a4aea4951d9a8855447131|1651756279
// Upgrade-Insecure-Requests: 1
// Sec-Fetch-Dest: document
// Sec-Fetch-Mode: navigate
// Sec-Fetch-Site: none
// Sec-Fetch-User: ?1

// Request: GET / HTTP/1.1
// Host: 127.0.0.1:7878
// User-Agent: Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:99.0) Gecko/20100101 Firefox/99.0
// Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8
// Accept-Language: en-US,en;q=0.5
// Accept-Encoding: gzip, deflate, br
// Connection: keep-alive
// Cookie: _xsrf=2|8e0e40f3|7e9114c690a4aea4951d9a8855447131|1651756279
// Upgrade-Insecure-Requests: 1
// Sec-Fetch-Dest: document
// Sec-Fetch-Mode: navigate
// Sec-Fetch-Site: none
// Sec-Fetch-User: ?1

// Request: GET / HTTP/1.1
// Host: 127.0.0.1:7878
// User-Agent: Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:99.0) Gecko/20100101 Firefox/99.0
// Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8
// Accept-Language: en-US,en;q=0.5
// Accept-Encoding: gzip, deflate, br
// Connection: keep-alive
// Cookie: _xsrf=2|8e0e40f3|7e9114c690a4aea4951d9a8855447131|1651756279
// Upgrade-Insecure-Requests: 1
// Sec-Fetch-Dest: document
// Sec-Fetch-Mode: navigate
// Sec-Fetch-Site: none
// Sec-Fetch-User: ?1

// Request: GET / HTTP/1.1
// Host: 127.0.0.1:7878
// User-Agent: Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:99.0) Gecko/20100101 Firefox/99.0
// Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8
// Accept-Language: en-US,en;q=0.5
// Accept-Encoding: gzip, deflate, br
// Connection: keep-alive
// Cookie: _xsrf=2|8e0e40f3|7e9114c690a4aea4951d9a8855447131|1651756279
// Upgrade-Insecure-Requests: 1
// Sec-Fetch-Dest: document
// Sec-Fetch-Mode: navigate
// Sec-Fetch-Site: none
// Sec-Fetch-User: ?1