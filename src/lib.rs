//! # ringstack
//! 
//! [RingStack] is a tiny stack implementation which uses circular buffer.
//!
//! Since [RingStack] is constructed upon a circular buffer,
//! the oldest item automatically dropped as you [push][RingStack::push()]
//! when the number of items has already reached its limit.
//! (Thus [len][RingStack::len()] method saturate with that number of limit.)
//!
//! And it supports [RingStack::iter()] method which returns `Iterator<&T>`.
//! It provides items one by one with historical order, latest to oldest.
//!
//! Though [RingStack] currently uses [Vec] as its internals,
//! once it allocates at the timing of [new][RingStack::new()]
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
//! assert_eq!(s.len(), 2);
//! assert_eq!(s.peek(), Some(&2));
//! assert_eq!(s.pop(), Some(2));
//! assert_eq!(s[0], 1);
//! assert_eq!(s.get(0), Some(&1));
//! assert_eq!(s.get(1), None);
//!
//! s.push(3);
//! s.push(4);
//! let v: Vec<i32> = s.iter().map(|e| e.clone()).collect();
//! assert_eq!(v, vec![4, 3, 1]);
//!
//! s.push(5);
//! let v: Vec<i32> = s.iter().map(|e| e.clone()).collect();
//! assert_eq!(v, vec![5, 4, 3]);
//!
//! assert_eq!(s.pop(), Some(5));
//! assert_eq!(s.pop(), Some(4));
//! assert_eq!(s.pop(), Some(3));
//! assert_eq!(s.pop(), None);
//! ```
//! 
use std::iter::Iterator;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone)]
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

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= N {
            None
        } else {
            let index = (self.index + N - index) % N;
            self.buffer[index].as_ref()
        }
    }

    pub fn iter(&self) -> impl Iterator<Item=&T> {
        let (a, b) = self.buffer.split_at(self.index + 1);
        let a = a.iter().rev();
        let b = b.iter().rev();
        a.chain(b).map_while(|e| e.as_ref())
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item=&mut T> {
        let (a, b) = self.buffer.split_at_mut(self.index + 1);
        let a = a.iter_mut().rev();
        let b = b.iter_mut().rev();
        a.chain(b).map_while(|e| e.as_mut())
    }
}

impl<T, const N: usize> Index<usize> for RingStack<T,  N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.len {
            panic!("Specified index is out of bounds.")
        }
        let index = (self.index + N - index) % N;
        &self.buffer[index].as_ref().unwrap()
    }
}

impl<T, const N: usize> IndexMut<usize> for RingStack<T, N> {

    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.len {
            panic!("Specified index is out of bounds.")
        }
        let index = (self.index + N - index) % N;
        self.buffer[index].as_mut().unwrap()
    }
}

#[cfg(test)]
mod t {
    use super::*;

