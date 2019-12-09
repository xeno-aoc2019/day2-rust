use std::path::Path;
use std::io::{self, Lines, BufRead, BufReader};
use std::fs::File;

fn main() {
    let expected: usize = 19690720;
    let program = read_program();
    println!("Computing");
    compute(&program, 12, 2);
    for verb in 0..99 {
        for noun in 0..99 {
            let res = compute(&program, verb, noun);
            if res == expected {
                println!("FOUND: {},{} => {}", verb, noun, res);
            }
        }
    }
}

fn read_program() -> Vec<usize> {
    let mut program: Vec<usize> = Vec::new();
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(lv) = line {
                let split = lv.split(",");
                for sv in split {
//                    println!("{}", sv);
                    let v: usize = sv.parse().unwrap();
                    program.push(v);
//                    println!("{}", v);
                }
            }
        }
    }
    return program;
}

fn compute(input_program: &Vec<usize>, noun: usize, verb: usize) -> usize {
    let mut program = input_program.clone();
    let mut ip = 0;
    program[1] = noun;
    program[2] = verb;
    while program[ip] != 99 {
        if program[ip] == 1 {
//            println!("1");
            let a1 = program[ip + 1];
            let a2 = program[ip + 2];
            let a3 = program[ip + 3];
            program[a3] = program[a1] + program[a2];
            ip = ip + 4;
        } else if program[ip] == 2 {
//            println!("2");
            let a1 = program[ip + 1];
            let a2 = program[ip + 2];
            let a3 = program[ip + 3];
            program[a3] = program[a1] * program[a2];
            ip = ip + 4;
        } else {
            println!("ERROR!!!");
            return 0;
        }
    }
//    println!("HALT: {} [0]={}", program[ip], program[0]);
    return program[0];
}


fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

