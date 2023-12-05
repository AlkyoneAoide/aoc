use std::fs;

fn main() {
    let data = fs::read_to_string("input").unwrap();
    let line_len = data.lines().collect::<Vec<_>>()[0].len();
    let mut result = 0;

    println!("Line length: {line_len}");

    for i in 0..data.len() {
        if data.chars().nth(i).unwrap() == '*' {
            let mut num_matches: Vec<usize> = Vec::new();

            num_matches.append(&mut get_matches(data.clone(), -(line_len as isize) - 1, i));

            println!("Top matches: {num_matches:#?}");

            num_matches.append(&mut get_matches(data.clone(), 0, i));

            println!("Middle matches: {num_matches:#?}");

            if i + line_len < data.len() {
                num_matches.append(&mut get_matches(data.clone(), (line_len as isize) + 1, i));
            }
            
            println!("Bottom matches: {num_matches:#?}");
            
            if num_matches.len() == 2 {
                result += num_matches[0]*num_matches[1];
            }

            println!("Intermediate result: {result}");
        }
    }

    println!("Result: {result}");
}

fn get_matches(data: String, offset: isize, data_pos: usize) -> Vec<usize> {
    let mut num_matches: Vec<usize> = Vec::new();
    let mut tmp_num: isize = -1;

    for pos in -1..2 {
        let char_pos = (((data_pos as isize) + offset) + (pos)) as usize;
        let c = data.chars().nth(char_pos).unwrap();

        if c.is_ascii_digit() {
            let result = find_num(data.clone(), char_pos);

            if result as isize != tmp_num {
                num_matches.push(result);
                tmp_num = result as isize;
            }
        } else {
            tmp_num = -1;
        }
    }

    return num_matches;
}

fn find_num(data: String, pos: usize) -> usize {
    let mut num: Vec<char> = Vec::new();
    let mut in_num: bool = false;
    let mut done: bool = false;

    println!("Seeking to {pos}");

    for i in 0..data.len() {
        if i == pos {
            done = true;
        }

        if data.chars().nth(i).unwrap().is_ascii_digit() {
            if !in_num {
                num.clear();
                in_num = true;
            }

            num.push(data.chars().nth(i).unwrap());
        } else {
            in_num = false;

            if done {
                return num.iter().collect::<String>().parse::<usize>().unwrap();
            }
        }
    }

    panic!("Couldn't find num...");
}
