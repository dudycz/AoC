use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

#[derive(Debug,Clone)]
struct Instruction {
    op: String,
    arg: i32,
}

impl Instruction {
    fn from_str(s: &str) -> Self {
        let mut iter = s.split_whitespace();
        Self {
            op: iter.next().unwrap().into(),
            arg: iter.next().unwrap().parse().unwrap()
        }
    }
}

fn execute_program(prog: &Vec<Instruction>) -> (i32, bool) {
    let mut accumulator = 0;
    let mut executed = HashSet::new();

    let mut pc:i32 = 0;
    let mut reached_end = false;
    while !executed.contains(&pc) {
        executed.insert(pc);
        let instr = &prog[pc as usize];
        match instr.op.as_str() {
            "nop" => pc += 1,
            "acc" => { accumulator += instr.arg; pc += 1},
            "jmp" => pc += instr.arg,
            "end" => {reached_end = true; break},
            _ => unimplemented!(),
        }
    }
    (accumulator, reached_end)
}

fn fix_endless_loop(prog: &Vec<Instruction>) -> i32 {
    let mut ret = 0;
    let mut fixed_prog = Vec::new();
    for (i,instr) in prog.iter().enumerate() {
        match instr.op.as_str() {
            "nop" => {fixed_prog = prog.clone(); fixed_prog[i].op = String::from("jmp")},
            "jmp" => {fixed_prog = prog.clone(); fixed_prog[i].op = String::from("nop")},
            _ => continue,
        }

        let (acc, finished) = execute_program(&fixed_prog);

        if finished {
            ret = acc;
            break;
        }
    }
    ret
}

fn main() {
    if let Ok(lines) = read_lines("input") {
        let mut program:Vec<Instruction> = Vec::new();
        for line in lines {
            program.push(Instruction::from_str(&line.unwrap()));
        }
        program.push(Instruction::from_str("end 0"));

        println!("Part 1: accumulator contains {}", execute_program(&program).0);
        println!("Part 2: accumulator after fix contains {}", fix_endless_loop(&program));

    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}