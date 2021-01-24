fn main() {
    let s = String::from("hello"); // sがスコープに入る

    takes_ownership(s); // sの値が関数にムーブされ...
                        // ... ここではもう有効ではない
                        // println!("s: {}", s);

    let x = 5; // xがスコープに入る

    makes_copy(x); // xも関数にムーブされるが、
                   // i32はCopyなので、この後にxを使っても
    println!("x: {}", x); // 大丈夫

    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    //'{}'の長さは、{}です
    println!("The length of '{}' is {}.", s2, len);

    let len = calculate_length2(&s2);

    // '{}'の長さは、{}です
    println!("The length of '{}' is {}.", s2, len);

    let mut s = String::from("hello");

    change(&mut s);

    // 同様の参照はコードブロックの中で一つだけ
    {
        let f1 = &mut s;
        f1.push_str(" hollor")
    }

    let f2 = &s;
    println!("f2: {}", f2);
    let f3 = &s;
    println!("f3: {}", f3);
    let f4 = &mut s;
    f4.push_str(" haaaaai");
    let f5 = &s;
    println!("f5: {}", f5);

    // let reference_to_nothing = dangle();
} // ここでxがスコープを抜け、sもスコープを抜ける。ただし、sの値はムーブされているので、何も特別なことは起こらない。
  //

fn takes_ownership(some_string: String) {
    // some_stringがスコープに入る。
    println!("{}", some_string);
} // ここでsome_stringがスコープを抜け、`drop`が呼ばれる。後ろ盾してたメモリが解放される。
  //

fn makes_copy(some_integer: i32) {
    // some_integerがスコープに入る
    println!("{}", some_integer);
} // ここでsome_integerがスコープを抜ける。何も特別なことはない。

/// 所有権も返す
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len()メソッドは、Stringの長さを返します

    (s, length)
}

fn calculate_length2(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str("string")
}

// fn dangle() -> &String {
//     // dangleはStringへの参照を返す

//     let s = String::from("hello"); // sは新しいString

//     &s // String sへの参照を返す
// } // ここで、sはスコープを抜け、ドロップされる。そのメモリは消される。
//   // 危険だ
