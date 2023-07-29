//three sum must add up to zero
#![allow(warnings)]
use std::collections::{HashSet, HashMap};

pub fn main(){
    let nums = vec![-1,0,1,2,-1,-4];
    let a = three_sum(nums);
    println!("{:?}",a);
}

pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>>
{
    nums.sort();
    let mut triplets : Vec<Vec<i32>> = Vec::new();
    for i in 0..nums.len(){
        if nums[i] > 0 {
            break; 
        }
        if i > 0 && nums[i] == nums[i-1] { 
            continue; 
        }
        for result in two_sum_sorted(&nums, i){
            triplets.push(result);
        }
    }
    return triplets;
}

fn two_sum_sorted(nums: &[i32], i: usize) -> Vec<Vec<i32>>{       
    let mut j = i+1;
    let mut k = nums.len()-1;
    let mut results = Vec::new();
    while j < k
    {
        let sum = nums[i] + nums[j] + nums[k];
        if sum < 0{
            j += 1; 
        }
        else if sum > 0 { 
            k -= 1; 
        }
        else{
            let result = vec![nums[i], nums[j], nums[k]];
            results.push(result);
            j += 1;
            while j < nums.len() && nums[j] == nums[j-1] {
                j += 1; 
            }
        }
    }
    return results;
}
