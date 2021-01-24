#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    NewYork,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => match state {
            UsState::Alaska => 10,
            UsState::Alabama => 2,
            // default
            _ => 10000,
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    println!(
        "value_in_cents(Coin::Penny): {}",
        value_in_cents(Coin::Penny)
    );
    let al = value_in_cents(Coin::Quarter(UsState::Alabama));
    println!("al: {}", al);

    let five = Some(5);
    let six = plus_one(five);
    eprintln!("six = {:?}", six);

    if let Some(5) = six {
        eprintln!("five = {:?}", five);
    } else {
        eprintln!("other!");
    }
}
