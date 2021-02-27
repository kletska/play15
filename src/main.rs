extern crate rand;

mod position;
mod A_star;

use std::io;
use std::io::prelude::*;
use std::time::Instant;

use crate::position::Pos;
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

fn call(start: Pos) -> Vec<Pos> {
    println!("Camputations started");
    let clock = Instant::now();
    let ans = A_star(start);
    let duration = clock.elapsed();
    println!("Solve in {:?}", duration);
    println!("{}", ans.len());
    println!("{:x?}", ans); 
    ans
}

fn main() {
    let s = read_from_console(); 
    let pos = Pos::from_string(s);
    call(pos);
}