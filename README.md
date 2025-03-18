# ringstack

## ringstack

[RingStack] is a tiny stack implementation which uses circular buffer.

Since [RingStack] is constructed upon a circular buffer,
the oldest item automatically dropped as you [push][RingStack::push()]
when the number of items has already reached its limit.
(Thus [len][RingStack::len()] method saturate with that number of limit.)

And it supports [RingStack::iter()] method which returns `Iterator<&T>`.
It provides items one by one with historical order, latest to oldest.
([RingStack::iter_mut()] method is also available)

Though [RingStack] currently uses [Vec] as its internals,
once it allocates at the timing of [new][RingStack::new()]
then additional allocation never happends.

### Examples

```rust
use ringstack::RingStack;

let mut s = RingStack::<i32, 3>::new();
assert_eq!(s.peek(), None);

s.push(1);
s.push(2);
assert_eq!(s.len(), 2);
assert_eq!(s.peek(), Some(&2));
assert_eq!(s.pop(), Some(2));
assert_eq!(s[0], 1);
assert_eq!(s.get(0), Some(&1));
assert_eq!(s.get(1), None);

s.push(3);
s.push(4);
let v: Vec<i32> = s.iter().map(|e| e.clone()).collect();
assert_eq!(v, vec![4, 3, 1]);

s.push(5);
let v: Vec<i32> = s.iter().map(|e| e.clone()).collect();
assert_eq!(v, vec![5, 4, 3]);

assert_eq!(s.pop(), Some(5));
assert_eq!(s.pop(), Some(4));
assert_eq!(s.pop(), Some(3));
assert_eq!(s.pop(), None);
```



## Changelog

### 0.3.0 (2025/03/18)

- Added [`iter_mut()`] and mut version of index accessing.
- Added [`clone()`]

### 0.2.0

- Added [`len()`], [`get()`] methods.
- Implemented [`std::ops::Index`].
- Change [`iter()`] return type

  Changed from `&Option<T>` into `&T` and it iterates only valid elements,
  since it returns reference of `T` not `Option`.

### 0.1.1

Make RingStack [Debug] derived

### 0.1.0

Initial Version


## License

The MIT License (MIT)

Copyright (c) 2022 msr1k
