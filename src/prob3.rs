use std::collections::HashSet;
//Longest sub string
//Dynamic Sliding Window
pub fn main(){
    let s = "bbbb".to_string();
    let answer = length_of_longest_substring(s);
    println!("Answer: {}",answer);

}

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut left = 0;
    let mut right = 0;
    let mut max_len:i32 = 0;
    let len = s.len();
    let mut set = HashSet::new();
    let char_list:Vec<char> = s.chars().collect();
    while right < len{
        while set.contains(&char_list[right]){
            set.remove(&char_list[left]);
            left +=1;
        }
        set.insert(char_list[right]);
        right +=1;
        max_len = max_len.max((right-left) as i32);
    }
    return max_len;
}

