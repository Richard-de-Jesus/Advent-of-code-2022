

fn round_result(opp: char, res: char) -> i32 {
    let  score = match res {
        'X' => {
            match opp {
                'A' => 3,
                'B' => 1,
                'C' => 2,
                 _ => panic!("wrong opponent 1"),
            }
        }
        'Y' => {
            match opp {
                'A' => 1 + 3,
                'B' => 2 + 3,
                'C' => 3 + 3,
                _ => panic!("wrong opponent 2"),
            }
        }
        'Z' => {
            match opp {
                'A' => 2 + 6,
                'B' => 3 + 6,
                'C' => 1 + 6,
                _ => panic!("wrong opponent 3"),
            }
        }
        _ => panic!("invalid result."),
    };
    score
}
#[allow(unused_assignments)]
fn main() {
   let contents: Vec<_> = include_str!("input.txt")
       .lines().collect();

   let mut opponent = 'a';
   let mut result = 'a';
   let mut score = 0;

   for line in contents {
       opponent = line.chars().next().unwrap();
       result = line.chars().nth(2).unwrap();

       score += round_result(opponent, result); 
   }
   println!("total: {score}");
}
