use std::env;
use std::fs;
use std::process;

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
        .map(|x| x.parse::<usize>().expect("Failed to parse file!"))
        .collect::<Vec<usize>>();

    let mut noun = 0;
    let mut verb = 0;
    println!("noun {}, verb {}", noun, verb);
    loop {
        let mut ops = original_ops.clone();
        ops[1] = noun;
        ops[2] = verb;
        let mut pc = 0;

        while pc < ops.len() {
            match ops[pc] {
                1 => {
                    let to = ops[pc + 3];
                    ops[to] = ops[ops[pc + 1]] + ops[ops[pc + 2]];
                    pc += 4
                }
                2 => {
                    let to = ops[pc + 3];
                    ops[to] = ops[ops[pc + 1]] * ops[ops[pc + 2]];
                    pc += 4
                }
                99 => {
                    break;
                }
                _ => {
                    println!("error");
                    break;
                }
            }
        }
        println!(
            "{}",
            ops.iter()
                .fold("".to_string(), |acc, x| format!("{},{}", acc, x))
        );
        if ops[0] == 19690720 {
            println!("{}", 100 * ops[1] + ops[2]);
            break;
        } else {
            noun += 1;
            if noun > 99 {
                noun = 0;
                verb += 1;
            }
        }
    }
}
