//Logest Consecutive sequence
#![allow(warnings)]
use std::collections::HashSet;
pub fn main(){
    let nums = vec![100,4,200,1,3,2];
    let nums = vec![0,3,7,2,5,8,4,6,0,1];
    let output = longest_consecutive(nums);
    println!("{}",output);
}

pub fn longest_consecutive(mut nums:Vec<i32>)->i32{
    if nums.len()<1{
        return 0
    }
    nums.sort();
    nums.dedup();
    let mut longest = 0;
    let mut count = 1;
    for i in  1..(nums.len()){
        if nums[i-1] == nums[i]-1{
            count += 1;
            if count > longest{
                longest = count;
            }
        }
        else{
            count = 1;
        }
    }
    return longest;
}
