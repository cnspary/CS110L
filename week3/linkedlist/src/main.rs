use std::fmt;

struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

struct Node<T> {
    value: T,
    next : Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(value: T, next: Option<Box<Node<T>>>) -> Node<T> {
        Node { value, next }
    }
}

impl<T: Copy> Clone for Node<T> {
    fn clone(&self) -> Node<T> {
        Node::new(self.value, self.next.clone())
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList { head: None, size: 0 }
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn empty(&self) -> bool {
        self.size == 0
    }

    pub fn push(&mut self, val: T) {
        let new_node: Box<Node<T>> = Box::new(Node::new(val, self.head.take()));
        self.head = Some(new_node);
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        let node: Box<Node<T>> = self.head.take()?;
        self.head = node.next;
        self.size -= 1;
        Some(node.value)
    }
}

impl<T: Copy> Clone for LinkedList<T> {
    fn clone(&self) -> LinkedList<T> {
        let mut list = LinkedList::new();
        list.head = self.head.clone();
        list.size = self.size;

        list
    }
}

impl<T: PartialEq> PartialEq for LinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size && {
            let mut p = &self.head;
            let mut q = &other.head;

            while let (Some(_p), Some(_q)) = (p, q) {
                if _p.value == _q.value {
                    p = &_p.next;
                    q = &_q.next;
                } else {
                    return false;
                }
            }
            true
        }
    }
}

pub trait ComputeNorm {
    fn compute_norm(&self) -> f64 {
        0.0
    }
}

impl ComputeNorm for LinkedList<f64> {
    fn compute_norm(&self) -> f64 {
        let mut cur = &self.head;
        let mut norm: f64 = 0.0;
        while let Some(_p) = cur {
            norm += _p.value * _p.value;
            cur = &_p.next;
        }

        norm.sqrt()
    }
}

struct LinkedListIter<'a, T> {
    current: &'a Option<Box<Node<T>>>,
}

impl<T: Clone> Iterator for LinkedListIter<'_, T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        match self.current {
            Some(node) => {
                self.current = &node.next;
                Some(node.value.clone())
            },
            None => None
        }
    }
}

impl<'a, T: Clone> IntoIterator for &'a LinkedList<T> {
    type Item = T;
    type IntoIter = LinkedListIter<'a, T>;

    fn into_iter(self) -> LinkedListIter<'a, T> {
        LinkedListIter { current: &self.head }
    }
}


impl<T: std::fmt::Display> fmt::Display for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut current: &Option<Box<Node<T>>> = &self.head;
        let mut result = String::new();
        loop {
            match current {
                Some(node) => {
                    result = format!("{} {}", result, node.value);
                    current = &node.next;
                },
                None => break,
            }
        }
        write!(f, "{}", result)
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut cur = self.head.take();
        while let Some(mut node) = cur {
            cur = node.next.take();
        }
    }
}

fn main() {
    // u32 LinkedList
    {   
        println!("***************** Create U32 LinkedList *****************");
        let mut u32_list = LinkedList::new();
        for i in 1..=10 {
            u32_list.push(i);
        }
        println!("LinkedList (size = {}): {}", u32_list.get_size(), u32_list);
        while !u32_list.empty() {
            u32_list.pop();
        }
        println!("LinkedList (size = {}): {}", u32_list.get_size(), u32_list);
    }

    // String LinkedList
    {
        println!("\n***************** Create String LinkedList *****************");
        let mut string_list = LinkedList::new();
        for _ in 1..=10 {
            string_list.push(String::from("abcd"));
        }
        println!("LinkedList (size = {}): {}", string_list.get_size(), string_list);
        while !string_list.empty() {
            string_list.pop();
        }
        println!("LinkedList (size = {}): {}", string_list.get_size(), string_list);
    }

    // for clone
    {
        println!("\n***************** Test Clone *****************");
        let mut list1 =  LinkedList::new();
        for i in 1..=10 {
            list1.push(i);
        }
        let mut list2 = list1.clone();
        println!("LinkedList (size = {}): {}", list1.get_size(), list1);
        println!("LinkedList (size = {}): {}", list2.get_size(), list2);
        println!("list2 pop {}", list2.pop().unwrap());
        println!("list2 pop {}", list2.pop().unwrap());
        println!("LinkedList (size = {}): {}", list1.get_size(), list1);
        println!("LinkedList (size = {}): {}", list2.get_size(), list2);

    }

    // for PartialEq
    {
        println!("\n***************** Test PartialEq *****************");
        let mut list1 =  LinkedList::new();
        for i in 1..=10 {
            list1.push(i);
        }
        let mut list2 = list1.clone();
        println!("LinkedList (size = {}): {}", list1.get_size(), list1);
        println!("LinkedList (size = {}): {}", list2.get_size(), list2);
        println!("list1 == list2: {}", list1.eq(&list2));
        list2.pop();
        println!("LinkedList (size = {}): {}", list1.get_size(), list1);
        println!("LinkedList (size = {}): {}", list2.get_size(), list2);
        println!("list1 == list2: {}", list1.eq(&list2));
        list2.push(100);
        println!("LinkedList (size = {}): {}", list1.get_size(), list1);
        println!("LinkedList (size = {}): {}", list2.get_size(), list2);
        println!("list1 == list2: {}", list1.eq(&list2));
        list2.pop();
        list2.push(10);
        println!("LinkedList (size = {}): {}", list1.get_size(), list1);
        println!("LinkedList (size = {}): {}", list2.get_size(), list2);
        println!("list1 == list2: {}", list1.eq(&list2));
    }

    // for ComputeNorm
    {
        println!("\n***************** Test ComputeNorm *****************");
        let mut list: LinkedList<f64> =  LinkedList::new();
        for i in 1..=10 {
            list.push(i as f64);
        }
        println!("list compute_norm = {}", list.compute_norm());
    }

    // for Iterator
    {
        let mut list: LinkedList<f64> =  LinkedList::new();
        for i in 1..=10 {
            list.push(i as f64);
        }
        for e in list.into_iter() {
            println!("{:?}", e);
        }
    }
}
