use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    // panic!("crash and burn");

    // let v = vec![1, 2, 3];

    // v[99];

    // let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("hello.txt") {
    //         Ok(fc) => fc,
    //         Err(e) => {
    //             panic!("Tried to create file but ther was a problem: {:?}", e)
    //         }
    //     },
    //     Err(error) => {
    //         panic!("There was a problem opening the file: {:?}", error)
    //     }
    // };

    // let f = File::open("hello.txt").unwrap();
    // let f = File::open("hello.txt").expect("failed to open hello.txt");

    let f = read_username_from_file().unwrap();
    println!("f: {}", f);
}

fn read_username_from_file() -> Result<String, std::io::Error> {
    // let f = File::open("hello.txt");

    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    // let mut s = String::new();

    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }

    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
