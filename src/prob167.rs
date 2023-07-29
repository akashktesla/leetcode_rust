//two pointers
//nums is sorted
#![allow(warnings)]
pub fn main(){
    let numbers = vec![2,7,11,15] ;
    let target = 9;
    let a = two_sum(numbers,target);
    println!("{:?}",a);
}

pub fn two_sum(numbers: Vec<i32>,target:i32)->Vec<i32>{
    let mut i:usize = 0;
    let mut j:usize = numbers.len()-1;
    while numbers[i]+numbers[j]!=target{
        if numbers[i]+numbers[j]>target{
            j-=1;
        }
        else{
            i+=1;
        }
    }
    return vec![i as i32,j as i32]
}
