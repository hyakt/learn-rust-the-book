use std::{collections::HashMap, hash::Hash, thread, time::Duration};

struct Cacher<T, U>
where
    T: Fn(U) -> U,
{
    calculation: T,
    value: HashMap<U, U>,
}

impl<T, U> Cacher<T, U>
where
    T: Fn(U) -> U,
    U: Eq + Hash + Copy,
{
    fn new(calculation: T) -> Cacher<T, U> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }
    fn value(&mut self, arg: U) -> U {
        match self.value.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                v
            }
        }
    }
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowy...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| simulated_expensive_calculation(num));

    if intensity < 25 {
        println!(
            // 今日は{}回腕立て伏せをしてください！
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );

        println!(
            // 次に、{}回腹筋をしてください！
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            // 今日は休憩してください！水分補給を忘れずに！
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                // 今日は、{}分間走ってください！
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

fn main() {
    let simulated_user_specified_value = 30;
    let simulated_random_number = 3;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

mod test {
    #[test]
    fn call_with_different_values() {
        let mut c = crate::Cacher::new(|a| a);

        let v1 = c.value(1);
        assert_eq!(v1, 1);
        let v2 = c.value(2);
        assert_eq!(v2, 2);
    }

    #[test]
    fn call_with_string_values() {
        let mut c = crate::Cacher::new(|a| a);
        let v1 = c.value("a");
        assert_eq!(v1, "a");
    }

    #[test]
    fn closure1() {
        let x = 4;
        let equal_to_x = |z| z == x;

        let y = 4;
        assert!(equal_to_x(y));
    }

    #[test]
    fn closure2() {
        let x = vec![1, 2, 3];

        let equal_to_x = move |z| z == x;

        // ここでは、xを使用できません: {:?}
        println!("can't use x here: {:?}", x);

        let y = vec![1, 2, 3];

        assert!(equal_to_x(y));
    }
}
