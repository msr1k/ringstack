# {{crate}}

{{readme}}


## Changelog

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
