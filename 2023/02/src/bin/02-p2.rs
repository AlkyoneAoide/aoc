use std::fs;
use regex::Regex;

fn main() {
    let data = fs::read_to_string("input").unwrap();
    let roll_data = Regex::new(r"(?U)((\d* \p{L}{1}).*)").unwrap();

    let mut result = 0;

    for line in data.lines() {
        let mut local_max_red = 0;
        let mut local_max_green = 0;
        let mut local_max_blue = 0;

        let results_iter = roll_data.find_iter(line);

        for result in results_iter {
            let result_chars: Vec<_> = result.as_str().chars().collect();
            let num: u32 = result_chars[0..result_chars.len()-2].iter().collect::<String>().parse::<u32>().unwrap();
            let col = result_chars.last().unwrap();

            match col {
                'r' => {
                    if num > local_max_red {
                        local_max_red = num;
                    }
                },
                'g' => {
                     if num > local_max_green {
                        local_max_green = num;
                    }               
                },
                'b' => {
                    if num > local_max_blue {
                        local_max_blue = num;
                    }
                },
                _ => (),
            }
        }

        result += local_max_red * local_max_green * local_max_blue;
    }

    println!("Result: {result}");
}
