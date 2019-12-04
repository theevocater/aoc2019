use std::collections::HashMap;
fn validate(i: i32) -> bool {
    let mut prev = i;
    let mut prev_digit = i % 10;
    let mut adjacent = false;
    let mut adjacents: HashMap<i32, i32> = HashMap::new();
    let mut prev_adjacent = prev;
    let mut prev_adjacent_count = 0;
    while prev > 0 {
        let curr = prev / 10;
        let curr_digit = curr % 10;
        if prev_digit < curr_digit {
            return false;
        }
        if prev_digit == curr_digit {
            // we are counting a list of them
            if adjacent == true {
                prev_adjacent_count += 1;
            } else {
                adjacent = true;
                prev_adjacent_count = 2
            }
        } else if adjacent == true {
            adjacent = false;
            adjacents.insert(prev_digit, prev_adjacent_count);
        }
        prev = curr;
        prev_digit = curr_digit;
    }
    return adjacents.values().find(|x| **x == 2).is_some();
}

fn main() {
    let lower = 265275;
    let upper = 781584 + 1;
    let mut total = 0;
    for i in lower..upper {
        if validate(i) {
            println!("{}", i);
            total += 1;
        }
    }
    println!("Total: {}", total);
}
