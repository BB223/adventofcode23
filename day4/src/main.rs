use std::usize;

use regex::Regex;

#[tokio::main]
async fn main() {
    let input = aoc_input_resolver::get_input(4).await;
    let lines = input.lines();
    let mut multi = vec![1; lines.clone().count()];
    let regex = Regex::new(r"Card\s+(\d+):\s+").unwrap();
    for (i, line) in lines.into_iter().enumerate() {
        let clean_line = regex.replace(line, "");
        let inputs: Vec<&str> = clean_line.split("|").collect();
        let winning: Vec<i32> = inputs[0]
            .split_whitespace()
            .map(|number| number.parse().unwrap())
            .collect();
        let own: Vec<i32> = inputs[1]
            .split_whitespace()
            .map(|number| number.parse().unwrap())
            .collect();
        let exp = matcher(&winning, &own);
        for n in i + 1..=i + exp {
            multi[n] += multi[i];
        }
    }
    println!("{}", multi.iter().sum::<u32>());
}

fn matcher(lookup: &Vec<i32>, input: &Vec<i32>) -> usize {
    let mut exp = 0;
    for number in input {
        if lookup.contains(number) {
            exp += 1;
        }
    }
    exp
}
