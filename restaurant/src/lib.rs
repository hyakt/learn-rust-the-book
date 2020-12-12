mod front_of_house;

fn serve_order() {}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // enumの値は全て公開される
    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}

pub fn eat_at_restarurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::add_to_waitlist();

    // order a breakfast in the summer with rye toast
    let mut meal = back_of_house::Breakfast::summer("rye");

    // やっぱり別のパンにする
    meal.toast = String::from("Wheat");
    println!("i'd like {} toast please", meal.toast);

    // アクセス権がないのでコンパイルエラー
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;

    hosting::add_to_waitlist();
}

// useでモジュールをスコープに持ち込む // 再公開
pub use crate::front_of_house::hosting;
