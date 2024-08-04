use aoc_input_resolver::get_input;
use regex::Regex;

#[tokio::main]
async fn main() {
    let input = get_input(1).await;
    let lines = input.lines();
    let mut cal = Vec::new();
    let regex = Regex::new(r"one|two|three|four|five|six|seven|eight|nine|\d").unwrap();
    for (i, line) in lines.enumerate() {
        let matches: Vec<&str> = regex
            .captures_iter(line)
            .map(|c| c.extract::<0>().0)
            .map(|d| map(d))
            .collect();
        let number = matches[0].to_owned() + matches[matches.len() - 1];
        println!("actual: {},{}", i + 1, &number);
        cal.push(number.parse::<u32>().unwrap());
    }
    let result: u32 = cal.iter().sum();
    println!("{}", result);
}

fn map(input: &str) -> &str {
    // println!("----------------------");
    // println!("{}", input);
    let output = match input {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => input,
    };
    // println!("{}", output);
    output
}
