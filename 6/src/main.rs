use std::collections::HashMap;
use std::env;
use std::fs;
use std::process;

fn traverse<'a>(vertex_map: &'a HashMap<&str, Vec<&str>>, current: &str, sum: u32) -> u32 {
    match vertex_map.get(current) {
        // terminal, return current sum + 1
        None => sum,
        // calculate sum for each child + current sum
        Some(vertexes) => vertexes
            .iter()
            .map(|next| traverse(vertex_map, next, sum + 1))
            .fold(sum, |acc, i| acc + i),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage {} INPUT", args[0]);
        process::exit(1);
    }
    let input = fs::read_to_string(&args[1]).expect("Unable to read file");
    let edges = input
        .lines()
        .map(|line| line.split(")").collect::<Vec<&str>>())
        .flat_map(|line| match line.as_slice() {
            [a, b] => Some((*a, *b)),
            _ => None,
        });
    let mut vertex_map = HashMap::<&str, Vec<&str>>::new();
    edges.for_each(|(from, to)| {
        print!("{}-{}", from, to);
        println!();
        match vertex_map.remove(from) {
            None => {
                vertex_map.insert(from, vec![to]);
            }
            Some(mut vertexes) => {
                vertexes.push(to);
                vertex_map.insert(from, vertexes);
            }
        };
    });
    let sum = traverse(&vertex_map, "COM", 0);
    println!("{}", sum);
}
