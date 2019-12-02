use std::env;
use std::fs;
use std::iter;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage {} INPUT", args[0]);
        process::exit(1);
    }
    let input = fs::read_to_string(&args[1]).expect("Unable to read file");
    let f = |x| 0.max((x / 3) - 2);
    let sum = input
        .lines()
        .map(|line| line.parse::<i64>().expect("Couldn't parse line!"))
        .flat_map(|module_mass| {
            iter::successors(Some(f(module_mass)), |&prev| {
                if prev <= 0 {
                    None
                } else {
                    Some(f(prev))
                }
            })
        })
        .fold(0, |acc, x| acc + x);
    println!("sum: {}", sum)
}
