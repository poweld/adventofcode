use std::error::Error;

mod aoc;

fn main() -> Result<(), Box<dyn Error>> {
    let result = aoc::solve("data/input.txt").expect("bad result");
    println!("{result}");
    Ok(())
}
