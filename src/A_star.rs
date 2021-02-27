use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

use crate::position::Pos;
use crate::position::neighbors;

fn f(prev: Pos, curr: Pos, target: Pos, results: &HashMap<Pos, usize>) -> usize {
    results[&prev] + 1 + curr.manhattan(target)
}


fn make_path_v2(first_middle: Pos, second_middle: Pos, back: &HashMap<Pos, PosData>) -> Vec<Pos> {
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

fn make_path_v1(first_middle: Pos, second_middle: Pos, back: &HashMap<Pos, Pos>) -> Vec<Pos> {
    let mut vec1 = vec![first_middle];

    while back[vec1.last().unwrap()] != *vec1.last().unwrap() {
        vec1.push(back[vec1.last().unwrap()]);
    }

    let mut vec2 = vec![second_middle];

    while back[vec2.last().unwrap()] != *vec2.last().unwrap() {
        vec2.push(back[vec2.last().unwrap()]);
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
    const END: Pos = Pos(18364758544493064720);

    let mut positions_data: HashMap<Pos, PosData> = HashMap::new(); // dist, target
    positions_data.insert(start, PosData {
        dist: 0,
        prev: start,
        target: END,
    });
    positions_data.insert(END, PosData {
        dist: 0,
        prev: END,
        target: start,
    });

    let mut queue: BinaryHeap<(Reverse<usize>, Pos, Pos)> = BinaryHeap::new();

    for start_neib in neighbors(start) {
        let dist = 1 + start_neib.manhattan(END);
        queue.push((Reverse(dist), start_neib, start));
    }
    
    for end_neib in neighbors(END) {
        let dist = 1 + end_neib.manhattan(start);
        queue.push((Reverse(dist), end_neib, END));
    }

    while let Some((Reverse(dist), pos, prev)) = queue.pop() {

        let prev_data = positions_data[&prev];

        if positions_data.contains_key(&pos) {
            let curr_data = positions_data[&pos];
            if curr_data.target != prev_data.target {
                return make_path_v2(pos, prev, &positions_data);
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