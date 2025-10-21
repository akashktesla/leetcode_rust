//3346. Maximum Frequency of an Element After Performing Operations I
#![allow(warnings)]
pub fn main(){
    let nums = vec![22,32,74,90];
    let k = 39;
    let num_operations = 4;
    let answer = max_frequency(nums,k,num_operations);
    println!("Answer: {}",answer);
}

pub fn max_frequency(mut nums: Vec<i32>, k: i32, num_operations: i32) -> i32 {
    let len = nums.len() as i32;
    let mut max_frequency:i32 = 1;
    nums.sort();
    for i in 0..nums.len(){
        println!("i:{}",i);
        let mut left_flag = true;
        let mut right_flag = true;
        let mut left:i32 = i as i32;
        let mut right:i32 = i as i32;
        let center_val = nums[i];
        let mut operations = num_operations;

        while (left_flag || right_flag) && operations>0 {
            //checking bounds
            println!("left: {}, right: {}",left,right);
            if left-1 < 0 {
                left_flag = false;
            }

            if right+1 >= len{
                right_flag = false;
            }

            // //calculate left and right operation contains
            let mut left_operation_cost = i32::MAX;
            let mut right_operation_cost = i32::MAX;
            if left_flag{
                left_operation_cost = ((center_val + nums[(left-1) as usize]) as f64 / (k as f64)).ceil() as i32;
                println!("left_operation_cost: {:?}",left_operation_cost);
            }
            if right_flag{
                right_operation_cost = ((nums[(right+1) as usize]-center_val) as f64 / (k as f64)).ceil() as i32;
                println!("right_operation_cost: {:?}",right_operation_cost);
            }
            //decide
            if left_operation_cost<=right_operation_cost && operations >= left_operation_cost{
                //choose left
                left -= 1;
                operations-=left_operation_cost;

            } 
            else if operations >= right_operation_cost{
                right += 1;
                operations -= right_operation_cost;

            }
            else{
                left_flag = false;
                right_flag = false;
            }

            max_frequency = max_frequency.max(((right-left)+1) as i32);

        }
    }
    return max_frequency;
}
