extern crate rand;

mod position;
mod A_star;

use std::io;
use std::io::prelude::*;
use std::time::Instant;

use crate::position::Pos;
use crate::position::permutation_sign;
use crate::A_star::A_star;


fn read_from_console() -> String {
    let stdin = io::stdin();

    let mut s: String = String::from("");
    for i in stdin.lock().lines().take(4) {
        s += &i.unwrap();
        s += "\n";
    }

    s
}

fn call(start: Pos) {
    let start_sign = permutation_sign(&start.to_permutation());
    let end = Pos(0xfedcba9876543210);
    let end_sign = permutation_sign(&end.to_permutation());

    if start == end {
        println!("Nothing to do");
        return;
    }

    if start_sign != end_sign {
        println!("exists solution and I start finding it");
    } else {
        println!("No solution");
        println!("To proof it I start finding solution of 14-15 game")
    }

    let clock = Instant::now();

    let ans = A_star(start);
    let duration = clock.elapsed();

    println!("Solve in {:?}", duration);
    println!("The solution is");

    println!("{}", ans.len());
    for pos in ans {
        println!("{}", pos);
    }
}

fn main() {
    let s = read_from_console(); 
    let pos = Pos::from_string(s);
    call(pos);
}