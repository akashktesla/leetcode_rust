#![allow(warnings)]

pub fn main(){
    let mut mstack = MinStack::new();
    mstack.push(-2);
    mstack.push(0);
    mstack.push(-3);
    mstack.push(-4);
    mstack.push(-1);
    println!("{:?}",mstack.stack);
    // mstack.get_min();
    // mstack.pop();
    // mstack.top();
    // mstack.get_min();
}
struct MinStack {
    stack: Vec<(i32, i32)>,
}

impl MinStack {

    fn new() -> Self {
        MinStack { stack: vec![] }
    }
    
    fn push(&mut self, val: i32) {
        let min = val
            .min(
                *self
                .stack
                .last()
                .map(|(value, min)| min)
                .unwrap_or(&val)
            );
        self.stack.push((val, min));
    }
    
    fn pop(&mut self) {
        self.stack.pop();
    }
    
    fn top(&mut self) -> i32 {
        self.stack[self.stack.len() - 1].0
   }
    
    fn get_min(&self) -> i32 {
        self.stack[self.stack.len() - 1].1
    }
}
