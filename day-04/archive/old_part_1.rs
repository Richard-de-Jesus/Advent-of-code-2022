fn range(arg: &str) -> u32 {
    let elfs: Vec<_> = arg.split(",").collect();
    if elfs.len() < 2 {
        return 0;
    }

    let elf1: Vec<i32> = elfs[0].split("-")
        .map(|x| x.parse().unwrap())
        .collect();

    let elf2: Vec<i32> = elfs[1].split("-")
        .map(|x| x.parse().unwrap())
        .collect();

    if elf1[0] <= elf2[0] && elf1[1] >= elf2[1] {
            return 1
    }
    if elf2[0] <= elf1[0] && elf2[1] >= elf1[1] {
        return 1;
    }
    0
}
fn main() {
    let total: u32 = include_str!("input.txt")
        .split("\n")
        .map(|x| range(x))
        .sum();
    
    println!("Total: {total}");
}
