use std::fs;
use regex::Regex;

fn main() {
    let data = fs::read_to_string("input").unwrap();
    let regex = Regex::new(r"((\d \p{L}{1}).*)").unwrap();

    let maxRed = 12;
    let maxGreen = 13;
    let maxBlue = 14;

    let mut result = 0;

    for line in data.lines() {
        let resultsIter = regex.find_iter(line);
        println!("{:?}", resultsIter);
    }
}
