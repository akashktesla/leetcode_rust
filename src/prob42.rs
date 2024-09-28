#![allow(warnings)]
use std::cmp::{max,min};

pub fn main(){
    // let height = vec![0,1,0,2,1,0,1,3,2,1,2,1];
    let height = vec![5,4,1,2];
    println!("{:?}",trap(height));

}

pub fn trap(height: Vec<i32>) -> i32 {
    let len = height.len();
    if len <= 2 {
        return 0;
    }

    let mut left = 0;
    let mut right = len - 1;
    let mut left_max = height[left];
    let mut right_max = height[right];
    let mut water_trapped = 0;

    while left < right {
        if height[left] < height[right] {
            // Water trapped depends on left_max
            if height[left] >= left_max {
                left_max = height[left];
            } else {
                water_trapped += left_max - height[left];
            }
            left += 1;
        } else {
            // Water trapped depends on right_max
            if height[right] >= right_max {
                right_max = height[right];
            } else {
                water_trapped += right_max - height[right];
            }
            right -= 1;
        }
    }

    water_trapped
}
