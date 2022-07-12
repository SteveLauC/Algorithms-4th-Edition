use std::fmt::Debug;

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug, Default)]
struct Node<T: Default + Debug> {
    item: T,
    next: Link<T>,
}

#[derive(Debug)]
pub struct LinkedStack<T: Default + Debug> {
    head: Link<T>,
    n: usize,
}

impl<T: Default + Debug> Node<T> {
    fn new(item: T) -> Self {
        Self { item, next: None }
    }
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

impl<T: Default + Debug> LinkedStack<T> {
    pub fn new() -> Self {
        Self { head: None, n: 0 }
    }
    pub fn push(&mut self, item: T) {
        self.n += 1;
        let mut new_node: Node<T> = Node::new(item);
        if self.head.is_none() {
            self.head = Some(Box::new(new_node));
        } else {
            let old_head: Link<T> = self.head.take();
            new_node.next = old_head;
            self.head = Some(Box::new(new_node));
        }
    }

    pub fn pop(&mut self) {
        if let Some(old_node) = self.head.take() {
            self.n -= 1;
            self.head = old_node.next;
        }
    }

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    pub fn size(&self) -> usize {
        self.n
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

fn main() {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut stack: LinkedStack<i32> = LinkedStack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        stack.pop();

        let mut iter: Iter<'_, i32> = stack.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);

        let mut iter_mut: IterMut<'_, i32> = stack.iter_mut();
        assert_eq!(iter_mut.next(), Some(&mut 3));
        assert_eq!(iter_mut.next(), Some(&mut 2));
        assert_eq!(iter_mut.next(), Some(&mut 1));
        assert_eq!(iter_mut.next(), None);
    }
}
