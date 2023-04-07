use std::fs::read_to_string;

pub fn solve() -> (String, String){
    // Part 1:
    //A for Rock, B for Paper, and C for Scissors; column A
    //X for Rock, Y for Paper, and Z for Scissors; column B

    // Part 2:
    //A for Rock, B for Paper, and C for Scissors; column A
    //X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win ; column B

    //General
    //1 for rock 2 for paper 3 for scissor.
    //0 for losing, 3 for draw, 6 for win.

    // Part 1 calculate total score:
    let file_raw = read_to_string("input/day02.txt").unwrap();

    let tournament_score_p1: Vec<u32> = file_raw.lines().into_iter().map(|x| get_score_from_round(x.split(' ').collect::<Vec<&str>>())).collect();
    let tournament_score_p2: Vec<u32> = file_raw.lines().into_iter().map(|x| get_score_from_round_2(x.split(' ').collect::<Vec<&str>>())).collect();

    let res1: u32 = tournament_score_p1.into_iter().sum();
    let res2: u32 = tournament_score_p2.into_iter().sum();

    return (res1.to_string(), res2.to_string());
}

// p1
fn get_score_from_round(entry: Vec<&str>) -> u32{
    
    let opponent = entry[0];
    let choice = entry[1];
    let mut res = 0;

    match choice {
        "X" => res += 1,
        "Y" => res += 2,
        "Z" => res += 3,
        _ => {panic!("Input value was unexpected");}
    }

    let match_res = get_result_from_match(opponent, choice);

    match match_res.as_str() {
        "win" => res += 6,
        "draw" => res += 3,
        "lose" => res += 0,
        _ => {panic!("Input value was unexpected");}
    }

    return res
}

fn get_result_from_match(opponent: &str, choice: &str) ->  String{
    let res;
    match choice {
        "X" => {
            match opponent {
                "A" => res = "draw",
                "B" => res = "lose",
                "C" => res = "win",
                _ => {panic!("Input value was unexpected");}
            }
        },
        "Y" => {
            match opponent {
                "A" => res = "win",
                "B" => res = "draw",
                "C" => res = "lose",
                _ => {panic!("Input value was unexpected");}
            }
        },
        "Z" => 
            match opponent {
                "A" => res = "lose",
                "B" => res = "win",
                "C" => res = "draw",
                _ => {panic!("Input value was unexpected");
            }
        },
        _ => {panic!("Input value was unexpected");}
    }

    String::from(res)
}

// p2
fn get_score_from_round_2(entry: Vec<&str>) -> u32{
    let opponent = entry[0];
    let result = entry[1];

    let mut res = 0;

    match result {
        "X" => res += 0,
        "Y" => res += 3,
        "Z" => res += 6,
        _ => {panic!("Input value was unexpected");}
    }

    let match_res = get_choice_to_make(opponent, result);

    match match_res.as_str() {
        "rock" => res += 1,
        "paper" => res += 2,
        "scissors" => res += 3,
        _ => {panic!("Input value was unexpected");}
    }

    res
}

fn get_choice_to_make(opponent: &str, result: &str) -> String{
    let res;

    match result {
        "X" => { // Lose
            match opponent {
                "A" => res = "scissors",
                "B" => res = "rock",
                "C" => res = "paper",
                _ => {panic!("Input value was unexpected");}
            }
        },
        "Y" => { // Draw
            match opponent {
                "A" => res = "rock",
                "B" => res = "paper",
                "C" => res = "scissors",
                _ => {panic!("Input value was unexpected");}
            }
        },
        "Z" => {
            match opponent {
                "A" => res = "paper",
                "B" => res = "scissors",
                "C" => res = "rock",
                _ => {panic!("Input value was unexpected");}
            }
        },
    _ => {panic!("Input value was unexpected");}
    }

    String::from(res)
}
