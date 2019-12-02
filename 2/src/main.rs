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
    let mut ops = input
        .trim()
        .split(',')
        .map(|x| x.parse::<usize>().expect("Failed to parse file!"))
        .collect::<Vec<usize>>();
    let mut pc = 0;

    while pc < ops.len() {
        println!("{}", ops[pc]);
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
                println!("break");
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
}
