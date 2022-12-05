
use std::fs;

const TABLE_POW_2: [u16;9] = [1,2,4,8,16,32,64,128,256];

fn to_bytes(d: u32) -> u128 {
    return 2u128.pow(d);
}

fn into_bytes_representation(section: &str) -> u16 {
    let mut b = 0;

    for c in section.chars() {
        if c == '.' {
            continue;
        }
        let digit = c as usize - '0' as usize;
        b = b | TABLE_POW_2[digit - 1];
    }

    return b;
}

fn one_section_contains_the_other(bytes1: u128, bytes2: u128) -> bool {
    return (bytes1 & bytes2 == bytes1) || (bytes2 & bytes1 == bytes2);
}

fn main() {
    let contents = fs::read_to_string("./data/test")
        .expect("file not found");

    let lines = contents.lines();

    let mut total = 0;
    
    for line in lines {
        let index = line.find(',').unwrap();
        let e1: &str = &line[..index];
        let e2: &str = &line[index+1..];

        let index = e1.find('-').unwrap();
        let s1 = &e1[..index].parse::<u32>().unwrap();
        let s2 = &e1[index+1..].parse::<u32>().unwrap();

        let mut b1 = 0;

        for s in *s1..(*s2+1) {
            b1 = b1 | to_bytes(s - 1);
        }

        let index = e2.find('-').unwrap();
        let s1 = &e2[..index].parse::<u32>().unwrap();
        let s2 = &e2[index+1..].parse::<u32>().unwrap();

        let mut b2 = 0;

        for s in *s1..(*s2+1) {
            b2 = b2 | to_bytes(s - 1);
        }

        //println!(">>> {:#011b}", b1);
        //println!(">>> {:#011b}", b2);
        
        if one_section_contains_the_other(b1, b2) {
            total += 1;
        }
    }

    println!(">>>> {}", total);
    
    /*
    let group_1 = lines.clone().skip(0).step_by(2);     // iterator over even lines
    let group_2 = lines.clone().skip(1).step_by(2);     // iterator over odd lines


    // e1 = first elf, e2 = second elf
    for (e1,e2) in std::iter::zip(group_1,group_2) {
        let (s1,_) = e1.split_at(9);
        let (s2,_) = e2.split_at(9);

        let bytes_s1 = into_bytes_representation(s1);
        let bytes_s2 = into_bytes_representation(s2);
        
        //println!("{} => {:#011b}", s1, bytes_s1);
        //println!("{} => {:#011b}", s2, bytes_s2);

        if one_section_contains_the_other(bytes_s1, bytes_s2) {
            total += 1;
        }
    }

    println!(">>>> {}", total);
    */
}
