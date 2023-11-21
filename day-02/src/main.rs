use std::time::Instant;

type Pair = (char, char);
// opponent and myself.
fn score1((opp, mys): &Pair) -> i32 {

    let mut score1 = match mys {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
         _ => panic!("invalid myself"),
    };
    match (opp, mys ) {
        ('A', 'Y')|('B', 'Z')|('C', 'X') => score1 += 6,
        ('A', 'X')|('B', 'Y')|('C', 'Z') => score1 += 3,
        _ => (),
    }
    score1
}
// result and opponent.
fn score2((res, opp): &Pair) -> i32 {
    return match (opp, res) {
        ('X', 'A') => 3,
        ('X', 'B') => 1,
        ('X', 'C') => 2,
        ('Y', 'A') => 1 + 3,
        ('Y', 'B') => 2 + 3,
        ('Y', 'C') => 3 + 3,
        ('Z', 'A') => 2 + 6,
        ('Z', 'B') => 3 + 6,
        ('Z', 'C') => 1 + 6,
        _ => panic!("Invalid input"),
    };
}
fn main() {
    let start = Instant::now();

    let contents = include_str!("input.txt")
        .lines()
        .map(|x| { let mut a = x.chars();
            (a.next().unwrap(),
             a.nth(1).unwrap())})
        .collect::<Vec<Pair>>();

    let part_1: i32 = contents.iter()
        .map(|x| score1(x)).sum();
    let part_2: i32 = contents.iter()
        .map(|x| score2(x)).sum();

    println!("part 1: {part_1}");
    println!("part 2: {part_2}");
    println!("T: {:?}", start.elapsed());
}
