#![allow(warnings)]
//import
use std::collections::HashMap;

pub fn main(){
    let s = String::from("anagram");
    let t = String::from("nagaram");
    let is_anagram = is_anagram(s, t);
    println!("{}",is_anagram);
}

pub fn is_anagram(s: String, t: String) -> bool {
    let mut shm:HashMap<char,i32> = HashMap::new();
    let mut thm:HashMap<char,i32> = HashMap::new();
    for i in s.chars(){
        match shm.get_mut(&i){
            Some(mut s)=>{
                *s = *s+1;
            }
            None=>{
                shm.insert(i,1);
            }
        }
    }
    for i in t.chars(){
        match thm.get_mut(&i){
            Some(mut s)=>{
                *s = *s+1;
            }
            None=>{
                thm.insert(i,1);
            }
        }
    }
    if shm==thm{
        return true;
    }
    return false;
}

fn is_anagram2(s: String, t: String) -> bool {
    let mut a:Vec<char> = s.chars().collect();
    a.sort();
    let mut b:Vec<char> = s.chars().collect();
    b.sort();
    if a==b{
        return true;
    }
    return false;
}












