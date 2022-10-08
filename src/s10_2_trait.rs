// トレイト: 他の構造体と共通で使うメソッドを束ねたもの
// あらゆる種類の構造体に対して使える、共通のメソッドを使える。
// pythonだと1クラスに1つのメソッドしか使えなかった。

// Trait名Summary
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String{
        String::from("(Read more...)")
    }
}


pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// デフォルト実装が呼び出される
// impl Summary for NewsArticle {
//     // fn summarize(&self) -> String {
//     //     format!{"{}, by {} ({})", self.headline, self.author, self.location}
//     // }
// }

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// デフォルト実装をオーバーライド
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    // fn summarize(&self) -> String {
    //     format!("{}: {}", self.username, self.content)
    // }
}


// impl Trait構文 (trait境界構文の糖衣構文=略記)
// 任意のトレイトに由来する、あらゆるメソッドを呼び出せる
// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }

// // trait境界構文
// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize())    
// }

// impl trait構文例 (Summaryの異なる型)
// pub fn notify(item1: &impl Summary, item2: &impl Summary){
//     println!("hoge");
// }


// trait境界構文例 (Summaryの同じ型)
pub fn notify<T: Summary>(item1: &T, item2: &T) {
    println!("hoge");
}


fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably know, people"),
        reply: false,
        retweet: false,
    };

    println!("{}", tweet.summarize());

    // let article = NewsArticle {
    //     // ペンギンチームがスタンレーカップチャンピオンシップを勝ち取る！
    //     headline: String::from("Penguins win the Stanley Cup Championship!"),
    //     // アメリカ、ペンシルベニア州、ピッツバーグ
    //     location: String::from("Pittsburgh, PA, USA"),
    //     // アイスバーグ
    //     author: String::from("Iceburgh"),
    //     // ピッツバーグ・ペンギンが再度NHL(National Hockey League)で最強のホッケーチームになった
    //     content: String::from(
    //         "The Pittsburgh Penguins once again are the best \
    //          hockey team in the NHL.",
    //     ),
    // };

    // println!("New article available! {}", article.summarize());
    
    println!("1 new tweet: {}", tweet.summarize());

}