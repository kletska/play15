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

    if start_sign == end_sign {
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

    println!("This solution need {} steps", ans.len() - 1);
    for pos in ans {
        println!("{}", pos);
    }
}

/*
Вот  неплохой сайт
https://www.onlinegdb.com/online_rust_compiler
Однако, там не хватает времени для слишом сложных примеров
Я приведу пару нетривиальных, но не слишком сложных
Пример ввода:
8 2 3 7
1 11 15 4
5 6 9 10
12 0 13 14
Что бы осознать, насколько сайт - слоупок.
У меня на ноуте 7.6 сек.
Там: 43.1 сек.
1 14 3 4 
5 6 7 8 
9 11 12 0 
13 2 10 15
У меня 0.28 сек.
У них 0.76 сек.
*/



fn main() {
    let s = read_from_console(); 
    let pos = Pos::from_string(s);
    call(pos);
}

