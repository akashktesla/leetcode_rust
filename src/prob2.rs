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
}
pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut returns = Option::Some(Box::new(ListNode::new(0)));
    let mut flag = true;
    let mut carry = 0;
    let mut a = l1;
    let mut b = l2;
    let mut flag_once = true;
    let mut next_ref:Option<Box<ListNode>>;
    while flag{
        let mut flag_a = false;
        let mut flag_b = false;
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
                returns = Some(Box::new(ListNode{
                    val:carry,
                    next:returns.take(),
                }));
            }
            flag = false;
            break;
        }
        let t1 = a.as_ref().unwrap().val;
        let t2 = b.as_ref().unwrap().val;
        let mut add = t1+t2+carry;
        carry = 0;
        if add >= 10{
            carry = add/10;
            add = add%10;
        }
        println!("add: {}",add);
        returns = Some(Box::new(ListNode{
            val:add,
            next:returns.take(),
        }));
        a = a.unwrap().next;
        b = b.unwrap().next;
    }
    return returns;

}




