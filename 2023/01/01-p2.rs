use std::fs;

fn main() {
    let data = fs::read_to_string("input").expect("Should have  been able to read the file");
    let mut result = 0;

    for line in data.lines() {
        let chars: Vec<char> = line.chars().collect();
        let mut nums: Vec<u32> = Vec::new();

        for i in 0..chars.len() {
            let (mut s3, mut s4, mut s5): (String, String, String) = ("".to_string(), "".to_string(), "".to_string());

            if chars.len()-i >= 3 {
                s3 = chars[i..=i+2].iter().collect();
            }
            
            if chars.len()-i >= 4 {
                s4 = chars[i..=i+3].iter().collect();
            }
            
            if chars.len()-i >= 5 {
                s5 = chars[i..=i+4].iter().collect();
            }

            match chars[i] {
                '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => {
                    nums.push(chars[i].to_digit(10).unwrap());
                },
                'o' => {
                    if s3 == "one" {
                        nums.push(1);
                    }
                },
                't' => {
                    if s3 == "two" {
                        nums.push(2);
                    } else if s5 == "three" {
                        nums.push(3);
                    }
                },
                'f' => {
                    if s4 == "four" {
                        nums.push(4);
                    } else if s4 == "five" {
                        nums.push(5);
                    }
                },
                's' => {
                    if s3 == "six" {
                        nums.push(6);
                    } else if s5 == "seven" {
                        nums.push(7);
                    }
                },
                'e' => {
                    if s5 == "eight" {
                        nums.push(8);
                    }
                },
                'n' => {
                    if s4 == "nine" {
                        nums.push(9);
                    }
                },
                _ => (),
            }
        }

        println!("{:#?}", nums);
        result += (nums[0].to_string() + &nums.last().unwrap().to_string()).parse::<u32>().unwrap();
    }

    println!("Result: {}", result);
}
