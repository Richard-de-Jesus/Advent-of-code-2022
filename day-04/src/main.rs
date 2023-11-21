use std::time::Instant;

type Elf = Vec<u32>;
fn parse(elfs: &Vec<&str>) -> Pair {
    let elfs = elfs.iter()
        .map(|x| x.split("-")
             .map(|y| y.parse().unwrap())
             .collect())
        .collect::<Vec<Elf>>();
    
    let elf1: Elf = elfs[0].clone();
    let elf2: Elf = elfs[1].clone();
    (elf1, elf2)
} 
type Pair = (Elf, Elf);
fn range1((elf1, elf2): &Pair) -> u32 {
    
    let x = elf1[0] <= elf2[0] && elf1[1] >= elf2[1];
    if x || elf2[0] <= elf1[0] && elf2[1] >= elf1[1] {
        return 1;
    }
    0
}
fn range2((elf1, elf2): &Pair) -> u32 {
    if elf1[1] >= elf2[0] {
        if elf2[1] < elf1[0] {
            return 0;
        }
        return 1;
    } 
    0
}
fn main() {
    // 9.25 ms
    let start = Instant::now();
    let input = include_str!("input.txt")
        .lines()
        .map(|x| x.split(",").collect())
        .collect::<Vec<Vec<_>>>();

    let mut elfs = Vec::with_capacity(input.len());
    for i in &input {
        elfs.push(parse(i));
    }

    let part_1: u32 = elfs.iter()
        .map(|x| range1(&x)).sum();
    let part_2: u32 = elfs.iter()
        .map(|x| range2(&x)).sum();

    println!("part 1: {part_1}");
    println!("part 2: {part_2}");
    println!("T: {:?}", start.elapsed());
}
