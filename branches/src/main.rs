fn main() {
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    }

    // ifは式なので、let文の右辺に持ってくることができる
    let number = if true {
        5
    } else {
        10
        // 同じ型でない場合はエラーになる
        // "six" // this is error
    };

    println!("The value of number is: {}", number);
}
