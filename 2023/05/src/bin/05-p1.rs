use std::fs;
use std::fmt;

struct GardenMap {
    src: String,
    dst: String,
    src_range_start: Vec<usize>,
    dst_range_start: Vec<usize>,
    range_len: Vec<usize>
}

impl fmt::Debug for GardenMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Source: {}", self.src).unwrap();
        writeln!(f, "Destination: {}", self.dst).unwrap();
        writeln!(f, "Left Column: {:#?}", self.dst_range_start).unwrap();
        writeln!(f, "Middle Column: {:#?}", self.src_range_start).unwrap();
        write!(f, "Right Column: {:#?}", self.range_len).unwrap();

        return Ok(());
    }
}

fn main() {
    let data = fs::read_to_string("input").unwrap();
    let data_lines = data.lines().collect::<Vec<&str>>();

    let mut seeds: Vec<usize> = get_seeds(data_lines[0]);
    let garden_maps: Vec<GardenMap> = get_map(data_lines[1..data_lines.len()].iter().map(|s| s.to_string() + "\n").collect());

    let mut seed = seeds[0];
    let mut src = "seed";
    let mut locations: Vec<usize> = Vec::new();

    loop {
        let map = find_garden_map(src, &garden_maps);

        println!("Target: {seed}");
        
        for i in 0..map.src_range_start.len() {
            let src_start = map.src_range_start[i];
            let dst_start = map.dst_range_start[i];
            let len = map.range_len[i];

            if (src_start..src_start+len).contains(&seed) {
                seed = seed + dst_start-src_start;
                break;
            }
        }

        src = &map.dst;

        if src == "location" {
            src = "seed";
            locations.push(seed);
            seeds.remove(0);

            if seeds.len() == 0 {
                break;
            }

            seed = seeds[0];
        }
    }

    println!("Locations: {locations:#?}");
    println!("Smallest location: {}", get_smallest(locations));
}

fn get_smallest(list: Vec<usize>) -> usize {
    let mut smallest = list[0];

    for num in list {
        if num < smallest {
            smallest = num;
        }
    }

    return smallest;
}

fn find_garden_map<'a>(src: &str, list: &'a Vec<GardenMap>) -> &'a GardenMap {
    for i in 0..list.len() {
        if list[i].src == src {
            return &list[i];
        }
    }

    panic!("Map not found in list");
}

fn get_seeds(data: &str) -> Vec<usize> {
    let mut seeds: Vec<usize> = Vec::new();
    let mut curr_num: String = String::new();

    for char in data.chars() {
        if char.is_ascii_digit() {
            curr_num += &char.to_string();
        } else if curr_num.len() > 0 {
            seeds.push(curr_num.parse::<usize>().unwrap());
            curr_num = String::new();
        }
    }

    if curr_num.len() > 0 {
        seeds.push(curr_num.parse::<usize>().unwrap());
    }

    return seeds;
}

fn get_map(data: Vec<String>) -> Vec<GardenMap> {
    let mut garden_maps: Vec<GardenMap> = Vec::new();

    let mut src = String::new();
    let mut dst = String::new();
    let mut src_found: usize = 0;

    let mut src_range_start: Vec<usize> = Vec::new();
    let mut dst_range_start: Vec<usize> = Vec::new();
    let mut range_len: Vec<usize> = Vec::new();
    let mut tmp_num: String = String::new();
    let mut num_selection = 0;

    for line in data.iter() {
        for char in line.chars() {
            if char.is_ascii_alphabetic() {
                if range_len.len() > 0 {
                    let new_garden_map: GardenMap = GardenMap { src, dst: dst[0..dst.len()-3].to_string(),
                        src_range_start, dst_range_start, range_len };
                    garden_maps.push(new_garden_map);

                    src = String::new();
                    dst = String::new();
                    src_found = 0;

                    src_range_start = Vec::new();
                    dst_range_start = Vec::new();
                    range_len = Vec::new();
                    tmp_num = String::new();
                    num_selection = 0;
                }

                if src_found == 0 {
                    src += &char.to_string();
                } else if src_found == 2 {
                    dst += &char.to_string();
                }
            } else if char.is_ascii_digit() {
                tmp_num += &char.to_string();
            } else if char == '-' {
                src_found += 1;
            } else if tmp_num.len() > 0 {
                match num_selection {
                    0 => dst_range_start.push(tmp_num.parse::<usize>().unwrap()),
                    1 => src_range_start.push(tmp_num.parse::<usize>().unwrap()),
                    2 => range_len.push(tmp_num.parse::<usize>().unwrap()),
                    _ => ()
                }

                tmp_num = String::new();
                num_selection += 1;

                if num_selection > 2 {
                    num_selection = 0;
                }
            }
        }
    }

    if range_len.len() > 0 {
        let new_garden_map: GardenMap = GardenMap { src, dst: dst[0..dst.len()-3].to_string(),
            src_range_start, dst_range_start, range_len };
        garden_maps.push(new_garden_map);
    }

    return garden_maps;
}
