use std::fmt;
use std::fmt::Display;
use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::Reverse;
use std::io;
use std::io::prelude::*;
use std::time::Instant;

#[derive(Hash, Eq, PartialEq, Debug, PartialOrd, Ord, Clone, Copy)]
pub struct Pos(pub u64);

impl Pos {
    pub fn hole_index(&self) -> usize {
        for i in 0..16 {
            if (self.0 >> (4 * i)) & 15 == 15 {
                return i;
            }
        }
        unreachable!()
    }

    pub fn swap(mut self, from: usize, to: usize) -> Pos {
        let to_mask = !(((!(self.0 >> (4 * to))) & 15) << (4 * from));
        self.0 &= to_mask;
        let from_mask = 15 << (4 * to);
        self.0 |= from_mask;
        self
    }

    pub fn manhattan(self, target: Pos) -> usize { 
        let mut res: i8 = 0;

        let mut invers_target: u64 = 0;
        for i in 0..16 {
            invers_target |= i << (4 * ((target.0 >> (4 * i)) & 15));
        }

        for pos in 0..16 {
            let curr_val = (self.0 >> (4 * pos)) & 15;
            let pos_in_target = (invers_target >> (4 * curr_val)) & 15;
            res += ((pos & 3) as i8 - (pos_in_target & 3) as i8).abs() + 
               ((pos >> 2) as i8 - (pos_in_target >> 2) as i8).abs();
        }

        (res as usize) / 2
    }

    pub fn from_string(input: String) -> Pos {
        let vec: Vec<u64> = input
            .split_whitespace()
            .map(|word| word.parse().unwrap())
            .map(|num: u64| if num == 0 { 15 } else { num - 1 })
            .collect();

        Self::from_permutation(vec) 
    }

    pub fn from_permutation(input: Vec<u64>) -> Pos {
        let mut acc: Pos = Pos(0);
        for i in 0..16 {
            acc.0 |= input[i] << (4 * i);
        }
        acc
    }

    pub fn to_permutation(self) -> Vec<u64> {
        let mut res = Vec::new();
        for i in 0..16 {
            let val = (self.0 >> (4 * i)) & 15;
            res.push(if val == 15 { 0 } else { val + 1 });
        }
        res
    }
}

impl Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result: String = String::new();

        for i in 0..4 {
            for j in 0..4 {
                let index = 4 * (i * 4 + j);
                let mut val = (self.0 >> index) & 15;
                if val == 15 {
                    val = 0;
                } else {
                    val += 1;
                }
                result += &val.to_string();
                result += " ";
            }
            result += "\n";
        }
        write!(f, "{}", result)
    }
}

pub enum Dir {
    Up, Right, Down, Left, End,
}

pub struct Neighbors {
    center: Pos,
    curr_dir: Dir,
}

impl Neighbors {
    pub fn new(center: Pos, curr_dir: Dir) -> Neighbors {
        Neighbors {
            center,
            curr_dir,
        }
    }
}

impl Iterator for Neighbors {
    type Item = Pos;

    fn next(&mut self) -> Option<Pos> {
        let hole_ind = self.center.hole_index();

        match self.curr_dir {
            Dir::Up  => {
                self.curr_dir = Dir::Right;
                if hole_ind >> 2 == 3 {
                    self.next()
                } else {
                    Some(self.center.swap(hole_ind, hole_ind + 4))
                }
            }
            Dir::Right => {
                self.curr_dir = Dir::Down;
                if hole_ind & 3 == 0 {
                    self.next()
                } else {
                    Some(self.center.swap(hole_ind, hole_ind - 1))
                }
            }
            Dir::Down => {
                self.curr_dir = Dir::Left;
                if hole_ind >> 2 == 0 {
                    self.next()
                } else {
                    Some(self.center.swap(hole_ind, hole_ind - 4))
                }
            }
            Dir::Left => {
                self.curr_dir = Dir::End;
                if hole_ind & 3 == 3 {
                    self.next()
                } else {
                    Some(self.center.swap( hole_ind, hole_ind + 1))
                }
            }
            _ => None,
        }
    }
}

pub fn neighbors(pos: Pos) -> Neighbors {
    Neighbors::new(pos, Dir::Up)
}


pub fn permutation_sign(perm: &Vec<u64>) -> usize {
    let mut cnt = 0;
    for i in 0..perm.len() {
        for j in i..perm.len() {
            if perm[i] > perm[j] {
                cnt += 1;
            }
        }
        if perm[i] == 0 {
            cnt += i / 4;
        }
    }
    cnt % 2
}

fn make_path(first_middle: Pos, second_middle: Pos, back: &HashMap<Pos, PosData>) -> Vec<Pos> {
    let mut vec1 = vec![first_middle];

    while back[vec1.last().unwrap()].prev != *vec1.last().unwrap() {
        vec1.push(back[vec1.last().unwrap()].prev);
    }

    let mut vec2 = vec![second_middle];

    while back[vec2.last().unwrap()].prev != *vec2.last().unwrap() {
        vec2.push(back[vec2.last().unwrap()].prev);
    }

    if vec2.last().unwrap() > vec1.last().unwrap() {
        vec1.reverse();
        vec1.append(&mut vec2);
        vec1
    } else {
       vec2.reverse();
       vec2.append(&mut vec1);
       vec2
    }
}

#[derive(Clone, Copy)]
struct PosData {
    pub dist: usize,
    pub prev: Pos,
    pub target: Pos, 
}

pub fn A_star(start: Pos) -> Vec<Pos> {
    let mut end: Pos = Pos(0xfedcba9876543210);// 18364758544493064720

    if start == end {
        return vec![start];
    }

    let start_sing = permutation_sign(&start.to_permutation());
    let end_sign = permutation_sign(&end.to_permutation());
    if start_sing != end_sign {
        end = Pos(0xfdecba9876543210);
    }

    if start == end {
        return vec![start];
    }
    

    let mut positions_data: HashMap<Pos, PosData> = HashMap::new(); // dist, target
    positions_data.insert(start, PosData {
        dist: 0,
        prev: start,
        target: end,
    });
    positions_data.insert(end, PosData {
        dist: 0,
        prev: end,
        target: start,
    });

    let mut queue: BinaryHeap<(Reverse<usize>, Pos, Pos)> = BinaryHeap::new();

    for start_neib in neighbors(start) {
        let dist = 1 + start_neib.manhattan(end);
        queue.push((Reverse(dist), start_neib, start));
    }
    
    for end_neib in neighbors(end) {
        let dist = 1 + end_neib.manhattan(start);
        queue.push((Reverse(dist), end_neib, end));
    }

    while let Some((_, pos, prev)) = queue.pop() {

        let prev_data = positions_data[&prev];

        if positions_data.contains_key(&pos) {
            let curr_data = positions_data[&pos];
            if curr_data.target != prev_data.target {
                return make_path(pos, prev, &positions_data);
            }
        } else {
            let dist = prev_data.dist + 1;
            positions_data.insert(pos, PosData {
                dist,
                prev,
                target: prev_data.target,
            });

            for new_pos in neighbors(pos) {
                let new_path_len = dist + 1 + new_pos.manhattan(prev_data.target); 
                if !positions_data.contains_key(&new_pos) {
                    queue.push((Reverse(new_path_len), new_pos, pos));                
                } else {
                    if positions_data[&new_pos].target != prev_data.target {
                        queue.push((Reverse(new_path_len), new_pos, pos));  
                    }
                }
            }
        }
    }
    unreachable!()
}

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

    println!("Решение за {} ходов", ans.len() - 1);
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

