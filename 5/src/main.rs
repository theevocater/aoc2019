use std::env;
use std::fs;
use std::process;

fn digit_n(x: i32, n: u32) -> i32 {
    let mut num = x;
    for _ in 0..n {
        num = num / 10;
    }
    return num % 10;
}

fn get_pos(mode: i32, pc: usize, ops: &Vec<i32>) -> usize {
    if mode == 1 {
        // immediate mode
        pc
    } else {
        // position mode
        ops[pc] as usize
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage {} INPUT", args[0]);
        process::exit(1);
    }
    let input = fs::read_to_string(&args[1]).expect("Unable to read file");
    let original_ops = input
        .trim()
        .split(',')
        .map(|x| x.parse::<i32>().expect("Failed to parse file!"))
        .collect::<Vec<i32>>();

    let mut ops = original_ops.clone();

    let mut pc = 0;
    let mut input = vec![5];
    println!(
        "{}",
        input
            .iter()
            .fold("".to_string(), |acc, x| format!("{},{}", acc, x))
    );
    let mut output: Vec<i32> = Vec::new();

    while pc < ops.len() {
        let op = ops[pc];
        let opcode = op % 100;
        let param1_mode = digit_n(op, 2);
        let param2_mode = digit_n(op, 3);
        let param3_mode = digit_n(op, 4);
        match opcode {
            1 => {
                // add param1 * param2 store at param3
                // The 3 parameter can never be in immediate mode
                assert!(param3_mode == 0);
                let param1 = get_pos(param1_mode, pc + 1, &ops);
                let param2 = get_pos(param2_mode, pc + 2, &ops);
                let param3 = ops[pc + 3] as usize;
                ops[param3] = ops[param1] + ops[param2];
                pc += 4
            }
            2 => {
                // multiply param1 * param2 store at param3
                // The 3 parameter can never be in immediate mode
                assert!(param3_mode == 0);
                let param1 = get_pos(param1_mode, pc + 1, &ops);
                let param2 = get_pos(param2_mode, pc + 2, &ops);
                let param3 = ops[pc + 3] as usize;
                ops[param3] = ops[param1] * ops[param2];
                pc += 4
            }
            3 => {
                // input store param1
                let param1 = get_pos(param1_mode, pc + 1, &ops);
                ops[param1] = input.pop().expect("Tried to read input and failed");
                pc += 2
            }
            4 => {
                // output from param1
                let param1 = get_pos(param1_mode, pc + 1, &ops);
                output.push(ops[param1]);
                pc += 2
            }
            5 => {
                // jump-if-true if param1 != 0, pc = param2, else continue
                let param1 = get_pos(param1_mode, pc + 1, &ops);
                let param2 = get_pos(param2_mode, pc + 2, &ops);
                if ops[param1] != 0 {
                    pc = ops[param2] as usize
                } else {
                    pc += 3
                }
            }
            6 => {
                // jump-if-false if param1 == 0, pc = param2, else continue
                let param1 = get_pos(param1_mode, pc + 1, &ops);
                let param2 = get_pos(param2_mode, pc + 2, &ops);
                if ops[param1] == 0 {
                    pc = ops[param2] as usize
                } else {
                    pc += 3
                }
            }
            7 => {
                // less than if param1 < param2, ops[param3] = 1 else ops[param3] = 0
                assert!(param3_mode == 0);
                let param1 = get_pos(param1_mode, pc + 1, &ops);
                let param2 = get_pos(param2_mode, pc + 2, &ops);
                let param3 = ops[pc + 3] as usize;
                if ops[param1] < ops[param2] {
                    ops[param3] = 1
                } else {
                    ops[param3] = 0
                }
                pc += 4
            }
            8 => {
                // equal to if param1 == param2, ops[param3] = 1 else ops[param3] = 0
                assert!(param3_mode == 0);
                let param1 = get_pos(param1_mode, pc + 1, &ops);
                let param2 = get_pos(param2_mode, pc + 2, &ops);
                let param3 = ops[pc + 3] as usize;
                if ops[param1] == ops[param2] {
                    ops[param3] = 1
                } else {
                    ops[param3] = 0
                }
                pc += 4
            }
            99 => {
                break;
            }
            _ => {
                println!("error, unrecognized opcode {}", opcode);
                break;
            }
        }
        println!(
            "{}",
            ops[pc..ops.len()]
                .iter()
                .fold("".to_string(), |acc, x| format!("{},{}", acc, x))
        );
    }
    println!(
        "{}",
        ops.iter()
            .fold("".to_string(), |acc, x| format!("{},{}", acc, x))
    );
    println!(
        "{}",
        output
            .iter()
            .fold("".to_string(), |acc, x| format!("{},{}", acc, x))
    );
}
