fn main() {
    let mut s: String = String::from("hello world");
    let word = first_word(&s);
    println!("word: {}", word);
    s.clear();
    // wordはまだ値5を保持しているが、もうこの値を有効に使用できる文字列は存在しない。
    // wordにもう意味ないのだ！

    let s = String::from("hello 世界");
    let hello = &s[..5];
    let world = &s[6..12];

    println!("s.len(): {}", s.len());
    println!("hello: {}", hello);
    println!("world: {}", world);
}

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    return &s[..];
}
