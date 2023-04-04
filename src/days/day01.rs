use std::fs::read_to_string;

pub fn solve() -> (String, String){
    let mut calories: Vec<u32> = read_to_string("input/day01.txt").unwrap()
        .split("\r\n\r\n")
        .map( |e| e.lines().map(|x| x.parse::<u32>().unwrap()).sum() )
        .collect();
    
    // let maxcal: u32 = *calories.iter().max().unwrap(); // works for part 1 not part 2.

    calories.sort_by(|a, b| b.cmp(a));
    let res1 = calories[0];
    let res2: u32 = calories [0..3].iter().sum();

    return (res1.to_string(), res2.to_string());
}

