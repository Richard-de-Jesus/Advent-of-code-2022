fn priority(left: &[u8], right: &[u8]) -> u32 {
    for i in left {
        if right.contains(i) {
            if *i >= b'a' {
                return (i - b'a') as u32 + 1;
            } else {
                return (i - b'A') as u32 + 27;
            };
        }
    }
    0
}

fn main() {
    let total: u32 = include_bytes!("input.txt")
        .split(|x| *x == b'\n')
        .map(|x| x.split_at(x.len() / 2))
        .map(|(l, r)| priority(l, r))
        .sum::<u32>();

    println!("{total}");
}
