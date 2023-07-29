#![allow(warnings)]
pub fn main(){
    let s = "A man, a plan, a canal: Panama".to_string();
    is_palindrome(s);
}

pub fn is_palindrome(s: String)->bool{
    let a:String = s.chars()
        .filter(|s|s.is_alphanumeric())
        .map(|s| s.to_ascii_lowercase())
        .collect();
    println!("a:{}",a);
    let r:String = a.chars().rev().collect();
    if a==r{
        return true;
    }
    return false;
}
