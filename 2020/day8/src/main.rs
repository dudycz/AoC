use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

#[derive(Debug)]
struct Instruction {
    op: String,
    arg: i32,
}

fn build_instruction(s: &String) -> Instruction {
    let mut iter = s.split_whitespace();
    match iter.next().unwrap() {
        "nop" => Instruction {op: "nop".to_string(), arg: 0},
        "jmp" => Instruction {op: "jmp".to_string(), arg: iter.next().unwrap().parse::<i32>().unwrap() },
        "acc" => Instruction {op: "acc".to_string(), arg: iter.next().unwrap().parse::<i32>().unwrap() },
        _ => panic!("Unsupported op code"),
    }
}

fn execute_part1(prog: &Vec<Instruction>) -> i32 {
    let mut accumulator = 0;
    let mut executed = HashSet::new();

    let mut pc:i32 = 0;
    while !executed.contains(&pc) {
        executed.insert(pc);
        let instr = &prog[pc as usize];
        match instr.op.as_str() {
            "nop" => pc += 1,
            "acc" => { accumulator += instr.arg; pc += 1},
            "jmp" => pc += instr.arg,
            _ => (),
        }
    }
    accumulator
}

fn main() {
    if let Ok(lines) = read_lines("input") {
        let mut program:Vec<Instruction> = Vec::new();
        for line in lines {
            program.push(build_instruction(&line.unwrap()));
        }

        println!("Part 1: accumulator contains {}", execute_part1(&program));
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}