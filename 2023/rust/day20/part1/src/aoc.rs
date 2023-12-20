use std::collections::HashMap;

fn connect(module_a: &mut impl Module, module_b: &mut impl Module) {
    module_a.add_connection_to(&module_b);
    module_b.add_connection_from(&module_a);
}

#[derive(Debug)]
trait Module {
    fn handle_pulse(&mut self, pulse: &Pulse);
    fn low_pulses_received(&self) -> u64;
    fn high_pulses_emitted(&self) -> u64;
    fn add_connection_to(&mut self, other: &dyn Module) {};
    fn add_connection_from(&mut self, other: &dyn Module) {};
}

#[derive(Debug)]
struct PulseCounter {
    low: u64,
    high: u64,
}
impl PulseCounter {
    fn new() -> Self { PulseCounter { low: 0, high: 0 } }
}
impl Module for PulseCounter {
    fn handle_pulse(&mut self, pulse: &Pulse) {
        match pulse {
            &Pulse::Low(_) => self.low += 1,
            &Pulse::High(_) => self.high += 1,
        }
    }
    fn low_pulses_received(&self) -> u64 { self.low }
    fn high_pulses_emitted(&self) -> u64 { self.high }
}

#[derive(Debug)]
struct Broadcast {
    name: String,
    counter: PulseCounter,
    connected_to: Vec<&dyn Module>,
}
impl Broadcast {
    fn new() -> Self {
        let name = String::from("broadcaster");
        let counter = PulseCounter::new();
        let connected_to = vec![];
        Self { name, counter, connected_to }
    }
}
impl Module for Broadcast {
    fn handle_pulse(&mut self, pulse: &Pulse) {
        self.counter.handle_pulse(&pulse);
    }
    fn low_pulses_received(&self) -> u64 { self.counter.low }
    fn high_pulses_emitted(&self) -> u64 { self.counter.high }
    fn add_connection_to(&mut self, other: &dyn Module) { self.connected_to.push(other); }
}

#[derive(Debug)]
struct FlipFlip {
    name: String,
    counter: PulseCounter,
    connected_to: Vec<&dyn Module>,
    is_on: bool,
}
impl FlipFlop {
    fn new(name: &str) -> Self {
        let name = name.to_string();
        let counter = PulseCounter::new();
        let connected_to = vec![];
        let is_on = false;
        Self { name, counter, connected_to, is_on }
    }
}
impl Module for FlipFlip {
    fn handle_pulse(&mut self, pulse: &Pulse) {
        self.counter.handle_pulse(&pulse);
        pulse match {
            &Pulse::Low(_) => {
                let was_on = self.is_on;
                self.is_on = !self.is_on;
                match self.was_on {
                    false => {
                        self.connected_to.iter()
                            .for_each(|connected_to| connected_to.handle_pulse(&Pulse::High(self.name.clone())))
                    },
                    true => {
                        self.connected_to.iter()
                            .for_each(|connected_to| connected_to.handle_pulse(&Pulse::Low(self.name.clone())))
                    },
                }
            },
            &Pulse::High(_) => (),
        }
    }
    fn low_pulses_received(&self) -> u64 { self.counter.low }
    fn high_pulses_emitted(&self) -> u64 { self.counter.high }
}

#[derive(Debug)]
struct Conjunction {
    name: String,
    counter: PulseCounter,
    connected_to: Vec<&dyn Module>,
    most_recent_pulses: HashMap<String, Pulse>,
}
impl Conjunction {
    fn new(name: &str) -> Self {
        let name = name.to_string();
        let counter = PulseCounter::new();
        let connected_to = vec![];
        let most_recent_pulses = HashMap::new();
        Self { name, counter, connected_to, most_recent_pulses }
    }
}
impl Module for Conjunction {
    fn handle_pulse(&mut self, pulse: &Pulse) {
        self.counter.handle_pulse(&pulse);
        match pulse {
            Pulse::High(sender_name) | Pulse::Low(sender_name) => {
                self.most_recent_pulses.insert(sender_name.clone(), pulse.clone());
                if self.most_recent_pulses.values().all(|pulse| matches!(pulse, Pulse::High(_))) {
                    self.connected_to.iter()
                        .for_each(|connected_to| connected_to.handle_pulse(&Pulse::Low(self.name.clone())))
                } else {
                    self.connected_to.iter()
                        .for_each(|connected_to| connected_to.handle_pulse(&Pulse::High(self.name.clone())))
                }
            }
        }
    }
    fn low_pulses_received(&self) -> u64 { self.counter.low }
    fn high_pulses_emitted(&self) -> u64 { self.counter.high }
    fn add_connection_from(&mut self, other: &Self) {
        self.most_recent_pulse.insert(other.name.clone(), Pulse::Low(other.name.clone()));
    }
}

/*
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
*/

/*
fn returns_summarizable() -> impl Summary {
*/

#[derive(Debug, Clone)]
// Include the name of the sender in the pulse
enum Pulse {
    High(String),
    Low(String),
}

#[derive(Debug)]
struct ParseResult {
}

fn parse(input: &str) -> ParseResult {
    let mut lines = input.lines();
    ParseResult { }
}

pub fn solve(input_path: &str) -> String {
    let input = std::fs::read_to_string(input_path).unwrap();
    let ParseResult { } = parse(&input);
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() {
        let result = solve("data/test_input.txt");
        println!("result: {result}");
        assert_eq!(result, 19114.to_string());
    }
}
