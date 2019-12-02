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
        .map(|x| {
            let module_mass = x.parse::<i64>().expect("Couldn't parse line!");
            println!("module mass: {}", module_mass);
            let mut total = module_mass / 3 - 2;
            let mut prev_fuel = total;
            loop {
                println!("fuel: {}", prev_fuel);
                let curr_fuel = prev_fuel / 3 - 2;
                if curr_fuel < 0 {
                    break total;
                }
                prev_fuel = curr_fuel;
                total += curr_fuel;
            }
        })
        .fold(0, |acc, x| {
            println!("total: {}", x);
            acc + x
        });
    println!("sum: {}", sum)
}
