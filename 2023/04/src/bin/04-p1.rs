use std::{fs, str::Chars};

fn main() {
    let data = fs::read_to_string("input").unwrap();
    let mut result = 0;

    for line in data.lines() {
        let mut winning_nums: Vec<usize> = Vec::new();
        let mut nums: Vec<usize> = Vec::new();
        let mut card_matches = 0;

        let mut char_iter = line.chars();
        let mut curr_char = char_iter.next().unwrap();

        curr_char = advance_to_next_digit(&mut char_iter);
        
        let mut current_game: String = String::new();

        while curr_char.is_ascii_digit() {
            current_game += &curr_char.to_string();
            curr_char = char_iter.next().unwrap();
        }

        curr_char = advance_to_next_digit(&mut char_iter);

        let mut curr_num: String = String::new();

        while curr_char != '|' {
            if curr_char.is_ascii_digit() {
                curr_num += &curr_char.to_string();
            } else if curr_num.len() > 0 {
                winning_nums.push(curr_num.parse::<usize>().unwrap());
                curr_num = String::new();
            }

            curr_char = char_iter.next().unwrap();
        }

        curr_char = advance_to_next_digit(&mut char_iter);
        curr_num = String::new();

        loop {
            if curr_char.is_ascii_digit() {
                curr_num += &curr_char.to_string();
            } else if curr_num.len() > 0 {
                nums.push(curr_num.parse::<usize>().unwrap());
                curr_num = String::new();
            }

            let tmp_next_char = char_iter.next();

            if tmp_next_char.is_some() {
                curr_char = tmp_next_char.unwrap();
            } else {
                break;
            }
        }

        if curr_num.len() > 0 {
            nums.push(curr_num.parse::<usize>().unwrap());
        }

        for num in nums {
            if winning_nums.contains(&num) {
                card_matches += 1;
            }
        }

        if card_matches > 0 {
            result += usize::pow(2, card_matches-1);
        }

        println!("Matches: {card_matches}");
        println!("Intermediate result: {result}");
    }

    println!("Result: {result}");
}

fn advance_to_next_digit(iter: &mut Chars) -> char {
    let mut curr_char = iter.next().unwrap();

    while !curr_char.is_ascii_digit() {
        curr_char = iter.next().unwrap();
    }

    return curr_char;
}
