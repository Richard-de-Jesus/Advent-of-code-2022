use std::time::Instant;

fn main() {
    let start = Instant::now();
    let mut contents = include_str!("input.txt")
        .split("\n\n")
        .map(|x| x.lines()
             .map(|x| x.parse::<u32>().unwrap()).sum())
        .collect::<Vec<u32>>();
    // sorting, so largest lists are at last.
    contents.sort_unstable();

    // getting largest calorie list.
    let part_1 = contents.last().unwrap();
    // 3 largest lists.
    let part_2: u32 = contents.iter().rev()
        .take(3).sum();

    println!("part 1: {part_1}");
    println!("part 2: {part_2}");
    println!("T: {:?}", start.elapsed());

}
