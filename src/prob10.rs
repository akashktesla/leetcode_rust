#![allow(warnings)]


pub fn main(){

    let s = "aaaa".to_string();
    let p = "a".to_string();
    println!("{:?}",is_match(s,p));


}

fn is_match(s:String,p:String)->bool{
    let pattern_list:Vec<char> = p.chars().collect();
    let mut pattern_pointer:i8 = 0;
    let mut pattern_class:i8 = 0;
    let pattern_length = pattern_list.len();
    let mut returns:bool = true;
    let mut starflag:bool = false;
    let mut starchar:char = ' ';
    for i in s.chars(){
        let pattern = *pattern_list.get(pattern_pointer as usize).unwrap();
        pattern_class = char2class(pattern);
        match pattern_class{
            0=>{
                //matching characters
                if i != pattern{
                    returns = false
                }
                pattern_pointer+=1;
            },
            1=>{
                //matching .
                pattern_pointer+=1
            }
            2=>{
                //matching *

                if pattern_pointer>0{
                let previous_pattern = pattern_list[(pattern_pointer-1) as usize];
                starflag = true;
                starchar = previous_pattern;
                if pattern_pointer < pattern_length as i8{
                    let future_pattern = pattern_list[(pattern_pointer+1) as usize];
                }
                else{
                    let future_pattern = None;
                }
                if i!=previous_pattern{
                    returns = false;
                }
                else if i==future_pattern{
                    pattern_pointer+=1;

                }
                }
                else{
                    return false 
                }

            }
            _=>{

            }
        }
        
        println!("{:?}",i);

    }

    return returns
}

fn char2class(input:char)->i8{
    match input{
        '.'=>return 1,
        '*'=>return 2,
        _=>return 0
    }
}
