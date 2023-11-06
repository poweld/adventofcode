use std::error::Error;

#[derive(Debug)]
enum Command {
    Cd(String),
    Ls,
}

#[derive(Debug)]
enum Item {
    Dir(String),
    File(usize, String),
}


#[derive(Debug)]
enum LineType {
    Command(Command),
    Item(Item),
}

fn parse(input: String) -> Vec<LineType> {
    input.lines()
         .map(|line| line.split_whitespace().collect::<Vec<_>>())
         .map(|split| {
             match split[..] {
                 ["dir", dirname] => LineType::Item(Item::Dir(dirname.to_string())),
                 ["$", "ls"] => LineType::Command(Command::Ls),
                 ["$", "cd", dirname] => LineType::Command(Command::Cd(dirname.to_string())),
                 [size, filename] => LineType::Item(Item::File(size.parse().unwrap(), filename.to_string())),
                 _ => panic!("Invalid line: {}", split.join(" ")),
             }
         })
         .collect()
}

pub fn is_targetable_dir(size: usize) -> bool {
    size <= 100_000
}

#[derive(Debug)]
struct Node {
    name: String,
    children: Vec<Node>,
    size: Option<usize>,
}
impl Node {
    fn new(name: &str) -> Self {
        let name = name.to_string();
        let children = Vec::new();
        let size = None;
        Node { name, children, size }
    }
    fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }
    fn with_size(self, size: usize) -> Self {
        Self { size: Some(size), ..self }
    }
    fn all_dirs(&self) -> Box<dyn Iterator<Item = &Node> + '_> {
        Box::new(
            std::iter::once(self).chain(
                self.children
                    .iter()
                    .filter(|child| !child.children.is_empty())
                    .flat_map(|child| child.all_dirs())
            )
        )
    }
    fn calculate_size(&mut self) -> usize {
        if self.children.is_empty() {
            // file
            self.size.unwrap()
        } else {
            let result = self.children
                .iter_mut()
                .map(|child| child.calculate_size())
                .sum();
            self.size = Some(result);
            result
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution() {
        let result = solve("data/test_input.txt").expect("bad result");
        let solution = 95437.to_string();
        assert_eq!(solution, result)
    }
}

pub fn solve(input_path: &str) -> Result<String, Box<dyn Error>> {
    let input: String = std::fs::read_to_string(input_path)?;

    let output = parse(input);
    let mut node_stack: Vec<Node> = vec![Node::new("/")];
    //dbg!(&node_stack);
    for line in output {
        match line {
            LineType::Command(Command::Cd(dir)) => {
                match &dir[..] {
                    ".." => {
                        let previous = node_stack.pop().expect("couldn't pop last dir off {node_stack}");
                        let cwd = node_stack.last_mut().expect("couldn't get last dir in {node_stack}");
                        cwd.add_child(previous);
                    },
                    "/" => (),
                    _ => {
                        let cwd = node_stack.last_mut().expect("couldn't get last dir in {node_stack}");
                        let child = Node::new(&dir);
                        node_stack.push(child);
                    },
                };
            },
            LineType::Command(Command::Ls) => (),  // Do nothing on lines with ls commands
            LineType::Item(Item::Dir(dir)) => (),  // Do nothing, handle in Cd
            LineType::Item(Item::File(size, name)) => {
                let cwd = node_stack.last_mut().expect("couldn't get last dir in {node_stack}");
                let child = Node::new(&name).with_size(size);
                cwd.add_child(child);
            },
        }
        //dbg!(&node_stack);
    }

    let mut root = node_stack.pop().unwrap();
    while let Some(mut next) = node_stack.pop() {
        next.children.push(root);
        root = next;
    }
    //dbg!(&root);
    root.calculate_size();

    let dirs = root.all_dirs().collect::<Vec<_>>();
    let to_delete = dirs.iter().filter(|dir| is_targetable_dir(dir.size.unwrap()));
    let sum: usize = to_delete.map(|dir| dir.size.unwrap()).sum();

    let result = sum.to_string();
    Ok(result)
}
