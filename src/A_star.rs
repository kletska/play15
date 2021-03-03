use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

use crate::position::Pos;
use crate::position::neighbors;
use crate::position::permutation_sign;

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