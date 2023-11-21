use std::time::Instant;

fn priority(arg: &u8) -> u32 {
    return match arg {
        b'a'..=b'z' => (arg - b'a') as u32 + 1,
        b'A'..=b'Z' => (arg - b'A') as u32 + 27,
        _ => panic!("Wrong byte"),
    } 
}
fn main() {
    let start = Instant::now();

    let mut file: Vec<_> = include_bytes!("input.txt")
        .split(|x| *x == b'\n').collect();
    file.pop().unwrap();

    let badge = |xs: &[&[u8]], x: u8| 
        xs[1].contains(&x) && xs[2].contains(&x); 

    let part_1: u32 = file.clone().iter()
        .map(|x| x.split_at(x.len() / 2))
        .map(|(l, r)| l.iter()
             .find(|x| r.contains(x)).unwrap())
        .map(|x| priority(x)).sum();

    let part_2: u32 = file 
        .chunks(3)
        .map(|xs| {
            xs[0]
                .iter()
                .find(|&&x| badge(xs, x))
                .unwrap()
        })
       .map(|x| priority(x)).sum::<u32>();

    println!("part 1: {part_1}");
    println!("part 2: {part_2}");
    println!("T: {:?}", start.elapsed());
}
