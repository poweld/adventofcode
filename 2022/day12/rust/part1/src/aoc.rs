use std::error::Error;

const lowercase: &str = "abcdefghijklmnopqrstuvwxyz";
fn char_to_height<T>(c: char) -> u8 {
    (c as u8) - ('a' as u8)
}

#[derive(Clone, Copy)]
struct Coord {
    x: usize,
    y: usize,
}
impl Coord {
    fn up(&self) -> Self {
        todo!()
        //Self { x: self.x + 1
    }
}

struct HeightMap<T> {
    data: Vec<Vec<T>>,
}
impl<T> HeightMap<T> {
    fn get(&self, coord: &Coord) -> Option<T> {
        self.data.get(coord.x)?.get(coord.y)
    }
    fn from(input: String) -> Self;
}
impl<u8> HeightMap<u8> {
    fn from(input: String) -> HeightMap<u8> {
        let data = input.lines()
            .map(|line| line.chars())
            .map(|chars| {
                chars.map(|c| char_to_height(c))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        Self { data }
    }
}

pub fn solve(input_path: &str) -> Result<String, Box<dyn Error>> {
    let input: String = std::fs::read_to_string(input_path)?;
    todo!()
    // Ok(result)
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve() {
        let result = solve("data/test_input.txt").expect("bad result");
        let solution = 31.to_string();
        assert_eq!(result, solution)
    }

    #[test]
    fn test_char_to_height() {
        assert_eq!(char_to_height('a'), 0);
        assert_eq!(char_to_height('z'), 25);
    }

    #[test]
    fn test_parse() {
        let input = "abc
xyz".to_string();
        let height_map = HeightMap::from<u8>
        assert_eq!(parse::<u8>(input).data, vec![vec![0, 1, 2], vec![23, 24, 25]]);
    }
}
