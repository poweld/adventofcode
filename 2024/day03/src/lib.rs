use regex::Regex;

pub fn read_to_string(path: &str) -> String {
  let input = std::fs::read_to_string(path).unwrap();
  let input = input.trim();
  String::from(input)
}

fn parse(input: &str) -> Vec<Vec<u32>> {
  let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
  input.lines()
    .flat_map(|line| {
      re.captures_iter(&line)
        .map(|capture| capture.extract::<2>())
        .map(|(_, captured)| {
          captured.iter()
            .map(|c| c.parse().unwrap())
            .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>()
}

pub fn part1(input: &str) -> u32 {
  let data = parse(&input);
  data.into_iter()
    .map(|parts| parts[0] * parts[1])
    .sum()
}

static DONT_VAL: &str = "don't()";
static DONT_LEN: usize = DONT_VAL.len();
static DO_VAL: &str = "do()";
static DO_LEN: usize = DO_VAL.len();

pub fn part2(input: &str) -> u32 {
  let mut input = String::from(input);
  loop {
    // Find a do() _after_ a don't()
    if let Some(idont) = input.find(DONT_VAL) {
      if let Some(ido) = input[idont + DONT_LEN..].find(DO_VAL) {
        let ido = ido + idont + DONT_LEN;
        // Remove the markers and everything between them
        input = [&input[..idont], &input[ido + DO_LEN..]].join("");
      } else {
        // Only a don't() left, ignore everything after it
        input = String::from(&input[..idont]);
      }
    } else {
      break;
    }
  }
  part1(&input)
}
