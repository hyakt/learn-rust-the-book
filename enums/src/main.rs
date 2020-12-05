use std::net::IpAddr;
use std::net::Ipv4Addr;

enum SampleIpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("self: {:?}", self);
    }
}

fn main() {
    let home = SampleIpAddr::V4(127, 0, 0, 1);
    // IpAddr::V4(Ipv4Addr(127, 0, 0, 1));

    let m = Message::Write(String::from("hoge"));
    m.call();

    let mut some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    some_number = None
}
