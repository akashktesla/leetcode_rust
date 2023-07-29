#![allow(warnings)]

pub fn main(){
    let s = "{}()[]".to_string();
    let s = "([)]".to_string();
    let s = "{[]}".to_string();
    let s = "lkj".to_string();
    let s = "]".to_string();
    let s = "(){}{}".to_string();
    let a = is_valid(s);
    println!("{}",a);
}

pub fn is_valid(s:String)->bool{
    let mut stack = Vec::new();
    for i in s.chars(){
        if i =='(' || i=='{'||i=='['{
            stack.push(i);
        }
        else if i ==')' || i=='}'||i==']'{
            println!("{:?}",stack);
            match stack.pop(){
                Some('{')=> {
                    if !(i=='}'){
                        return false
                    }},
                Some('(')=> {
                    if !(i==')'){
                        return false
                    }},
                Some('[')=> {
                    if !(i==']'){
                        return false
                    }},
                None=> return false,
                _=>(),
            }
            }
        }

    return stack.is_empty();
}

