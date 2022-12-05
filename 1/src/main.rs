
use std::fs;

fn min_calories(summary_calories: &mut [i32;3]) -> &mut [i32] {
    let mut min: i32 = 2147483647;
    let mut index: usize = 0;
    for (i,value) in summary_calories.into_iter().enumerate() {
        if value < &mut min {
            min = *value;
            index = i;
        }
    }
    return &mut summary_calories[index..index+1];
}

fn main() {
    let contents = fs::read_to_string("./data/input_2")
        .expect("file not found");

    let lines = contents.split("\n");

    let mut summary_calories: [i32; 3] = [-1,-1,-1];
    let mut sum = 0;

    for line in lines {
        if line.eq("") {
            let min = min_calories(&mut summary_calories);
            if sum > min[0] {
                min[0] = sum;
                // remplacer sum par min dans le tableau
            }
            sum = 0;
            continue;
        }

        let calories = line.parse::<i32>().unwrap();
        sum += calories;
    }

    println!(">>> {:?}", summary_calories);

    let mut sum = 0;
    for c in summary_calories {
        sum += c;
    }
    println!(">>> {}", sum);
}

