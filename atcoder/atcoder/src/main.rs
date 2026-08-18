use proconio::input;
use proconio::marker::Chars;
// use std::collections::HashSet;
// use std::collections::BinaryHeap;
// use std::cmp::Reverse;

fn main() {
    input!{
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    for si in s{
        println!{"{:?}", si};
    }
}