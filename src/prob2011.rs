pub fn main(){
    let operations = vec!["--X".to_string(),"X++".to_string(),"X++".to_string()];
    let answer = final_value_after_operations(operations);
    println!("Answer: {:?}",answer);


}

fn final_value_after_operations(operations: Vec<String>) -> i32 {
    let mut returns = 0;
    for i in operations{
        match i.as_str(){
            "--X"|"X--" =>{
                returns -= 1

            }
            "++X"|"X++" =>{
                returns += 1

            }
            _ =>{}
        }

    }
    return returns

}
