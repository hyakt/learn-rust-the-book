fn main() {
    // 変数
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // 定数
    const MAX_POINTS: u32 = 100_000;
    println!("The const value is: {}", MAX_POINTS);

    // シャドーイング
    let y = 5;
    let y = y + 1;
    println!("The shadowing value {}", y);
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The shadowing value {}", spaces);

    // データ型
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{}", guess);

    // スカラ型
    //// 整数型
    let x: i32 = -32;
    let x: f32 = 3.0;
    let x: f64 = 3.0;

    //// 浮動小数点型
    let y: f32 = 3.0;

    //// 数値演算
    let sum = 5 + 10;
    let difference = 95.5 - 3.0;
    let product = 4 * 3;
    let quotient = 56 / 3;
    let remainder = 42 % 3;

    //// 論理値型
    let t = true;
    let f: bool = false;

    //// 文字型
    let c = 'z';
    let heart_eyed_cat: char = '😻';

    // 複合型
    //// タプル型
    let tup: (i32, f64, u8) = (500, 6.3, 255);
    let (x, y, z) = tup;
    println!("The value of x, y, z is: {} {} {}", x, y, z);
    let five_hundred = tup.0;

    //// 配列型(固定長)
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
}
