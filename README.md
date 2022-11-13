# ringstack

## ringstack

[RingStack] is a tiny stack implementation which uses circular buffer.

Since [RingStack] is constructed upon a circular buffer,
the oldest item automatically dropped as you [push][RingStack::push()]
when the number of items has already reached its limit.

And it supports [RingStack::iter()] method which returns `Iterator<&Option<T>>`.
It provides items one by one with historical order, latest to oldest.

Though [RingStack] currently uses [Vec] as its internals,
once it allocates at the timing of [new][RingStack::iter()]
then additional allocation never happends.

### Examples

```rust
use ringstack::RingStack;

let mut s = RingStack::<i32, 3>::new();
assert_eq!(s.peek(), None);

s.push(1);
s.push(2);
assert_eq!(s.peek(), Some(&2));
assert_eq!(s.pop(), Some(2));

s.push(3);
s.push(4);
let v: Vec<Option<i32>> = s.iter().map(|e| e.clone()).collect();
assert_eq!(v, vec![Some(4), Some(3), Some(1)]);

s.push(5);
let v: Vec<Option<i32>> = s.iter().map(|e| e.clone()).collect();
assert_eq!(v, vec![Some(5), Some(4), Some(3)]);

assert_eq!(s.pop(), Some(5));
assert_eq!(s.pop(), Some(4));
assert_eq!(s.pop(), Some(3));
assert_eq!(s.pop(), None);
```



## Changelog

### 0.1.1

Make RingStack [Debug] derived

### 0.1.0

Initial Version


## License

The MIT License (MIT)

Copyright (c) 2022 msr1k
