mod parse_input;

use std::collections::VecDeque;

fn main() {
    let lines = parse_input::parse_input(None);
    //let lines = parse_input::parse_input(Some("test_input.txt".to_string()));
    let mut nums = Vec::new();
    for line_result in lines {
        match line_result {
            Ok(line) => {
                let num: i32 = match line.parse::<i32>() {
                    Ok(n) => n,
                    Err(e) => {
                        println!("error converting string to num: {:?}", e);
                        -1
                    },
                };
                nums.push(num);
            },
            Err(e) => println!("error parsing line: {:?}", e),
        }
    }

    println!("part 1");
    part1(&nums);

    println!("part 2");
    part2(&nums);
}

fn part1(nums: &Vec<i32>) {
    let mut previous: Option<i32> = None;
    let mut num_increases = 0;
    for num in nums {
        match previous {
            Some(p) => {
                if p < *num {
                    num_increases += 1;
                }
            },
            None => (),
        }
        previous = Some(*num);
    }
    println!("num_increases: {:?}", num_increases);
}

fn part2(nums: &Vec<i32>) {
    let mut previous_sum: Option<i32> = None;
    let mut window: VecDeque<i32> = VecDeque::new();
    let mut num_increases = 0;
    for num in nums {
        while window.len() >= 3 {
            window.pop_front();
        }
        window.push_back(*num);
        let sum = window.iter().sum();
        println!("sum: {:?}", sum);
        println!("window: {:?}", window);
        match previous_sum {
            Some(p) => {
                if window.len() >=3 && p < sum {
                    num_increases += 1;
                }
            },
            None => (),
        }
        if window.len() >= 3 {
            previous_sum = Some(sum);
        }
    }
    println!("num_increases: {:?}", num_increases);
}
