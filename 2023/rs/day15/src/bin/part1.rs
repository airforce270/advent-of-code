use day15::part1::process;

fn main() {
    let file = include_str!("../../input.txt");
    match process(file) {
        Ok(r) => println!("Result: {}", r),
        Err(e) => println!("Error: {}", e),
    }
} 
