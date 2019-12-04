use std::env;
use std::fs;
use std::process;

fn print_matrix(matrix: &Vec<Vec<i32>>) {
    for row in matrix {
        for c in row {
            print!("{}", c);
        }
        println!("");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage {} INPUT", args[0]);
        process::exit(1);
    }

    let mut grids: Vec<Box<Vec<Box<Vec<i32>>>>> = vec![];

    let input = fs::read_to_string(&args[1]).expect("Unable to read file");
    let matrix = 30000 as usize;
    for line in input.lines() {
        let mut wire_grid = Box::new(vec![Box::new(vec![0; matrix]); matrix]);
        //print_matrix(&wire_grid);
        let mut r: usize = matrix / 2 - 1;
        let mut c: usize = matrix / 2 - 1;
        for d in line.split(',') {
            let direction = d.chars().nth(0).expect("Needed One char");
            let distance = *&d[1..].parse::<usize>().expect("Failed to parse distance!!");
            print!("{} {}, ", direction, distance);
            match direction {
                'R' => {
                    for i in 0..distance {
                        wire_grid[r][c + i] = 1;
                    }
                    c += distance
                }
                'L' => {
                    for i in 0..distance {
                        wire_grid[r][c - i] = 1;
                    }
                    c -= distance
                }
                'D' => {
                    for i in 0..distance {
                        wire_grid[r + i][c] = 1;
                    }
                    r += distance
                }
                'U' => {
                    for i in 0..distance {
                        wire_grid[r - i][c] = 1;
                    }
                    r -= distance
                }
                _ => print!("error"),
            };
            println!("{} {}", r, c);
        }
        grids.push(wire_grid);
        println!();
    }

    println!("adding matrixes {}", grids.len());
    let mut wire_grid = Box::new(vec![Box::new(vec![0; matrix]); matrix]);
    for grid in grids {
        let mut r = 0 as usize;
        while r < matrix {
            let mut c = 0 as usize;
            while c < matrix {
                wire_grid[r][c] += grid[r][c];
                c += 1;
            }
            r += 1;
        }
    }

    let center_r = (matrix / 2 - 1) as i32;
    let center_c = (matrix / 2 - 1) as i32;
    // now find overlaps and calculate manhattan distance
    let mut r = 0 as usize;
    while r < matrix {
        let mut c = 0 as usize;
        while c < matrix {
            if wire_grid[r][c] == 2 && !(center_r == r as i32 && center_c == c as i32) {
                let manhattan_distance = (center_r - r as i32).abs() + (center_c - c as i32).abs();
                println!("{} {}  = {} ", r, c, manhattan_distance);
            }
            c += 1;
        }
        r += 1;
    }
}
