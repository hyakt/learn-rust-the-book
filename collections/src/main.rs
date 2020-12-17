use std::collections::HashMap;

fn main() {
    // let v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3];

    // ここでvectorの最初の要素を不変の状態で借用すると
    // vectorの作りから、終端への要素追加を行う push 関数を呼べなくなる
    // let first = &v[0];
    v.push(5);

    for i in &v {
        println!("i: {}", i);
    }

    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("i: {}", i);
    }

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    eprintln!("row = {:?}", row);

    let mut s = String::from("foo");
    s.push('c');
    s.push_str("string");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1はムーブされ、もう使用できないことに注意

    // println!("s1: {}", s1);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s: {}", s);

    // Rustで文字を得るのにStringに添え字アクセスすることが許されない最後の理由は、 添え字アクセスという処理が常に定数時間(O(1))になると期待されるからです。 しかし、Stringでそのパフォーマンスを保証することはできません。
    // let hello = "Здравствуйте";
    // let answer = &hello[0];
    let s1 = String::from("😂");
    println!("s1.len(): {}", s1.len());

    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("s: {}", s);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    eprintln!(" = {:?}", scores);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    // let mut scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    scores.insert(String::from("key"), 10);
    eprintln!("scores = {:?}", scores);

    scores.entry(String::from("key")).or_insert(10000000);
    scores.entry(String::from("bar")).or_insert(10000000);
    eprintln!("scores = {:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

    let mut num = vec![1, 10, 100, 2000, 3, 2, 2];

    let mut hash = HashMap::new();

    println!("mean: {}", num.iter().sum::<usize>() / num.len());
    num.sort();
    println!("median: {}", num[num.len() / 2]);

    num.iter().for_each(|v| {
        let count = hash.entry(v).or_insert(0);
        *count += 1;
    });
    eprintln!("hash = {:?}", hash);

    let mut mode = num[0];
}
