pub fn read_to_string(path: &str) -> String {
  let input = std::fs::read_to_string(path).unwrap();
  let input = input.trim();
  String::from(input)
}

#[derive(Debug, Clone)]
enum Block {
  File { id: usize },
  Gap,
}

#[derive(Debug, Clone)]
enum Span {
  File { len: u8, id: usize },
  Gap { len: u8 },
}


#[derive(Debug)]
struct DiskMap {
  pub spans: Vec<Span>,
}

impl DiskMap {
  fn blocks(&self) -> Vec<Block> {
    self.spans.iter().flat_map(|span| match span {
      Span::File { len, id } => std::iter::repeat_n(Block::File { id: *id }, (*len).into()),
      Span::Gap { len } => std::iter::repeat_n(Block::Gap, (*len).into()),
    }).collect::<Vec<_>>()
  }
}

fn parse(input: &str) -> DiskMap {
  let mut is_file = true;
  let mut id = 0;
  let spans = input.chars().map(|c| {
    let len = (c as u8) - 48;
    let span = match is_file {
      true => Span::File { len, id },
      false => Span::Gap { len },
    };
    if is_file {
      id += 1;
    }
    is_file = !is_file;
    span
  }).filter(|span| { match span {
    Span::File { len, id: _ } if *len > 0 => true,
    Span::Gap { len } if *len > 0 => true,
    _ => false,
  }}).collect::<Vec<_>>();
  DiskMap { spans }
}

pub fn part1(input: &str) -> usize {
  let disk_map = parse(&input);
  let blocks = disk_map.blocks();
  let file_blocks = blocks.clone().into_iter().map(|block|
    match block {
      Block::File { id } => Some(id),
      _ => None,
    })
    .filter(|maybe_file| maybe_file.is_some())
    .map(|file| file.unwrap())
    .collect::<Vec<_>>();
  let blocks = &blocks[..file_blocks.len()];
  let mut rev_file_blocks = file_blocks.iter().rev();
  // TODO not a fan of all this cloning
  let compacted = blocks.iter().map(|block| { match block {
    Block::File { id } => id.clone(),
    Block::Gap => rev_file_blocks.next().unwrap().clone(),
  }}).collect::<Vec<_>>();
  compacted.into_iter().enumerate().map(|(index, id)| index * id).sum()
}

pub fn part2(input: &str) -> usize {
  let disk_map = parse(&input);
  let mut spans = disk_map.spans;
  let rev_file_spans = spans.clone().into_iter().rev().map(|span|
    match span {
      Span::File { len, id } => Some(Span::File { len, id }),
      _ => None,
    })
    .filter(|maybe_file| maybe_file.is_some())
    .map(|file| file.unwrap())
    .collect::<Vec<_>>();
  for rev_file_span in rev_file_spans {
    let Span::File { len: rev_file_span_len, id: rev_file_span_id } = rev_file_span else { panic!() };
    if let Some(to_position) = spans.iter().position(|span| match span {
      Span::Gap { len } if len >= &rev_file_span_len => true,
      _ => false,
    }) {
      let Span::Gap { len: gap_len } = spans[to_position] else { panic!() };
      let remaining_gap_len = gap_len - rev_file_span_len;
      spans[to_position] = rev_file_span;
      if remaining_gap_len > 0 {
        spans.insert(to_position + 1, Span::Gap { len: remaining_gap_len });
      }
      if let Some(from_position) = spans.iter().rposition(|span| match span {
        Span::File { len: _, id } if *id == rev_file_span_id => true,
        _ => false
      }) {
        spans[from_position] = Span::Gap { len: rev_file_span_len };
      }
    }
  }

  let blocks = DiskMap { spans }.blocks();
  blocks.into_iter().enumerate().map(|(index, block)| match block {
    Block::File { id } => index * id,
    _ => 0,
  }).sum()
}
