#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// method
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }

    //関連関数。関連関数は関数であり、メソッドではありません。
    fn squre(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// implは分けることもできる
impl Rectangle {
    fn hoge(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let width = 30;
    let height = 50;
    let rect = Rectangle { width, height };

    println!("rect: {:#?}", rect);

    println!("area(witdh1, height1): {}", area(&rect));
    println!("rect.area(): {}", rect.area());

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    // rect1にrect2ははまり込む？
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