    #[test]
    fn test_internal_representaions_and_get_index() {
        let mut s = RingStack::<i32, 3>::new();
        assert_eq!(s.peek(), None);
        assert_eq!(s.index, 0);
        assert_eq!(s.buffer[0], None);
        assert_eq!(s.buffer[1], None);
        assert_eq!(s.buffer[2], None);

        assert_eq!(s.get(0), None);
        assert_eq!(s.get(1), None);
        assert_eq!(s.get(2), None);


        s.push(1);
        assert_eq!(s.peek(), Some(&1));
        assert_eq!(s.index, 1);
        assert_eq!(s.buffer[0], None);
        assert_eq!(s.buffer[1], Some(1));
        assert_eq!(s.buffer[2], None);

        assert_eq!(s.get(0), Some(&1));
        assert_eq!(s.get(1), None);
        assert_eq!(s.get(2), None);

        assert_eq!(s[0], 1i32);

        s.push(2);
        assert_eq!(s.peek(), Some(&2));
        assert_eq!(s.index, 2);
        assert_eq!(s.buffer[0], None);
        assert_eq!(s.buffer[1], Some(1));
        assert_eq!(s.buffer[2], Some(2));

        assert_eq!(s.get(0), Some(&2));
        assert_eq!(s.get(1), Some(&1));
        assert_eq!(s.get(2), None);

        assert_eq!(s[0], 2);
        assert_eq!(s[1], 1);

        s.push(3);
        assert_eq!(s.peek(), Some(&3));
        assert_eq!(s.index, 0);
        assert_eq!(s.buffer[0], Some(3));
        assert_eq!(s.buffer[1], Some(1));
        assert_eq!(s.buffer[2], Some(2));

        assert_eq!(s.get(0), Some(&3));
        assert_eq!(s.get(1), Some(&2));
        assert_eq!(s.get(2), Some(&1));
        assert_eq!(s.get(3), None);

        assert_eq!(s[0], 3);
        assert_eq!(s[1], 2);
        assert_eq!(s[2], 1);

        s.push(4);
        assert_eq!(s.peek(), Some(&4));
        assert_eq!(s.index, 1);
        assert_eq!(s.buffer[0], Some(3));
        assert_eq!(s.buffer[1], Some(4));
        assert_eq!(s.buffer[2], Some(2));

        assert_eq!(s.get(0), Some(&4));
        assert_eq!(s.get(1), Some(&3));
        assert_eq!(s.get(2), Some(&2));
        assert_eq!(s.get(3), None);

        assert_eq!(s[0], 4);
        assert_eq!(s[1], 3);
        assert_eq!(s[2], 2);

        assert_eq!(s.peek(), Some(&4));
        assert_eq!(s.pop(), Some(4));
        assert_eq!(s.buffer[0], Some(3));
        assert_eq!(s.buffer[1], None);
        assert_eq!(s.buffer[2], Some(2));

        assert_eq!(s.get(0), Some(&3));
        assert_eq!(s.get(1), Some(&2));
        assert_eq!(s.get(2), None);

        assert_eq!(s[0], 3);
        assert_eq!(s[1], 2);

        assert_eq!(s.pop(), Some(3));
        assert_eq!(s.buffer[0], None);
        assert_eq!(s.buffer[1], None);
        assert_eq!(s.buffer[2], Some(2));

        assert_eq!(s.get(0), Some(&2));
        assert_eq!(s.get(1), None);

        assert_eq!(s[0], 2);


        assert_eq!(s.pop(), Some(2));
        assert_eq!(s.buffer[0], None);
        assert_eq!(s.buffer[1], None);
        assert_eq!(s.buffer[2], None);

        assert_eq!(s.get(0), None);


        assert_eq!(s.pop(), None);
        assert_eq!(s.buffer[0], None);
        assert_eq!(s.buffer[1], None);
        assert_eq!(s.buffer[2], None);

        assert_eq!(s.get(0), None);

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
        assert_eq!(s.len(), 2);
        let v: Vec<&i32> = s.iter().collect();
        assert_eq!(v.len(), 2);
        assert_eq!(v[0], &7);
        assert_eq!(v[1], &6);

        s.push(8);
        assert_eq!(s.len(), 3);
        let v: Vec<&i32> = s.iter().collect();
        assert_eq!(v.len(), 3);
        assert_eq!(v[0], &8);
        assert_eq!(v[1], &7);
        assert_eq!(v[2], &6);

        s.push(9);
        assert_eq!(s.len(), 3);
        let v: Vec<&i32> = s.iter().collect();
        assert_eq!(v.len(), 3);
        assert_eq!(v[0], &9);
        assert_eq!(v[1], &8);
        assert_eq!(v[2], &7);

        s.pop();
        assert_eq!(s.len(), 2);
        let v: Vec<&i32> = s.iter().collect();
        assert_eq!(v.len(), 2);
        assert_eq!(v[0], &8);
        assert_eq!(v[1], &7);
    }

    #[test]
    fn test_iter_mut() {

        #[derive(Debug, PartialEq)]
        struct I32(i32); 

        let mut s = RingStack::<I32, 3>::new();

        s.push(I32(6));
        s.push(I32(7));
        assert_eq!(s.len(), 2);
        let mut v: Vec<&mut I32> = s.iter_mut().collect();
        assert_eq!(v.len(), 2);
        assert_eq!(v[0], &I32(7));
        assert_eq!(v[1], &I32(6));

        v[0].0 = 1;
        v[1].0 = 2;

        assert_eq!(v[0], &I32(1));
        assert_eq!(v[1], &I32(2));
    }

    #[test]
    fn test_clone() {
        let mut s = RingStack::<i32, 3>::new();

        s.push(6);
        s.push(7);

        let t = s.clone();

        assert_eq!(s[0], t[0]);
        assert_eq!(s[1], t[1]);
    }

    #[test]
    #[should_panic]
    fn test_index_access() {
        let mut s = RingStack::<i32, 3>::new();

        s.push(100);
        assert_eq!(s[0], 100);
        let _out_of_bounds_access = s[1];
    }

    #[test]
    #[should_panic]
    fn test_index_mut_access() {
        let mut s = RingStack::<i32, 3>::new();

        s.push(100);
        assert_eq!(s[0], 100);

        s[0] = 200;
        assert_eq!(s[0], 200);

        let _out_of_bounds_access = s[1];
    }

    #[test]
    #[should_panic(expected = "Specified index is out of bounds")]
    fn test_index_access_check() {
        let s = RingStack::<i32, 3>::new();
        let _out_of_bounds_access = s[3];
    }
}
