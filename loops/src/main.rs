fn main() {
    // // 無限ループ
    // loop {
    //     println!("again!");
    // }

    // 条件付きループ
    let mut number = 3;
    while number != 0 {
        println!("{}", number);
        number = number - 1;
    }

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("element: {}", element);
    }

    for number in (1..4).rev() {
        println!("number: {}", number);
    }

    println!("fibo: {}", fibo(7));
    println!("c_to_f: {}", c_to_f(11.0));
}

/// fibonacci
fn fibo(x: u32) -> u32 {
    if x == 0 || x == 1 {
        x
    } else {
        fibo(x - 2) + fibo(x - 1)
    }
}

/// 華氏変換
fn c_to_f(c: f32) -> f32 {
    c * 1.8 + 32.0
}
