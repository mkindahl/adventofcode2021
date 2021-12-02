use std::num::ParseIntError;
use std::str::FromStr;
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
    path::PathBuf,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/bin/aoc2.dat");
    let file = File::open(path)?;
    let input = BufReader::new(file);
    let program = input
        .lines()
        .map(|line| line?.parse())
        .collect::<Result<Vec<Instr>, MachineError>>()
        .unwrap();
    let mut state = State1 { depth: 0, horiz: 0 };
    state.run(&program);

    println!("Part 1: {}", state.depth * state.horiz);

    let mut state = State2 {
        depth: 0,
        horiz: 0,
        aim: 0,
    };
    state.run(&program);

    println!("Part 2: {}", state.depth * state.horiz);
    Ok(())
}

#[derive(Debug)]
enum Instr {
    Forward(isize),
    Down(isize),
    Up(isize),
}

#[derive(Debug)]
enum MachineError {
    BadInstr(String),
    IoError,
}

impl FromStr for Instr {
    type Err = MachineError;

    fn from_str(text: &str) -> Result<Instr, MachineError> {
        let words: Vec<&str> = text.split_whitespace().collect();
        match words[0] {
            "forward" => Ok(Instr::Forward(words[1].parse()?)),
            "down" => Ok(Instr::Down(words[1].parse()?)),
            "up" => Ok(Instr::Up(words[1].parse()?)),
            _ => Err(MachineError::BadInstr(text.to_string())),
        }
    }
}

impl From<io::Error> for MachineError {
    fn from(_err: io::Error) -> MachineError {
        MachineError::IoError
    }
}

impl From<ParseIntError> for MachineError {
    fn from(err: ParseIntError) -> MachineError {
        MachineError::BadInstr(format!("bad instruction: {}", err))
    }
}

#[derive(Debug)]
struct State1 {
    depth: isize,
    horiz: isize,
}

impl State1 {
    fn execute(&mut self, instr: &Instr) {
        match instr {
            Instr::Forward(n) => self.horiz += n,
            Instr::Down(n) => self.depth += n,
            Instr::Up(n) => self.depth -= n,
        }
    }

    fn run(&mut self, program: &Vec<Instr>) {
        for instr in program {
            self.execute(&instr)
        }
    }
}

#[derive(Debug)]
struct State2 {
    depth: isize,
    horiz: isize,
    aim: isize,
}

impl State2 {
    fn execute(&mut self, instr: &Instr) {
        match instr {
            Instr::Forward(n) => {
                self.horiz += n;
                self.depth += n * self.aim
            }
            Instr::Down(n) => self.aim += n,
            Instr::Up(n) => self.aim -= n,
        }
    }

    fn run(&mut self, program: &Vec<Instr>) {
        for instr in program {
            self.execute(&instr)
        }
    }
}
