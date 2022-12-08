
use std::fs;

fn scenic_score(trees: &Vec<Vec<u32>>, i: usize, j: usize, height: u32) -> u32 {
    if i == 0 || i == trees.len()-1 || j == 0 || j == trees[0].len()-1 {
        return 0;
    }

    // scenic score of the current tree
    let mut score = 1;

    // scan up and down direction
    for dx in [-1, 1] { 
        let mut si = i;
        let sj = j;
        let mut score_dir = 0;

        while si > 0 && si < trees.len()-1 {
            si = if dx == -1 { si-1 } else { si+1 };
            score_dir += 1;
            if trees[si][sj] >= height {
                break;
            };
        }

        score *= score_dir;
    }

    // scan left and right direction
    for dy in [-1,1] {
        let si = i;
        let mut sj = j;
        let mut score_dir = 0;

        while sj > 0 && sj < trees[0].len()-1 {
            sj = if dy == -1 { sj-1 } else { sj+1 };
            score_dir += 1;
            if trees[si][sj] >= height {
                break;
            };
        }
        
        score *= score_dir;
    }

    score
}

fn is_visible(trees: &Vec<Vec<u32>>, i: usize, j: usize, height: u32) -> u32 {
    if i == 0 || i == trees.len()-1 || j == 0 || j == trees[0].len()-1 {
        return 1;
    }

    // tell if tree (i,j) is visible in any direction
    let mut tree_visible = false;

    // scan up and down direction
    for dx in [-1, 1] { 
        let mut si = i;
        let sj = j;
        let mut visible = true;

        while si > 0 && si < trees.len()-1 {
            si = if dx == -1 { si-1 } else { si+1 };
            visible &= trees[si][sj] < height;
        }

        tree_visible |= visible;
    }

    // scan left and right direction
    for dy in [-1,1] {
        let si = i;
        let mut sj = j;
        let mut visible = true;

        while sj > 0 && sj < trees[0].len()-1 {
            sj = if dy == -1 { sj-1 } else { sj+1 };
            visible &= trees[si][sj] < height;
        }

        tree_visible |= visible;
    }

    if tree_visible { 1 } else { 0 }
}

fn number_of_trees_visible(trees: &Vec<Vec<u32>>) -> u32 {
    trees.iter().enumerate().map(|(i,line)| -> u32 {
        line.iter().enumerate().map(|(j,height)| -> u32 {
            is_visible(&trees,i,j,*height)
        }).sum()
    }).sum()
}

fn highest_scenic_score(trees: &Vec<Vec<u32>>) -> u32 {
    trees.iter().enumerate().map(|(i,line)| -> u32 {
        line.iter().enumerate().map(|(j,height)| -> u32 {
            scenic_score(&trees,i,j,*height)
        }).max().unwrap()
    }).max().unwrap()
}

fn parse(input: &str) -> Vec<Vec<u32>> {
    input.lines().map(|line: &str| -> Vec<u32> {
        line.chars().map(|c| -> u32 {
            let o: Option<u32> = c.to_digit(10);

            match o {
                Some(d) => d,
                None => 999,
            }
        }).collect::<Vec<u32>>()
    }).collect::<Vec<Vec<u32>>>()
}

fn main() {
    let contents = fs::read_to_string("./data/input")
        .expect("file not found");

    let trees = parse(&contents);

    let n = number_of_trees_visible(&trees);
    let s = highest_scenic_score(&trees);

    println!(">>> {}", n);
    println!(">>> {}", s);
}

