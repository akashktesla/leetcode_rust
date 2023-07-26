#![allow(warnings)]
use std::collections::{HashMap, HashSet};

pub fn main(){
    let nums = vec![1,2,3,1];
    let is_dup = contains_duplicate(nums);
    println!("{}",is_dup);
}

fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut map = HashSet::with_capacity(nums.len());
    for i in nums{
        if map.contains(&i){
            return true;
        }
        else{
            map.insert(i);
        } 
    }
    return false;
}
        

