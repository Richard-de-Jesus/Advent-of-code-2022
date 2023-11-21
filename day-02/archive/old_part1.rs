// opp = opponent, mys = myself, reducing verbosity.
fn check_score(opp: char, mys: char) -> i32 {
    let mut score = match mys {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
         _ => panic!("invalid myself"),
    };
    // casting char to u32 in last branch, to compare ASCII values.
    if opp == 'A' && mys == 'Y' {
        score += 6;
    } else if  opp == 'B' && mys == 'Z' {
        score += 6;
    } else if opp == 'C' && mys == 'X' {
        score += 6;
    } else if opp as u32 + 23 == mys as u32 {
        score += 3;
    }
    score
}
#[allow(unused_assignments)]
fn main() {
    let contents: Vec<_> = include_str!("input.txt")
        .lines().collect();
 
    let mut opponent = 'a';
    let mut myself = 'a';
    let mut total = 0;
    for line in contents {
        opponent = line.chars().nth(0).unwrap();
        myself = line.chars().nth(2).unwrap();

        total += check_score(opponent, myself);
    } 
    println!("total: {total}");
}










