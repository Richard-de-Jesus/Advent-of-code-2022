
fn read(_lines: &Vec<&str>) {

}

fn main() {
    let file = include_str!("input.txt")
        .lines().collect::<Vec<_>>();

    read(&file);

}
