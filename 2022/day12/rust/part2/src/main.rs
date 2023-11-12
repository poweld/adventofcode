use std::error::Error;

mod aoc;

fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", aoc::solve("data/input.txt").expect("bad result"));
    Ok(())
}
