use std::fmt;
use std::fmt::Display;

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
