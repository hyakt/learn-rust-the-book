fn main() {
    let v1 = vec![1, 2, 3];

    let result: Vec<i32> = v1.iter().map(|x| x + 1).collect();
    println!("result: {:?}", result);
}
