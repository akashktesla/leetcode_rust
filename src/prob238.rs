#![allow(warnings)]
pub fn main(){
    let nums = vec![1,2,3,4];
    let nums = vec![-1,1,0,-3,3];
    let output = product_except_self(nums);
    println!("output: {:?}",output);

}

fn product_except_self(nums:Vec<i32>)->Vec<i32>{
    let mut isg1 = false;
    let mut isg0 = false;
    let mut product = 1;
    let mut zcount = 0;
    for i in nums.iter(){
        if *i==0{
            zcount+=1;
        }
        else{
            product *= i;
        }
        if zcount >0{
            isg0 = true;
            if zcount >1{
                isg1 = true;
            }
        }
    }
    if isg1{
        return (0..nums.len()).into_iter().map(|x|return 0).collect();
    }
    if isg0{
        return nums.into_iter().map(|x|{
            if x ==0 {
                return product;
            }
            else{
                return 0;
            }
        })
        .collect();
    }
    return nums.into_iter().map(|x|{
        if x ==0 {
            return product;
        }
        else{
            return product/x;
        }
    })
    .collect();
}


