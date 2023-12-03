use std::{fs, ops::RangeInclusive};

fn main() {
    let data: String = fs::read_to_string("input").unwrap();
    let lines: Vec<&str> = data.lines().collect();
    let mut result: usize = 0;

    for line_num in 0..lines.len() {
        let sym_line = lines[line_num];

        println!("Line {line_num}");

        if line_num != 0 {
            println!("Top:");
            let prev_line = lines[line_num-1];
            result += find_result(sym_line, prev_line);
        }

        println!("Middle:");
        result += find_result(sym_line, sym_line);

        if line_num != lines.len()-1 {
            println!("Bottom:");
            let next_line = lines[line_num+1];
            result += find_result(sym_line, next_line);
        }

    }

    println!("Result: {result}");

    for line_num in 0..lines.len() {
        println!("Line {line_num}, {} nums found", line_to_num_ranges(lines[line_num].chars().collect()).len());
    }
}

fn find_result(sym_line: &str, char_line: &str) -> usize {
    let mut result = 0;

    for char_num in 0..sym_line.len() {
        let pos_result: Option<Vec<usize>> = find_relative_pos(sym_line, char_line, char_num);

        // Because of this, we only stay in the loop if we are a symbol
        if pos_result == None {
            continue;
        }

        let line_num_ranges: Vec<RangeInclusive<usize>> = line_to_num_ranges(char_line.chars().collect());
        let pos: Vec<usize> = pos_result.unwrap();
        let mut pos_to_remove: Vec<bool> = vec![false; pos.len()];

        for range in line_num_ranges.clone() {
            let matches: Vec<bool> = pos.clone().iter().map(|a| { range.contains(a) } ).collect();
            let mut matched: bool = false;
            
            for i in 0..matches.clone().len() {
                if matches[i] == true && matched == false {
                    matched = true;
                } else if matches[i] == true {
                    pos_to_remove[i] = true;
                }
            }
        }

        let mut removed_pos: Vec<Option<usize>> = vec![None; pos.len()];
        for i in 0..pos.len() {
            if pos_to_remove[i] {
                removed_pos[i] = None;
            } else {
                removed_pos[i] = Some(pos[i]);
            }
        }

        println!("{:#?}", pos);
        println!("{:#?}", removed_pos);

        for digit in removed_pos {
            if digit == None {
                continue;
            }

            for range in line_num_ranges.clone() {
                if range.contains(&digit.unwrap()) {
                    result += char_line.chars().collect::<Vec<_>>()[range]
                        .iter().collect::<String>()
                        .parse::<usize>().unwrap();
                }
            }
        }
    }

    if result != 0 {
        println!("{result}\n");
    }

    return result;
}

// Finds positions of digits relative to symbol char_num
fn find_relative_pos(sym_line: &str, char_line: &str, sym_pos: usize) -> Option<Vec<usize>> {
    let c: char = sym_line.chars().nth(sym_pos).unwrap();

    if c.is_ascii_digit() || c == '.' {
        return None;
    }

    let has_digit: Vec<bool> = char_line.chars().collect::<Vec<char>>()[sym_pos-1..=sym_pos+1].to_vec()
        .iter().map(|d| { d.is_ascii_digit() } ).collect::<Vec<bool>>();

    let mut digit_pos: Vec<usize> = Vec::new();

    for i in 0..has_digit.len() {
        if has_digit[i] {
            digit_pos.push(sym_pos + i-1);
        }
    }

    if digit_pos.len() > 0 {
        return Some(digit_pos);
    } else {
        return None;
    }
}

fn line_to_num_ranges(line: Vec<char>) -> Vec<RangeInclusive<usize>> {
    let mut ranges: Vec<RangeInclusive<usize>> = Vec::new();
    let mut first: (char, usize) = ('a', 0);
    let mut second: (char, usize) = ('a', 0);

    for i in 0..line.len() {
        let c: char = line[i];
        if c.is_ascii_digit() {
            if first.0.is_ascii_digit() {
                second.0 = c;
                second.1 = i;
            } else {
                first.0 = c;
                first.1 = i;
            }

            if i == line.len()-1 {
                if second.0.is_ascii_digit() {
                    ranges.push(first.1..=second.1);
                } else {
                    ranges.push(first.1..=first.1);
                }
            }

        } else if first.0.is_ascii_digit() && second.0.is_ascii_digit() {
            ranges.push(first.1..=second.1);
            first = ('a', 0);
            second = ('a', 0);
        } else if first.0.is_ascii_digit() {
            ranges.push(first.1..=first.1);
            first = ('a', 0);
            second = ('a', 0);
        }
    }

    return ranges;
}
