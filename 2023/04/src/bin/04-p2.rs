use std::{fs, str::Chars};

fn main() {
    let data = fs::read_to_string("input").unwrap();
    let data_lines: Vec<&str> = data.lines().collect();
    let card_associations: Vec<usize> = get_card_nums(data_lines.clone());
    
    let mut cards: Vec<usize> = vec![0; data_lines.len()];
    let mut processed_cards: usize = 0;

    println!("Cards and wins: {card_associations:#?}");

    for i in 0..data_lines.len() {
        cards[i] += 1;

        for j in 1..=card_associations[i] {
            for k in 0..cards[i] {
                cards[i+j] += 1;
            }
        }

        processed_cards += 1;
    }

    println!("Cards: {cards:#?}");

    for i in 0..cards.len() {
        processed_cards += cards[i] * card_associations[i];
    }

    println!("Total processed cards: {processed_cards}");
}

fn get_card_nums(data: Vec<&str>) -> Vec<usize> {
    let mut num_won: Vec<usize> = vec![0; data.len()];

    for i in 0..data.len() {
        let line = data[i];

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

        num_won[i] = card_matches;
    }

    return num_won;
}

fn advance_to_next_digit(iter: &mut Chars) -> char {
    let mut curr_char = iter.next().unwrap();

    while !curr_char.is_ascii_digit() {
        curr_char = iter.next().unwrap();
    }

    return curr_char;
}
