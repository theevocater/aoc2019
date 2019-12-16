use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::process;

fn sum_edges<'a>(vertex_map: &'a HashMap<&str, Vec<&str>>, current: &str, sum: u32) -> u32 {
    match vertex_map.get(current) {
        // terminal, return current sum + 1
        None => sum,
        // calculate sum for each child + current sum
        Some(vertexes) => vertexes
            .iter()
            .map(|next| sum_edges(vertex_map, next, sum + 1))
            .fold(sum, |acc, i| acc + i),
    }
}

fn min_distance<'a>(vertex_map: &'a HashMap<&str, Vec<&str>>, current: &str, target: &str) -> u32 {
    let mut queue = vec![(current, 0)];
    let mut visited = HashSet::new();
    while !queue.is_empty() {
        // kludgey, but we know the map isn't empty
        let (next, distance) = queue.pop().unwrap();
        if next == target {
            return distance;
        }
        visited.insert(next);
        match vertex_map.get(next) {
            Some(vertexes) => vertexes.iter().for_each(|vertex| {
                if !visited.contains(vertex) {
                    queue.push((vertex, distance + 1));
                }
            }),
            None => {}
        }
    }
    return 0;
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
        match vertex_map.remove(from) {
            None => {
                vertex_map.insert(from, vec![to]);
            }
            Some(mut vertexes) => {
                vertexes.push(to);
                vertex_map.insert(from, vertexes);
            }
        };
        match vertex_map.remove(to) {
            None => {
                vertex_map.insert(to, vec![from]);
            }
            Some(mut vertexes) => {
                vertexes.push(from);
                vertex_map.insert(to, vertexes);
            }
        };
    });
    // no longer works because this graph is no longer a dag
    //println!("{}", sum_edges(&vertex_map, "COM", 0));

    // i'm counting the two extra edges, so subtract them
    println!("{}", min_distance(&vertex_map, "YOU", "SAN") - 2);
}
