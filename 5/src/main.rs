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
    let mut input = vec![1];
    let mut output: Vec<i32> = Vec::new();

    while pc < ops.len() {
        let op = ops[pc];
        let a = digit_n(op, 4);
        let b = digit_n(op, 3);
        let c = digit_n(op, 2);
        let opcode = op % 100;
        match opcode {
            1 => {
                // The 3 parameter can never be in immediate mode
                assert!(a == 0);
                let a_pos = ops[pc + 3] as usize;
                let b_pos = get_pos(b, pc + 2, &ops);
                let c_pos = get_pos(c, pc + 1, &ops);
                ops[a_pos] = ops[c_pos] + ops[b_pos];
                pc += 4
            }
            2 => {
                // The 3 parameter can never be in immediate mode
                assert!(a == 0);
                let a_pos = ops[pc + 3] as usize;
                let b_pos = get_pos(b, pc + 2, &ops);
                let c_pos = get_pos(c, pc + 1, &ops);
                ops[a_pos] = ops[c_pos] * ops[b_pos];
                pc += 4
            }
            3 => {
                let pos = get_pos(c, pc + 1, &ops);
                ops[pos] = input.pop().expect("Tried to read input and failed");
                pc += 2
            }
            4 => {
                let pos = get_pos(c, pc + 1, &ops);
                output.push(ops[pos]);
                pc += 2
            }
            99 => {
                break;
            }
            _ => {
                println!("error, unrecognized opcode {}", opcode);
                break;
            }
        }
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
