use std::fmt::Debug;

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug, Default)]
struct Node<T: Default + Debug> {
    item: T,
    next: Link<T>,
}

#[derive(Debug)]
pub struct LinkedBag<T: Default + Debug> {
    first: Link<T>,
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

impl<T: Default + Debug> LinkedBag<T> {
    pub fn new() -> Self {
        Self { first: None, n: 0 }
    }
    pub fn add(&mut self, item: T) {
        self.n += 1;
        let mut new_node: Node<T> = Node::new(item);
        if self.first.is_none() {
            self.first = Some(Box::new(new_node));
        } else {
            let old_head: Link<T> = self.first.take();
            new_node.next = old_head;
            self.first = Some(Box::new(new_node));
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
            next: self.first.as_deref(),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.first.as_deref_mut(),
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
        let mut stack: LinkedBag<i32> = LinkedBag::new();
        stack.add(1);
        stack.add(2);
        stack.add(3);

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
