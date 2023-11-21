use std::{time::Instant, collections::BTreeSet};


// pkt: packet.
fn marker(pkt: &[u8], p: u8) -> usize {
    // number of distinct chars.
    // part 1 is 4 chars, part 2 is 14 chars.
    let num: usize;
    num = match p {
        1 => 4,
        _ => 14,
    };
    let (result, _) = pkt.windows(num)
        .enumerate()
        .find(|(_i, arr)| {
            let set = arr.iter()
                .collect::<BTreeSet<&u8>>();
            if arr.len() == set.len() {
                println!("iter: {set:?}");
            }
            arr.len() == set.len()
        }).unwrap();

    return result + num;
}
fn main() {
    let start = Instant::now();

    let file = include_bytes!("input.txt");

    println!("{}", marker(file, 1));
    println!("{}", marker(file, 2));
    println!("T: {:?}", start.elapsed());    
}
