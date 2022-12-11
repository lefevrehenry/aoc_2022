
use std::fs;
use std::str::FromStr;
use std::num::ParseIntError;

// parameter u8 is the number of cycle for each operation
enum Operation {
    Add(u8),
    Noop(u8)
}

struct Command {
    operation: Operation,   // what to do
    count: i32,             // number that increments the register
}

struct Simulation {
    timeline: Vec<i32>,     // keep state of the register along each cycle
    commands: Vec<Command>,
}

impl Simulation {

    fn execute(&mut self) -> i32 {
        self.timeline.push(1);
        let commands = std::mem::take(&mut self.commands);
       
        commands.iter().map(|command| -> () {
            self.execute_command(&command);
        }).for_each(drop);

        self.commands = commands;

        // print the value of the register after each cycle
        //self.timeline.iter().enumerate().map(|(i,r)| -> () {
        //    println!(">>> cycle({}) => {}", i+1, r)
        //}).for_each(drop);
        
        let result = [20,60,100,140,180,220].iter().map(|i| -> i32 {
            let cycle = *i as i32;
            let register = self.timeline[i-1];

            register * cycle
        }).sum::<i32>();

        //if let Some(register) = self.timeline.last() { *register } else { 0 }
        result
    }

    fn execute_command(&mut self, command: &Command) -> () {
        match command.operation {
            Operation::Add(n) => self.build_timeline(n, command.count),
            Operation::Noop(n) => self.build_timeline(n, command.count),
        };
    }

    fn build_timeline(&mut self, number_of_cycles: u8, increment: i32) {
        let register: i32 = *(self.timeline.last().unwrap());

        (0..number_of_cycles).map(|_| -> () {
            self.timeline.push(register);
        }).for_each(drop);

        // increment the register after the last cycle
        if let Some(r) = self.timeline.last_mut() {
            *r += increment;
        }
    }
}

impl FromStr for Simulation {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut simu = Simulation {
            timeline: Vec::new(),
            commands: Vec::new(),
        };

        simu.commands = s.lines().map(|line| -> Command {
            let mut split = line.split_whitespace();

            let operation_name = split.next().unwrap();
            let n = split.next().unwrap_or("0").parse::<i32>().unwrap();

            let operation = match operation_name {
                "addx" => Operation::Add(2),
                _ => Operation::Noop(1),
            };

            Command {
                operation: operation,
                count: n
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

