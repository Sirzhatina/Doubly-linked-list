use std::cell::{RefCell, Ref};
use std::fmt::Display;
use std::rc::Rc;

type RCell<T> = Rc<RefCell<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    next: Option<RCell<T>>,
    prev: Option<RCell<T>>,
    value: T
}

#[derive(Debug)]
pub struct List<T: Display> {
    head: Option<RCell<T>>,
    tail: Option<RCell<T>>
}

impl<T: Display> List<T> {
    pub fn new() -> Self {
        List { head: None, tail: None }
    }

    pub fn append(&mut self, val: T) {
        match &self.tail {
            Some(current) => {
                let new_node = Rc::new(RefCell::new(Node {
                    next: None,
                    prev: Some(Rc::clone(&current)),
                    value: val
                }));
                current.borrow_mut().next = Some(Rc::clone(&new_node));
                self.tail = Some(Rc::clone(&new_node));
            },
            None => {
                let new_node = Rc::new(RefCell::new(Node {
                    next: None,
                    prev: None,
                    value: val
                }));
                self.head = Some(Rc::clone(&new_node));
                self.tail = Some(Rc::clone(&new_node));
            }
        }
    }

    pub fn prepend(&mut self, val: T) {
        match &self.head {
            Some(n) => {
                let new_node = Rc::new(RefCell::new(Node {
                    next: Some(Rc::clone(n)),
                    prev: None,
                    value: val
                }));
                n.borrow_mut().prev = Some(Rc::clone(&new_node));
                self.head = Some(new_node);
            },
            None => {
                let new_node = Rc::new(RefCell::new(Node {
                    next: None,
                    prev: None,
                    value: val
                }));
                self.head = Some(Rc::clone(&new_node));
                self.tail = Some(new_node);
            }
        }
    }

    pub fn print_values(&self, sep: &str) {
        let mut current = self.head.as_ref().map(|r| Rc::clone(r));

        let mut first = true;
        while let Some(n) = current {
            if !first {
                print!("{}", sep);
            }
            first = false;

            print!("{}", n.borrow().value);
            current = n.borrow().next.as_ref().map(|r| Rc::clone(r));
            
        }
        println!();
    }
}