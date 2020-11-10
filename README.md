# thread-priority

[![Build status](https://travis-ci.org/vityafx/thread-priority.svg?branch=master)](https://travis-ci.org/vityafx/thread-priority)
[![Crates](https://img.shields.io/crates/v/thread-priority.svg)](https://crates.io/crates/thread-priority)
[![Docs](https://docs.rs/thread-priority/badge.svg)](https://docs.rs/thread-priority)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

A simple library to control thread schedule policies and thread priority.

This crate does not support all the plaforms yet but it is inteded to be developed so,
so feel free to contribute!

## Supported platforms

- Linux
- Windows

## Example

Setting current thread's priority to minimum:

```rust,no_run
use thread_priority::*;

fn main() {
    assert!(set_current_thread_priority(ThreadPriority::Min).is_ok());
}
```

## License

This project is [licensed under the MIT license](https://github.com/vityafx/thread-priority/blob/master/LICENSE).
