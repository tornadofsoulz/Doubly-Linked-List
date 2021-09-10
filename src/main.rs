use std::fs::OpenOptions;

pub struct List<T> {
    tail: Option<Box<Node<T>>>,
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
    previous: Option<Box<Node<T>>>
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { tail: None, head: None }
    }

    pub fn pushback(&mut self, value: T) {
        let new_node = Box::new(Node {
            data: value,
            next: self.tail.take(),
            previous: self.head.take()//todo чтобы указывал на предыдущую ноду
        });
        self.tail = Some(new_node);
    }

    pub fn rm(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }
}

fn main() {}