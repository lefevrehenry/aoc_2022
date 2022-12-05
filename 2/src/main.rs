use std::fs;

fn shape_score(shape: &str) -> i8 {
    if shape.eq("X") {
        return 1;
    } else if shape.eq("Y") {
        return 2;
    } else if shape.eq("Z") {
        return 3;
    }
    
    return -1;
}

fn round_score(shape_1: &str, shape_2: &str) -> i8 {
    if shape_1.eq("X") {        // rock
        if shape_2.eq("A") {
            return 3;
        } else if shape_2.eq("B") {
            return 0;
        } else if shape_2.eq("C") {
            return 6;
        }
    } else if shape_1.eq("Y") { // paper
        if shape_2.eq("A") {
            return 6;
        } else if shape_2.eq("B") {
            return 3;
        } else if shape_2.eq("C") {
            return 0;
        }
    } else if shape_1.eq("Z") {  // scissors
        if shape_2.eq("A") {
            return 0;
        } else if shape_2.eq("B") {
            return 6;
        } else if shape_2.eq("C") {
            return 3;
        }
    }
    panic!("something went wrong ...")
}

fn get_right_shape(what_i_do: &str, enemy_shape: &str) -> String {
    if enemy_shape.eq("A") {        // rock
        if what_i_do.eq("X") {
            return String::from("Z");
        } else if what_i_do.eq("Y") {
            return String::from("X");
        } else if what_i_do.eq("Z") {
            return String::from("Y");
        }
    } else if enemy_shape.eq("B") { // paper
        if what_i_do.eq("X") {
            return String::from("X");
        } else if what_i_do.eq("Y") {
            return String::from("Y");
        } else if what_i_do.eq("Z") {
            return String::from("Z");
        }
    } else if enemy_shape.eq("C") { // scissors
        if what_i_do.eq("X") {
            return String::from("Y");
        } else if what_i_do.eq("Y") {
            return String::from("Z");
        } else if what_i_do.eq("Z") {
            return String::from("X");
        }
    }
    panic!("something went wrong ...")
}

fn main() {

    let contents = fs::read_to_string("./data/input_2")
        .expect("file not found");

    let lines = contents.split("\n");

    let mut total: i32 = 0;

    for line in lines {
        if line.eq("") {
            continue;
        }

        let round = line.split(" ");
        let round: Vec<&str> = round.collect::<Vec<&str>>();

        let enemy_shape = round[0];
        let what_i_do = round[1];
        let my_shape: String = get_right_shape(what_i_do, enemy_shape);
        
        let my_shape_score = shape_score(&my_shape);
        let my_round_score = round_score(&my_shape, enemy_shape);

        let my_score = my_shape_score + my_round_score;

        total += i32::from(my_score);
    }

    println!(">>> {}", total);
}
