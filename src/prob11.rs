#![allow(warnings)]
pub fn main(){
    let height = vec![1,8,6,2,5,4,8,3,7];
    let output = max_area(height);
    println!("{:?}",output);
}

pub fn max_area(height: Vec<i32>) -> i32{
    let mut max_area = 0;
    let mut p = 0;
    let mut q = height.len()-1;
    while p<q{
        if height[p]>height[q]{
            max_area = max_area.max(height[q]*(q-p) as i32);
            q-=1;
        }
        else{
            max_area = max_area.max(height[p]*(q-p) as i32);
            p+=1;
        }
    }
    return max_area;
}

