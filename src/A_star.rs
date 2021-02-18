use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

use crate::position::Pos;
use crate::position::neighbors;

fn f(prev: Pos, curr: Pos, target: Pos, results: &HashMap<Pos, usize>) -> usize {
    results[&prev] + 1 + curr.manhattan(target)
}

fn make_path(first_middle: Pos, second_middle: Pos, back: &HashMap<Pos, Pos>) -> Vec<Pos> {
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

pub fn A_star(start: Pos) -> Vec<Pos> {
    const END: Pos = Pos(18364758544493064720);

    let mut results: HashMap<Pos, usize> = HashMap::new();
    results.insert(start, 0);
    results.insert(END, 0);

    let mut back: HashMap<Pos, Pos> = HashMap::new();
    back.insert(start, start);
    back.insert(END, END);

    let mut target: HashMap<Pos, Pos> = HashMap::new();
    target.insert(start, END);
    target.insert(END, start);

    let mut queue: BinaryHeap<(Reverse<usize>, Pos)> = BinaryHeap::new();
    queue.push((Reverse(0), start));
    queue.push((Reverse(0), END));

    while let Some((_, curr)) = queue.pop() {
        for n in neighbors(curr) {
            if !results.contains_key(&n) { 
                results.insert(n, results[&curr] + 1);

                back.insert(n, curr);
                
                target.insert(n, target[&curr]);

                queue.push((Reverse(f(curr, n, target[&curr], &results)), n));
            } else {
                if target[&n] != target[&curr] {
                    return make_path(n, curr, &back);
                }
            }
        }
    }
    vec![]
}