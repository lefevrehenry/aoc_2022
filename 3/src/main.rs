
use std::fs;

fn find_match2_char(s1: &str, s2: &str) -> char {
    for c1 in s1.chars() {
        for c2 in s2.chars() {
            if c1 == c2 {
                return c1;
            }
        }
    };

    panic!("Something went wrong ...")
}

fn find_match3_char(s1: &str, s2: &str, s3: &str) -> char {
    for c1 in s1.chars() {
        for c2 in s2.chars() {
            if c1 == c2 {
                for c3 in s3.chars() {
                    if c2 == c3 {
                        return c3;
                    }
                }
            }
        }
    }
    
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

    let contents = fs::read_to_string("./data/input_2")
        .expect("file not found");

    let lines = contents.lines();

    let e1 = lines.clone().skip(0).step_by(3);
    let e2 = lines.clone().skip(1).step_by(3);
    let e3 = lines.clone().skip(2).step_by(3);

    let mut total = 0;

    for ((s1,s2),s3) in std::iter::zip(std::iter::zip(e1,e2), e3) {

        let c: char = find_match3_char(s1,s2,s3);
        let n = score(c);

        total += n;
    }

    println!(">>> {}", total);

    /*
    let mut total: u32 = 0;

    for line in lines {
        if line.eq("") {
            continue;
        }

        let bagpack: (&str, &str) = line.split_at(line.len() / 2);
        let pocket_1: &str = &bagpack.0;
        let pocket_2: &str = &bagpack.1;

        let c: char = find_match2_char(pocket_1, pocket_2);
        let n: u32 = score(c);

        total += n;
    }

    println!(">>> {}", total);
*/
}
