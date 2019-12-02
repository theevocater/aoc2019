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
    let sum = input
        .lines()
        .map(|x| (x.parse::<u64>().expect("Couldn't parse line!")) / 3 - 2)
        .fold(0, |acc, x| acc + x);
    println!("sum: {}", sum)
}
