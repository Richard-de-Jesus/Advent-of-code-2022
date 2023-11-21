This is the 1st version of part 1, just for archiving.

fn main() {
    let contents: Vec<String> = fs::read_to_string("Day01.txt")
        .expect("Erorr: reading file.")
        .lines()
        .map(String::from)
        .collect();
    
    let mut max = 0;
    let mut temp = 0;
    let mut number = 0;
       for element in &contents {
            temp = element.to_string().parse().unwrap_or(-1);
            match temp {
                -1 => {
                    if number > max {
                        max = number;
                    }
                    number = 0;
                }, 
                _ => number += temp,
            };
       }
    println!("temp: {temp}");
   
    println!("{max}");
}
