#![allow(warnings)]

pub fn main(){
    let mut a = Option::Some(Box::new(ListNode::new(2)));
    a.as_mut().unwrap().next= Option::Some(Box::new(ListNode::new(4)));
    a.as_mut().unwrap().next.as_mut().unwrap().next = Option::Some(Box::new(ListNode::new(3)));

    let mut b = Option::Some(Box::new(ListNode::new(5)));
    b.as_mut().unwrap().next= Option::Some(Box::new(ListNode::new(6)));
    b.as_mut().unwrap().next.as_mut().unwrap().next = Option::Some(Box::new(ListNode::new(4)));

    let answer = add_two_numbers(a, b);
    println!("{:?}",answer);

}

//Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
    fn push_data(&mut self,val:i32){
        match self.next{
            Some(_)=>{self.next.as_mut().unwrap().push_data(val)}
            None=>{self.next =  Some(Box::new(ListNode::new(val)))}
        }
    }
}
pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut returns = Option::Some(Box::new(ListNode::new(0)));
    let mut flag = true;
    let mut carry = 0;
    let mut a = l1;
    let mut b = l2;
    let mut flag_once = true;
    while flag{
        let mut flag_a = false;
        let mut flag_b = false;
        let t1 = 0;
        let t2 = 0;
        match a.as_ref(){
            Some(s)=>{
                let t1 = s.val;
            }
            None=>{
                let t1 = 0;
                flag_a = true;
            }
        }

        match b.as_ref(){
            Some(s)=>{
                let t2 = s.val;
            }
            None=>{
                let t2 = 0;
                flag_b = true;
            }
        }

        if flag_a && flag_b{
            if carry>0{
                returns.as_mut().unwrap().push_data(carry);
            }
            flag = false;
            break;
        }
        let mut add = t1+t2+carry;
        carry = 0;
        if add >= 10{
            carry = add/10;
            add = add%10;
        }
        println!("add: {}",add);
        if flag_once{
            returns.as_mut().unwrap().val = add;
            flag_once = false;
        }
        else{
            returns.as_mut().unwrap().push_data(add);
        }
        match a{
            Some(s)=>{ a= s.next}
            None=>{
            }
        }
        match b{
            Some(s)=>{ b = s.next}
            None=>{
            }
        }
        a = a.unwrap().next;
        b = b.unwrap().next;
    }
    return returns;

}




