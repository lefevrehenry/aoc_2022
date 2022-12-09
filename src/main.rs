
use std::fs;
use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Copy)]
#[derive(Clone)]
struct Position {
    x: i32,
    y: i32,
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Position {
    fn move_from_direction(&mut self, direction: &Direction) -> () {
        match direction {
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
        };
    }

    // adjacent Position
    fn next_to(&self, other: &Position) -> bool {
        (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1 
    }

    fn move_toward(&mut self, other: &Position) -> () {
        if self.next_to(other) {
            return;             // no need to move
        }

        let dx = other.x - self.x;
        let dy = other.y - self.y;

        if dx.abs() > 0 {
            self.x += if dx > 0 { 1 } else { -1 }
        };
        if dy.abs() > 0 {
            self.y += if dy > 0 { 1 } else { -1 }
        };
    }
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

struct Command {
    direction: Direction,
    count: u8,
}

struct Simulation {
    ropes: [Position;10],
    //head: Position,       // exo 1
    //tail: Position,
    commands: Vec<Command>,
    visited: Vec<Position>,
}

impl Simulation {

    fn execute(&mut self) -> usize {
        let commands = std::mem::take(&mut self.commands);
       
        commands.iter().map(|command| -> () {
            self.do_move(&command);
        }).collect::<()>();

        self.commands = commands;

        self.visited.len()
    }

    fn do_move(&mut self, command: &Command) -> () {
        let repeat = command.count;

        (0..repeat).map(|_| -> () {
            self.ropes[0].move_from_direction(&command.direction);                      // move head
            let slicing_ropes: &mut [Position] = &mut self.ropes[..];                   // move ropes
            for i in 1..self.ropes.len() {
                let previous_knot = self.ropes[i-1].clone();
                let mut knot = &mut self.ropes[i];
                knot.move_toward(&previous_knot);
            }
            if !self.visited.contains(&self.ropes[9]) {
                self.visited.push(self.ropes[9].clone());                               // record visited Position
            };
        }).collect::<()>();
    }
    
    /*  do_move version from exo 1
    fn do_move(&mut self, command: &Command) -> () {
        let repeat = command.count;

        (0..repeat).map(|_| -> () {
            self.head.move_from_direction(&command.direction);                      // move head
            self.tail.move_toward(&self.head);                                      // move tail
            if !self.visited.contains(&self.tail) {
                self.visited.push(self.tail.clone());                               // record visited Position
            };
        }).collect::<()>();
    }
    */
}

impl FromStr for Simulation {
    type Err = ParseIntError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut simu = Simulation {
            ropes: [Position {x:0, y:0}; 10],
            //head: Position { x: 0, y: 0 },
            //tail: Position { x: 0, y: 0 },
            commands: Vec::new(),
            visited: Vec::new(),
        };

        simu.commands = s.lines().map(|line| -> Command {
            let mut split = line.split_whitespace();

            let direction_name = split.next().unwrap();
            let n = split.next().unwrap().parse::<u8>().unwrap();

            let direction = match direction_name {
                "L" => Direction::Left,
                "R" => Direction::Right,
                "U" => Direction::Up,
                "D" => Direction::Down,
                 _ => Direction::Left
            };

            Command {
                direction: direction,
                count: n,
            }
        }).collect();

        Ok(simu)
    }
}

fn main() {
    let contents = fs::read_to_string("./data/input")
        .expect("file not found");

    if let Ok(mut simu) = Simulation::from_str(&contents) {
        let n = simu.execute();

        println!(">>> {}", n);
    }
}

