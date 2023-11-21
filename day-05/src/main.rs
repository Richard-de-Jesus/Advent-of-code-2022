use std::{str::from_utf8, time::Instant};

type Int = Vec<usize>;
type VInt = Vec<Vec<usize>>;
// parse the stacks.
fn data(arg: &str) -> Vec<Vec<char>> {
    
    let input = arg.lines()
        .map(|x| x.as_bytes().to_owned())
        .collect::<Vec<Vec<u8>>>();

    let num = from_utf8(&input[input.len()-1]);
    // get number of stacks. 
    let size: usize = num.unwrap().split(" ")
        .map(|x| x.parse().unwrap_or(0))
        .max().unwrap();

    // column of each crate.
    let mut positions = vec![1; size];
    for i in 1..size {
        let val = positions[i-1] + 4;
        positions[i] = val;
    }

    let mut stacks = vec![];
    for i in 0..size{
        let mut foo = vec![];
        // the -1 is to skip the 
        // last line with numbers.
        // .rev to put in stack order.
        for j in (0..input.len() - 1).rev() {
            let x = input[j][positions[i]];
            foo.push(x as char);
        }
        while foo.last().unwrap() == &' ' {
            foo.pop().unwrap();
        }
        stacks.push(foo);
    }
    stacks
}

// parse procedures
fn parse(arg: &str) -> Vec<Int> {
    let res = arg.lines()
        .map(|x| x.split(" "))
        .map(|xs| xs
             .map(|x| x.parse().unwrap_or(0))
             .filter(|x| x > &0)
             .collect::<Int>())
        .collect::<Vec<Int>>();
    res 
}

type VStack = Vec<Vec<char>>;
type Stack = Vec<char>;
fn execute1(s: &VStack, procs: &VInt) -> VStack {
    let mut stacks = s.clone();

    for proc in procs { 
        let _move = proc[0];
        // -1 because the stacks are 1-indexed.
        let (from, to) = (proc[1]-1, proc[2]-1);

        for _ in 1..=_move {
            let val = stacks[from].pop().unwrap();
            stacks[to].push(val);
        }
    }
    stacks    
}
fn execute2(st: &VStack, procs: &VInt) -> VStack {
    let mut st = st.clone();
    for proc in procs {
        let (from, to) = (proc[1]-1, proc[2]-1);
       
        let mut temp = vec![];
        move_st(proc[0], &mut st[from], &mut temp);
        move_st(proc[0], &mut temp, &mut st[to]);
    }
    st
}
// pops form st1 and pushs to st2
fn move_st(_move: usize, st1: &mut Stack, st2: &mut Stack) {
    for _ in 1..=_move {
        let val = st1.pop().unwrap();
        st2.push(val);
    }
}

fn main() {
    let start = Instant::now();

    let file = include_str!("input.txt")
        .split("\n\n")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    let stacks = data(&file[0]);

    // procedures
    let procs = parse(&file[1]);

    let part_1 = execute1(&stacks, &procs);
    let part_2 = execute2(&stacks, &procs);

    for mut x in part_1 {
        print!("{:?}", x.pop().unwrap());
    }
    println!("");
    for mut x in part_2 {
        print!("{:?}", x.pop().unwrap());
    }
    println!("\nT: {:?}", start.elapsed());
    
}
