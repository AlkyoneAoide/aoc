use std::fs;
use regex::Regex;

fn main() {
    let data = fs::read_to_string("input").unwrap();
    let game_num = Regex::new(r"Game \d*").unwrap();
    let roll_data = Regex::new(r"(?U)((\d* \p{L}{1}).*)").unwrap();

    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

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

        if local_max_red <= max_red && local_max_blue <= max_blue && local_max_green <= max_green {
            let game_num_chars: Vec<_> = game_num.find(line).unwrap().as_str().chars().collect();
            let game_num_result: String = game_num_chars[5..game_num_chars.len()].iter().collect();
            result += game_num_result.parse::<u32>().unwrap();
        }
    }

    println!("Result: {result}");
}
