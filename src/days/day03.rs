use std::fs::read_to_string;
use substring::Substring;

pub fn solve() -> (String, String){
    let file_raw: String = read_to_string("input/day03.txt").unwrap();
    let scores: Vec<u32> = file_raw.lines().map(|ruck| find_common_element(ruck)).map(|s| char2prio(s.chars().collect::<Vec<char>>()[0])).collect();
    
    println!("{:?}", scores);
    
    let res1: u32 = scores.into_iter().sum();



    return (res1.to_string(), "world".to_string());
}

//TODO: Part 2.

fn find_common_element(rucksack: &str) -> String{
    let l = rucksack.len();
    let first: Vec<String> = rucksack.substring(0, l/2).to_string().split("").filter(|s| !s.is_empty()).map(|s| s.to_string()).collect();
    let second: Vec<String> = rucksack.substring(l/2, l).to_string().split("").filter(|s| !s.is_empty()).map(|s| s.to_string()).collect();

    // println!("{:?}{:?}", first, second);

    // Let this be a warning to NEVER AGAIN use fucking sets in rust untill I know more about borrowing and boxes etc.
    // let common_chars: HashSet<String> = first.intersection(&second).collect(); 

    for s in first{
        if second.contains(&s){
            return String::from(s);
        }
    }

    panic!("No elements match in rucksack! (day03)")
}


fn char2prio(ch: char) -> u32 {
    // this is using rust property that characters can be turned into
    // is ascii value by casting them to u32 starting with 'a' as 0.
    match ch {
        'a'..='z' => ch as u32 - 'a' as u32 + 1,
        'A'..='Z' => ch as u32 - 'A' as u32 + 27,
        _ => unreachable!()
    }
}