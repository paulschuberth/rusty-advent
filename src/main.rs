mod one;

fn main() {
    day_one();
}

fn day_one() {
    let content = std::fs::read_to_string("input/one/one.txt")
        .expect("Invalid path!");

    println!("Solution Day 1 Part 1: {}", one::solve_part_one(content.as_str()));
    println!("Solution Day 1 Part 2: {}", one::solve_part_two(content.as_str()));
}
