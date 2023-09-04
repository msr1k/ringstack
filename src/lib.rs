//! # ringstack
//! 
//! [RingStack] is a tiny stack implementation which uses circular buffer.
//!
//! Since [RingStack] is constructed upon a circular buffer,
//! the oldest item automatically dropped as you [push][RingStack::push()]
//! when the number of items has already reached its limit.
//!
//! And it supports [RingStack::iter()] method which returns `Iterator<&Option<T>>`.
//! It provides items one by one with historical order, latest to oldest.
//!
//! Though [RingStack] currently uses [Vec] as its internals,
//! once it allocates at the timing of [new][RingStack::iter()]
//! then additional allocation never happends.
//!
//! ## Examples
//! 
//! ```rust
//! use ringstack::RingStack;
//!
//! let mut s = RingStack::<i32, 3>::new();
//! assert_eq!(s.peek(), None);
//!
//! s.push(1);
//! s.push(2);
//! assert_eq!(s.peek(), Some(&2));
//! assert_eq!(s.pop(), Some(2));
//!
//! s.push(3);
//! s.push(4);
//! let v: Vec<Option<i32>> = s.iter().map(|e| e.clone()).collect();
//! assert_eq!(v, vec![Some(4), Some(3), Some(1)]);
//!
//! s.push(5);
//! let v: Vec<Option<i32>> = s.iter().map(|e| e.clone()).collect();
//! assert_eq!(v, vec![Some(5), Some(4), Some(3)]);
//!
//! assert_eq!(s.pop(), Some(5));
//! assert_eq!(s.pop(), Some(4));
//! assert_eq!(s.pop(), Some(3));
//! assert_eq!(s.pop(), None);
//! ```
//! 
use std::iter::Iterator;

#[derive(Debug)]
pub struct RingStack<T, const N: usize> {
    buffer: Vec<Option<T>>,
    index: usize,
    len: usize,
}

impl<T, const N: usize> RingStack<T, N> {

    pub fn new() -> Self {
        let index = 0;
        let len = 0;
        let mut buffer = Vec::with_capacity(N);
        (0..N).for_each(|_| buffer.push(None));

        RingStack { buffer, index, len }
    }

    pub fn push(&mut self, val: T) -> () {
        self.index = match self.index >= N - 1 { true => 0, false => self.index + 1 };
        self.len = if self.len == N { self.len } else { self.len + 1 };
        std::mem::swap(&mut self.buffer[self.index], &mut Some(val));
    }

    pub fn pop(&mut self) -> Option<T> {
        let mut v = None;
        std::mem::swap(&mut self.buffer[self.index], &mut v);
        self.index = match self.index == 0 { true => N - 1, false => self.index - 1 };
        self.len = if self.len == 0 { self.len } else { self.len - 1 };
        v
    }

    pub fn peek(&self) -> Option<&T> {
        self.buffer[self.index].as_ref()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn iter(&self) -> impl Iterator<Item=&Option<T>> {
        let a = self.buffer[0..=self.index].iter().rev();
        let b = self.buffer[(self.index + 1)..N].iter().rev();
        a.chain(b)
    }
}

#[cfg(test)]
mod t {
    use super::*;

    #[test]
    fn test_internals() {
        let mut s = RingStack::<i32, 3>::new();
        assert_eq!(s.peek(), None);
        assert_eq!(s.index, 0);
        assert_eq!(s.buffer[0], None);
        assert_eq!(s.buffer[1], None);
        assert_eq!(s.buffer[2], None);

        s.push(1);
        assert_eq!(s.peek(), Some(&1));
        assert_eq!(s.index, 1);
        assert_eq!(s.buffer[0], None);
        assert_eq!(s.buffer[1], Some(1));
        assert_eq!(s.buffer[2], None);

        s.push(2);
        assert_eq!(s.peek(), Some(&2));
        assert_eq!(s.index, 2);
        assert_eq!(s.buffer[0], None);
        assert_eq!(s.buffer[1], Some(1));
        assert_eq!(s.buffer[2], Some(2));

        s.push(3);
        assert_eq!(s.peek(), Some(&3));
        assert_eq!(s.index, 0);
        assert_eq!(s.buffer[0], Some(3));
        assert_eq!(s.buffer[1], Some(1));
        assert_eq!(s.buffer[2], Some(2));

        s.push(4);
        assert_eq!(s.peek(), Some(&4));
        assert_eq!(s.index, 1);
        assert_eq!(s.buffer[0], Some(3));
        assert_eq!(s.buffer[1], Some(4));
        assert_eq!(s.buffer[2], Some(2));

        assert_eq!(s.peek(), Some(&4));
        assert_eq!(s.pop(), Some(4));
        assert_eq!(s.buffer[0], Some(3));
        assert_eq!(s.buffer[1], None);
        assert_eq!(s.buffer[2], Some(2));

        assert_eq!(s.pop(), Some(3));
        assert_eq!(s.buffer[0], None);
        assert_eq!(s.buffer[1], None);
        assert_eq!(s.buffer[2], Some(2));

        assert_eq!(s.pop(), Some(2));
        assert_eq!(s.buffer[0], None);
        assert_eq!(s.buffer[1], None);
        assert_eq!(s.buffer[2], None);

        assert_eq!(s.pop(), None);
        assert_eq!(s.buffer[0], None);
        assert_eq!(s.buffer[1], None);
        assert_eq!(s.buffer[2], None);
    }

    #[test]
    fn test_multiple_times_of_push_and_pop() {
        let mut s = RingStack::<i32, 3>::new();
        s.push(2);
        s.push(3);
        s.push(4);
        s.push(5);

        assert_eq!(s.peek(), Some(&5));

        assert_eq!(s.pop(), Some(5));
        assert_eq!(s.pop(), Some(4));
        assert_eq!(s.pop(), Some(3));
        assert_eq!(s.pop(), None);
    }

    #[test]
    fn test_iter() {
        let mut s = RingStack::<i32, 3>::new();

        s.push(6);
        s.push(7);
        let v: Vec<Option<i32>> = s.iter().map(|e| e.clone()).collect();
        assert_eq!(v.len(), 3);
        assert_eq!(v[0], Some(7));
        assert_eq!(v[1], Some(6));
        assert_eq!(v[2], None);

        s.push(8);
        let v: Vec<Option<i32>> = s.iter().map(|e| e.clone()).collect();
        assert_eq!(v.len(), 3);
        assert_eq!(v[0], Some(8));
        assert_eq!(v[1], Some(7));
        assert_eq!(v[2], Some(6));

        s.push(9);
        let v: Vec<Option<i32>> = s.iter().map(|e| e.clone()).collect();
        assert_eq!(v.len(), 3);
        assert_eq!(v[0], Some(9));
        assert_eq!(v[1], Some(8));
        assert_eq!(v[2], Some(7));

        s.pop();
        let v: Vec<Option<i32>> = s.iter().map(|e| e.clone()).collect();
        assert_eq!(v.len(), 3);
        assert_eq!(v[0], Some(8));
        assert_eq!(v[1], Some(7));
        assert_eq!(v[2], None);
    }
}
