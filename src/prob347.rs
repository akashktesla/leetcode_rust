#![allow(warnings)]
//Top K frequent elements
use std::collections::{HashMap,BinaryHeap};
pub fn main(){

 // let nums = vec![1,1,1,2,2,3];
 let nums = vec![-1,-1];
 let k = 1;
 let a = top_k_frequent(nums, k);
 println!("{:?}",a);
}

fn top_k_frequent(nums:Vec<i32>,k:i32)->Vec<i32>{
    let mut returns = Vec::new();
    let mut map:HashMap<i32,i32> = HashMap::new();
    for i in nums{
        match map.get_mut(&i){
            Some(s)=>{
                *s+=1
            }
            None=>{
                map.insert(i,1);
            }
        }
    }
    println!("{:?}",map);
    let mut heap = BinaryHeap::new();
    for (k,v) in map{
        heap.push((v,k));
    }
    for i in 0..k{
        returns.push(heap.pop().unwrap().1);
    }
    return returns;
}
