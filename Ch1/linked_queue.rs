use std::fmt::{self, Debug, Display};

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug, Default)]
struct Node<T: Default + Debug> {
    item: T,
    next: Link<T>,
}

#[derive(Debug)]
pub struct LinkedQueue<T: Default + Debug> {
    head: Link<T>,
    n: usize,
}

pub struct Iter<'a, T>
where
    T: Debug + Default,
{
    next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T>
where
    T: Debug + Default,
{
    next: Option<&'a mut Node<T>>,
}
impl<T: Default + Debug> Node<T> {
    fn new(item: T) -> Self {
        Self { item, next: None }
    }
}

impl<T: Default + Debug> LinkedQueue<T> {
    pub fn new() -> Self {
        Self { head: None, n: 0 }
    }

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    pub fn size(&self) -> usize {
        self.n
    }

    pub fn enqueue(&mut self, item: T) {
        if self.n == 0 {
            self.head = Some(Box::new(Node::new(item)));
        } else {
            let mut p: Option<&mut Node<T>> = self.head.as_deref_mut();
            for _ in 0..self.n - 1 {
                p = p.unwrap().next.as_deref_mut();
            }
            p.unwrap().next = Some(Box::new(Node::new(item)));
        }

        self.n += 1;
    }

    pub fn dequeue(&mut self) {
        if let Some(old_head) = self.head.take() {
            self.n -= 1;
            self.head = old_head.next;
        }
    }

    pub fn peek(&self) -> Option<&T> {
        if self.n == 0 {
            None
        } else {
            let mut p: Option<&Node<T>> = self.head.as_deref();
            for _ in 0..self.n - 1 {
                p = p.unwrap().next.as_deref();
            }
            Some(&p.unwrap().item)
        }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T>
where
    T: Debug + Default,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|n: &Node<T>| {
            self.next = n.next.as_deref();
            &n.item
        })
    }
}

impl<'a, T> Iterator for IterMut<'a, T>
where
    T: Debug + Default,
{
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|n: &mut Node<T>| {
            self.next = n.next.as_deref_mut();
            &mut n.item
        })
    }
}

impl<T> Display for LinkedQueue<T>
where
    T: Debug + Default,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let iter: Iter<T> = self.iter();
        let mut s: String = String::new();
        iter.for_each(|item: &T| s.push_str(format!("{:?} ", item).as_str()));
        write!(f, "LinkedQueue: {}", s)
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut q: LinkedQueue<i32> = LinkedQueue::new();

        q.enqueue(1);
        q.enqueue(2);
        q.enqueue(3);
        assert_eq!("LinkedQueue: 1 2 3 ", q.to_string());
        q.dequeue();
        q.dequeue();
        q.dequeue();

        assert_eq!("LinkedQueue: ", q.to_string());
    }
}
