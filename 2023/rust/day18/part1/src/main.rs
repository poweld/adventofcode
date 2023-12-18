use std::error::Error;

mod aoc;

fn main() -> Result<(), Box<dyn Error>> {
    let result = aoc::solve("data/input.txt", 1, 226);
    println!("{result}");
    Ok(())
}
