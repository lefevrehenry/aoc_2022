
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

fn score(c: char) -> u32 {
    let n: u32 = u32::from(c);
    if n > 96 {
        return n - 96;
    } else {
        return 26 + (n - 64);
    }
}

fn main() {
    println!("Hello, world!");

    let contents = fs::read_to_string("./data/input_1")
        .expect("file not found");

    let lines = contents.lines();

    let mut total: u32 = 0;

    for line in lines {
        if line.eq("") {
            continue;
        }

        let bagpack: (&str, &str) = line.split_at(line.len() / 2);
        let pocket_1: &str = &bagpack.0;
        let pocket_2: &str = &bagpack.1;

        let c: char = find_match_char(pocket_1, pocket_2);
        let n: u32 = score(c);

        /*
        let chars: [char;8] = ['a','b','y','z','A','B','Y','Z'];
        for c in chars { 
            let u: u32 = get_score(c);
            println!(">>> '{}' = {}", c, u);
        }
        */

        total += n;
    }

    println!(">>> {}", total);
}
