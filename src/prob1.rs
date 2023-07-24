#![allow(warnings)]
use std::collections::{HashMap, HashSet};
use std::ops::Index;

//two sum
pub fn main(){
    let nums = vec![2,7,11,15];
    let target = 9;
    let answer = two_sum(nums,target);
    println!("answer: {:?}",answer);
}

 pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
     let mut returns = Vec::new();
     let nums_set:HashSet<i32> = nums.clone().into_iter().collect();
     for (i,val) in nums.iter().enumerate(){
         let temp = target - val;
         if nums_set.contains(&temp){
             match nums.iter().position(|&x|x==temp){
                 Some(s)=>{
                     if s != i{
                         returns.push(i as i32);
                         returns.push(s as i32);
                         break;
                     } 
                 }
                 None=>{}
             }
         }
     }
     return returns;
 }

