use regex::Regex;

#[tokio::main]
async fn main() {
    let input = aoc_input_resolver::get_input(4).await;
    let lines = input.lines();
    let mut points = 0;
    let base: u32 = 2;
    let regex = Regex::new(r"Card\s+\d+:\s+").unwrap();
    for line in lines {
        let clean_line = regex.replace(line, "");
        let inputs: Vec<&str> = clean_line.split("|").collect();
        let winning: Vec<i32> = inputs[0].split_whitespace().map(|number| number.parse().unwrap()).collect();
        let own: Vec<i32> = inputs[1].split_whitespace().map(|number| number.parse().unwrap()).collect();
        let exp = matcher(&winning, &own);
        let p = match exp {
            Some(n) => base.pow(n),
            None => 0
        };
        points += p;
    }
    println!("{points}");
}


fn matcher(lookup: &Vec<i32>, input: &Vec<i32>) -> Option<u32> {
    let mut exp = 0;
    for number in input {
        if lookup.contains(number) {
            exp += 1;
        }
    }
    if exp == 0 {
        None
    } else {
        Some(exp - 1)
    }
}

