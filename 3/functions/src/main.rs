fn main() {
    println!("Hello, world!");
    another_function(5, 10);

    // セミコロンの有無で式か文か分ける
    let y = {
        let x = 3;
        x + 1
    };
    println!("{}", y);
    println!("{}", five());
    println!("{}", plus_one(10));
}

fn another_function(x: i32, y: i32) {
    println!("Another function");
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> u32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

// 複数行のコメントは
// スラッシュをつなげて書くしかない
