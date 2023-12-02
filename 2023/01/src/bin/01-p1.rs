use std::fs;

fn main() {
    let data = fs::read_to_string("input").unwrap();
    let mut result = 0;

    for line in data.lines() {
        let v: Vec<u32> = line.chars()
            .filter_map(|c| c.to_digit(10))
            .collect();
        println!("{:#?}", v);
        
        let num = (v[0].to_string() + &v.last().unwrap().to_string()).parse::<u32>().unwrap();
        println!("{}", num);
        result += num;
    }

    println!("Result: {}", result);
}
