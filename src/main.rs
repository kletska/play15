mod position;
mod A_star;

use std::io;
use std::io::prelude::*;
use std::time::Instant;

use crate::position::Pos;
use crate::A_star::A_star;

fn main() {
    let stdin = io::stdin();

    let mut s: String = String::from("");
    for i in stdin.lock().lines().take(4) {
        s += &i.unwrap();
        s += "\n";
    }

    println!("Camputations started");
    let clock = Instant::now();
    let start = Pos::from_string(s);     
    let ans = A_star(start);
    let duration = clock.elapsed();
    println!("Solve in {:?} seconds", duration);
    println!("{}", ans.len());
    println!("{:x?}", ans); 
}