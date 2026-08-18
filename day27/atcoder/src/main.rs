use proconio::input;
// use proconio::marker::Chars;
// use std::collections::HashSet;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    input! {
        q: usize,
    }

    let mut ans = Vec::new();
    let mut heap = BinaryHeap::new();

    for _ in 0..q{
        input! {
            que: usize,
            h: usize,
        }

        if que == 1{
            heap.push(Reverse(h));
        }
        
        else{

            while let Some(&Reverse(top)) = heap.peek(){
                if top <= h{
                    heap.pop();
                }else{
                    break;
                }
            }

        }

        ans.push(heap.len());
    }

    for a in ans{
        println!("{:?}", a);
    }

}