
use std::fs;

fn has_duplicates_chars(ss: &[char]) -> bool {

    let find: bool = ss.iter().map(|x| -> bool {
        let count: u8 = ss.iter().map(|y| -> u8 {
            if x == y { 1 } else { 0 }
        }).sum::<u8>();

        if count > 1 { true } else { false }
    }).any(|b| -> bool { b });

    find
}

fn main() {
    let contents = fs::read_to_string("./data/input")
        .expect("file not found");

    contents.lines().map(|line: &str| -> u8 {
        let array: Vec<char> = line.chars().collect();
        let mut indice: usize = 0;

        for i in 14..array.len()+1 {
            let ss: &[char] = &array[i-14..i];
            if !has_duplicates_chars(ss) {
                indice = i;
                break;
            }
        }

        println!("{}", indice);
        0
    }).sum::<u8>();
}
