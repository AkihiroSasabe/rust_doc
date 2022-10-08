use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;

fn main() {
    // let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(ref error) if error.kind() == ErrorKind::NotFound => {
    //         match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => {
    //                 panic!("Tried to create file but there was a problem: {:?}", e);
    //             }
    //         }
    //     },
    //     Err(error)=> {
    //         panic!("There was a problem opening the file {:?}", error);
    //     }
    // };
    
    // unwrap()で冗長なmatch構文を省略できる。。
    // let f = File::open("hello.txt").unwrap();

    // expect()は自分の好きなエラーメッセージを返せる。
    let f = File::open("hello.txt").expect("failed to open file!!!");


}

// エラーの委譲
// エラーが起きたときの対処を、呼び出し元のコードに任せる
// 成功すればStringを保持するOk(String)を返し、
// 失敗したらio::Errorを保持するErr(io::Error)を返す
// fn read_username_from_file() -> Result<String, io::Error> {
//     let f = File::open("hello.txt");

//     let mut f = match f{
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut s = String::new();
//     // ファイルfの中身をsに書き込む
//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }


// エラーの委譲2
// ?演算子で短く書く
// ?演算子はReslutを返す関数でしか使用できない
fn read_username_from_file() -> Result<String, io::Error> {
    // ?演算子はOkなら中身を返し、ErrならErrを返す
    // unwrapメソッドと似ているが、その場でpanicせず呼び出し元にErrを返す点で異なる
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// エラーの委譲2
// ?演算子の連結でさらに短く書く
fn read_username_from_file2() -> Result<String, io::Error> {
    // ?演算子はOkなら中身を返し、ErrならErrを返す
    // unwrapメソッドと似ているが、その場でpanicせず呼び出し元にErrを返す点で異なる
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}


