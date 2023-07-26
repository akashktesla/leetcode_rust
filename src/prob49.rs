#![allow(warnings)]
use std::collections::HashMap;

pub fn main(){
    let strs = vec!["eat".to_string(),"tea".to_string(),"tan".to_string(),"ate".to_string(),"nat".to_string(),"bat".to_string()];
    let a = group_anagrams(strs);
    println!("{:?}",a);

}

pub fn group_anagrams(strs: Vec<String>) ->Vec<Vec<String>>{
    let mut hm:HashMap<String,Vec<String>> = HashMap::new();
    for i in strs{
        let mut thm:HashMap<char,i32> = HashMap::new();
        let mut v:Vec<char> = i.chars().collect();
        v.sort();
        //use this or use Vec<char> as a key (no diff)
        let key = String::from_iter(v.iter());
        match hm.get_mut(&key){
            Some(s)=>{
                s.push(i);
            }
            None=>{
                hm.insert(key,vec![i]);
            }
        }
    }
    return  hm.values().cloned().collect();
}
