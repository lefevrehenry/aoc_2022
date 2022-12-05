
use std::fs;

fn find_match_char(s1: &str, s2: &str) -> char {
    for c1 in s1.chars() {
        for c2 in s2.chars() {
            if c1 == c2 {
                return c1;
            }
        }
    };

    panic!("Something went wrong ...")
}

fn main() {
    println!("Hello, world!");

    let contents = fs::read_to_string("./data/test")
        .expect("file not found");

    let lines = contents.lines();

    let mut total: i32 = 0;

    for line in lines {
        if line.eq("") {
            continue;
        }

        let bagpack: (&str, &str) = line.split_at(line.len() / 2);
        let pocket_1: &str = &bagpack.0;
        let pocket_2: &str = &bagpack.1;

        let result: char = find_match_char(pocket_1, pocket_2);

        println!(">>> {}", result);
        println!(">>> {:?}", bagpack);
    }
}
