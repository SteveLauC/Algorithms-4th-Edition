//! For the reason that the size of `array` can not be determined at runtime, we
//! use `Vec` instead


struct ResizingVecStack<I: Clone> {
    a: Vec<I>,
}

impl<I: Clone> ResizingVecStack<I> {
    pub fn new() -> Self {
        Self { a: Vec::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.a.is_empty()
    }

    pub fn size(&self) -> usize {
        self.a.len()
    }

    // pub fn resize(&mut self, cap: usize) {
    //     self.a.reserve(cap);
    // }

    pub fn push(&mut self, item: I) {
        self.a.push(item);
    }

    pub fn pop(&mut self) -> I {
        if self.is_empty() {
            panic!("stack is empty: can not pop");
        }
        let item: I = self.a[self.a.len() - 1].clone();
        self.a.truncate(self.a.len() - 1);
        item
    }

    pub fn peek(&self) -> I {
        if self.is_empty() {
            panic!("stack is empty: can not peek");
        }
        self.a[self.a.len() - 1].clone()
    }
}

impl<I: Clone> Iterator for ResizingVecStack<I> {
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.is_empty() {
            Some(self.pop())
        } else {
            None
        }
    }
}
fn main() {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn iter_test() {
        let mut stack: ResizingVecStack<i32> = ResizingVecStack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(Some(3), stack.next());
        assert_eq!(Some(2), stack.next());
        assert_eq!(Some(1), stack.next());
        assert_eq!(None, stack.next());
    }

    #[test]
    fn peek_test() {
        let stack: ResizingVecStack<i32> = ResizingVecStack { a: vec![1, 2] };
        let orig_len: usize = stack.size();
        assert_eq!(stack.peek(), 2);
        assert_eq!(orig_len, stack.size());
    }

    #[test]
    fn pop_test() {
        let mut stack: ResizingVecStack<i32> = ResizingVecStack { a: vec![1, 2] };
        let orig_len: usize = stack.size();
        assert_eq!(stack.pop(), 2);
        assert_eq!(orig_len-1, stack.size());
    }
}

